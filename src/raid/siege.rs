use sha2::{Digest, Sha256};
use serde_json::Value;

use crate::support::consts::app::SIM_FILE;
use crate::support::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct Sim {
    source: String,
    digest: String,
}

pub fn fingerprint(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

impl Sim {
    pub fn load() -> Result<Sim> {
        let bytes = std::fs::read(SIM_FILE)?;
        let source = String::from_utf8(bytes.clone())
            .map_err(|e| Error::Sim(format!("simulator source is not UTF-8: {}", e)))?;
        if source.trim().is_empty() {
            return Err(Error::Sim("simulator source is empty".to_string()));
        }
        Ok(Sim { source, digest: fingerprint(&bytes) })
    }

    pub fn digest(&self) -> &str { &self.digest }
    pub fn source(&self) -> &str { &self.source }

    // The engine is loaded only after the bundled game script has been verified.
    // Callers must compare the digest with the current game's published digest first.
    pub fn run(&self, input: &Value) -> Result<Value> {
        let mut context = boa_engine::Context::default();
        let bootstrap = format!(
            "{}\n;JSON.stringify((typeof simulateBattle === 'function' ? simulateBattle({}) : (typeof battle === 'function' ? battle({}) : null)))",
            self.source,
            serde_json::to_string(input).unwrap_or_else(|_| "null".to_string()),
            serde_json::to_string(input).unwrap_or_else(|_| "null".to_string())
        );
        let result = context.eval(boa_engine::Source::from_bytes(bootstrap.as_bytes()))
            .map_err(|e| Error::Sim(format!("battle simulation failed: {}", e)))?;
        let text = result.to_string(&mut context)
            .map_err(|e| Error::Sim(format!("battle simulation result failed: {}", e)))?;
        let text = text.to_std_string_escaped();
        serde_json::from_str(&text)
            .map_err(|e| Error::Sim(format!("battle simulator returned invalid JSON: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fingerprints_are_sha256_hex() {
        assert_eq!(fingerprint(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    }
}
