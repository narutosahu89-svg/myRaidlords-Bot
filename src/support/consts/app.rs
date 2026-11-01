pub const ENV_FILE: &str = ".env";
pub const CONFIG_FILE: &str = "config.json";
pub const PROXY_FILE: &str = "proxy.txt";
pub const SESSION_FILE: &str = "session.json";
pub const DEVICE_FILE: &str = "device.json";
pub const SIM_FILE: &str = "assets/sim.js";

pub const DEFAULT_SLEEP_SECS: u64 = 3600;
pub const MIN_SLEEP_SECS: u64 = 60;
pub const RECONNECT_SECS: u64 = 2;
pub const TRANSIENT_PAUSE_SECS: u64 = 3;
pub const CYCLE_JITTER_SECS: u64 = 90;
pub const CYCLE_STEPS_MAX: usize = 80;
pub const RECOVER_TRIES: u32 = 3;

// The live build and simulator digest are deliberately overridable at runtime.
// A game update should pause battle planning rather than run an unverified sim.
pub const GAME_BUILD: &str = "unknown";
pub const SIM_SHA256: &str = "";
