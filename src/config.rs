use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Config {
    pub bind_addr: SocketAddr,
    pub log_level: String,
    pub environment: String,
    /// Base URL of the srvcs-pi dependency.
    pub pi_url: String,
    /// Base URL of the srvcs-floatmultiply dependency.
    pub floatmultiply_url: String,
}

impl Config {
    pub fn from_vars(
        bind: Option<String>,
        log: Option<String>,
        env: Option<String>,
        pi_url: Option<String>,
        floatmultiply_url: Option<String>,
    ) -> Self {
        let bind_addr = bind
            .unwrap_or_else(|| "0.0.0.0:8080".to_string())
            .parse()
            .expect("SRVCS_BIND_ADDR must be host:port");
        Config {
            bind_addr,
            log_level: log.unwrap_or_else(|| "info,tower_http=info".to_string()),
            environment: env.unwrap_or_else(|| "development".to_string()),
            pi_url: pi_url.unwrap_or_else(|| "http://127.0.0.1:8090".to_string()),
            floatmultiply_url: floatmultiply_url
                .unwrap_or_else(|| "http://127.0.0.1:8091".to_string()),
        }
    }

    pub fn from_env() -> Self {
        Self::from_vars(
            std::env::var("SRVCS_BIND_ADDR").ok(),
            std::env::var("RUST_LOG").ok(),
            std::env::var("SRVCS_ENV").ok(),
            std::env::var("SRVCS_PI_URL").ok(),
            std::env::var("SRVCS_FLOATMULTIPLY_URL").ok(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_sane() {
        let c = Config::from_vars(None, None, None, None, None);
        assert_eq!(c.bind_addr.port(), 8080);
        assert_eq!(c.pi_url, "http://127.0.0.1:8090");
        assert_eq!(c.floatmultiply_url, "http://127.0.0.1:8091");
    }

    #[test]
    fn parses_explicit_bind_addr() {
        let c = Config::from_vars(Some("127.0.0.1:9000".into()), None, None, None, None);
        assert_eq!(c.bind_addr.port(), 9000);
    }
}
