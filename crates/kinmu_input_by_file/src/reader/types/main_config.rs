//! main_configを読み込むための構造体

#[derive(Debug, PartialEq, Clone, serde::Deserialize)]
pub struct RawMainConfig {
    pub schedule_config_paths: Vec<String>,
    pub thread_count: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_read() {
        let config: RawMainConfig = toml::from_str(
            r#"
            # this is a comment
            schedule_config_paths = [
                "some_path.toml"
            ]

            thread_count = 1 # you have to ignore me
            "#,
        )
        .unwrap();

        assert_eq!(
            config.schedule_config_paths,
            vec![String::from("some_path.toml")]
        );
        assert_eq!(config.thread_count, Some(1));
    }

    #[test]
    fn test_minimal_read() {
        let config: RawMainConfig = toml::from_str(
            r#"
            schedule_config_paths = []
            "#,
        )
        .unwrap();

        assert_eq!(config.schedule_config_paths, <Vec<String>>::new());
        assert_eq!(config.thread_count, None);
    }
}
