//! Where Atelier Scribe and the Nation live on the web (About, the title-bar Nation button, the
//! Help ribbon, the MCP server). The command id keeps its upstream name: `ui.discord` opens the
//! Nation.

/// The Aetherion Nation.
pub const NATION: &str = "https://www.aetherionnnation.com";
/// Atelier, the Nation's creative suite.
pub const SUITE: &str = "https://www.aetherionnnation.com/atelier";
/// Atelier Scribe's page on the Nation site.
pub const APP_PAGE: &str = "https://www.aetherionnnation.com/atelier/scribe";
/// Atelier Scribe's source repository.
pub const GITHUB: &str = "https://github.com/Aetherion-Nation-Builders/atelier-scribe";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_follow_the_atelier_scheme() {
        assert!(SUITE.starts_with(NATION) && APP_PAGE.starts_with(SUITE));
        assert!(APP_PAGE.ends_with("/scribe") && GITHUB.ends_with("/atelier-scribe"));
    }
}
