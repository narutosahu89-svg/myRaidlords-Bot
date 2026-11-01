use std::sync::{Mutex, MutexGuard};

use reqwest::blocking::{Client, ClientBuilder};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT};
use serde_json::{json, Value};

use crate::net::{pace::Pace, proxy::ProxySpec};
use crate::support::consts::net::{self, MIN_REQUEST_GAP_MS, REQUEST_JITTER_MS, REQUEST_TIMEOUT_SECS};
use crate::support::error::{Error, Result};

pub struct Api {
    client: Client,
    token: Mutex<Option<String>>,
    pace: Pace,
    seed: String,
}

impl Api {
    pub fn new(proxy: Option<&ProxySpec>, seed: &str) -> Result<Api> {
        let device = crate::net::device::for_account(seed);
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        if let Ok(value) = HeaderValue::from_str(&device.user_agent) {
            headers.insert(USER_AGENT, value);
        }
        if let Ok(value) = HeaderValue::from_str(&device.accept_language) {
            headers.insert("accept-language", value);
        }
        if let Ok(value) = HeaderValue::from_str(&device.sec_ch_ua) {
            headers.insert("sec-ch-ua", value);
        }
        if let Ok(value) = HeaderValue::from_str(&device.sec_ch_ua_mobile) {
            headers.insert("sec-ch-ua-mobile", value);
        }
        if let Ok(value) = HeaderValue::from_str(&device.sec_ch_ua_platform) {
            headers.insert("sec-ch-ua-platform", value);
        }

        let mut builder: ClientBuilder = Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .https_only(true);
        if let Some(spec) = proxy {
            builder = spec.apply(builder);
        }
        let client = builder.build()?;
        Ok(Api {
            client,
            token: Mutex::new(None),
            pace: Pace::new(MIN_REQUEST_GAP_MS, REQUEST_JITTER_MS),
            seed: seed.to_string(),
        })
    }

    fn token_guard(&self) -> MutexGuard<'_, Option<String>> {
        self.token.lock().unwrap_or_else(|p| p.into_inner())
    }

    pub fn set_token(&self, token: Option<String>) {
        *self.token_guard() = token;
    }

    pub fn wallet_auth(&self, body: Value) -> Result<Value> {
        self.request_json(net::auth_url(), &body, false)
    }

    pub fn rpc(&self, method: &str, body: Value) -> Result<Value> {
        if method.is_empty() || !method.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(Error::Cfg("invalid RPC method name".to_string()));
        }
        let url = format!("{}/{}", net::rpc_url().trim_end_matches('/'), method);
        self.request_json(&url, &body, true)
    }

    pub fn battle_finish(&self, body: Value) -> Result<Value> {
        self.rpc("api_battle_finish", body)
    }

    pub fn fetch(&self, url: &str) -> Result<Vec<u8>> {
        self.pace.wait();
        let response = self.client.get(url).send()?;
        let status = response.status();
        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(Error::Denied);
        }
        if !status.is_success() {
            let text = response.text().unwrap_or_default();
            if crate::net::edge::is_banned(&text) {
                return Err(Error::Banned);
            }
            return Err(Error::Api { status: status.as_u16(), message: crate::support::log::sanitize(&text) });
        }
        Ok(response.bytes()?.to_vec())
    }

    fn request_json(&self, url: &str, body: &Value, authenticated: bool) -> Result<Value> {
        self.pace.wait();
        let mut request = self.client.post(url).json(body);
        if authenticated {
            if let Some(token) = self.token_guard().as_ref() {
                request = request.bearer_auth(token);
            }
        }
        if !self.seed.is_empty() {
            request = request.header("x-client-wallet", &self.seed);
        }
        let response = request.send()?;
        let status = response.status();
        let text = response.text()?;

        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(Error::Denied);
        }
        if !status.is_success() {
            if crate::net::edge::is_banned(&text) {
                return Err(Error::Banned);
            }
            let message = Value::from(text.clone())
                .as_str().unwrap_or("")
                .to_string();
            return Err(Error::Api { status: status.as_u16(), message: crate::support::log::sanitize(&message) });
        }

        let value: Value = serde_json::from_str(&text)
            .map_err(|e| Error::State(format!("server reply was not JSON: {}", e)))?;
        if let Some(error) = value.get("error") {
            if !error.is_null() {
                let message = error.as_str().map(str::to_string).unwrap_or_else(|| error.to_string());
                if crate::net::edge::is_banned(&message) { return Err(Error::Banned); }
                return Err(Error::Api { status: 400, message: crate::support::log::sanitize(&message) });
            }
        }
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rpc_names_are_restricted_to_identifier_characters() {
        assert!(Api::new(None, "").is_ok());
        assert!("api_state".chars().all(|c| c.is_ascii_alphanumeric() || c == '_'));
        assert!(!"api/state".chars().all(|c| c.is_ascii_alphanumeric() || c == '_'));
    }
}
