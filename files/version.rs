//! Static build info for Milanote Desktop.

pub const BUILD_VERSION: &str  = "v4.6.9";
pub const BUILD_CHANNEL: &str  = "lts";
pub const BUILD_COMMIT: &str   = "eb0681331135";
pub const BUILD_TIMESTAMP: u64 = 1774391797;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Milanote Desktop", BUILD_VERSION, BUILD_CHANNEL)
}
