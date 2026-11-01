use std::sync::OnceLock;

pub const REQUEST_TIMEOUT_SECS: u64 = 25;
pub const MIN_REQUEST_GAP_MS: u64 = 250;
pub const REQUEST_JITTER_MS: u64 = 250;

// Set RAIDLORDS_API_URL, RAIDLORDS_RPC_URL, RAIDLORDS_AUTH_URL,
// RAIDLORDS_VERSION_URL and RAIDLORDS_SIM_URL in the process environment
// when the game deployment changes its public endpoints.
static API: OnceLock<String> = OnceLock::new();
static RPC: OnceLock<String> = OnceLock::new();
static AUTH: OnceLock<String> = OnceLock::new();
static VERSION: OnceLock<String> = OnceLock::new();
static SIM: OnceLock<String> = OnceLock::new();

pub fn api_url() -> &'static str {
    API.get_or_init(|| std::env::var("RAIDLORDS_API_URL")
        .unwrap_or_else(|_| "https://raidlords.com".to_string()))
}
pub fn rpc_url() -> &'static str {
    RPC.get_or_init(|| std::env::var("RAIDLORDS_RPC_URL")
        .unwrap_or_else(|_| format!("{}/rest/v1/rpc", api_url().trim_end_matches('/'))))
}
pub fn auth_url() -> &'static str {
    AUTH.get_or_init(|| std::env::var("RAIDLORDS_AUTH_URL")
        .unwrap_or_else(|_| format!("{}/auth/v1/token", api_url().trim_end_matches('/'))))
}
pub fn version_url() -> &'static str {
    VERSION.get_or_init(|| std::env::var("RAIDLORDS_VERSION_URL")
        .unwrap_or_else(|_| format!("{}/version", api_url().trim_end_matches('/'))))
}
pub fn sim_url() -> &'static str {
    SIM.get_or_init(|| std::env::var("RAIDLORDS_SIM_URL")
        .unwrap_or_else(|_| format!("{}/assets/sim/", api_url().trim_end_matches('/'))))
}

// Compatibility names used by the cycle coordinator.
pub fn version_url_static() -> &'static str { version_url() }
pub fn sim_url_static() -> &'static str { sim_url() }
