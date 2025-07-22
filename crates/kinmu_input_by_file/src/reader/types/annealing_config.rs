//! annealing_configを読み込むための構造体

#[derive(Debug, PartialEq, Clone, serde::Deserialize)]
pub struct RawAnnealingConfig {
    pub step_count: u32,
    pub seed: Option<u64>,
    pub score_functions: Vec<RawAnnealingScoreFunction>,
    pub update_function: String,
    pub temperature: RawTemperatureKey,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize)]
pub struct RawAnnealingScoreFunction {
    pub scores: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize)]
pub struct RawTemperatureKey {
    pub begin: f32,
    pub end: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_read() {
        let config: RawAnnealingConfig = toml::from_str(
            r#"
            step_count = 1000
            seed = 25565

            # you have to ignore comments
            # 日本語のコメントも無視する
            score_functions = [
            {scores = [
                "Score1",
                "Score2",
            ]},
            {scores = [
                "Score1",
                "Score3",
            ], unused_field = 10},
            ]

            update_function = "swap"

            temperature = {begin = 1000, end = 50}
            "#,
        )
        .unwrap();

        assert_eq!(config.step_count, 1000);
        assert_eq!(config.seed, Some(25565));
        assert_eq!(
            config.score_functions,
            vec![
                RawAnnealingScoreFunction {
                    scores: vec![String::from("Score1"), String::from("Score2")]
                },
                RawAnnealingScoreFunction {
                    scores: vec![String::from("Score1"), String::from("Score3")]
                },
            ]
        );
        assert_eq!(config.update_function, "swap");
        assert_eq!(
            config.temperature,
            RawTemperatureKey {
                begin: 1000.0,
                end: 50.0,
            }
        );
    }

    #[test]
    fn test_minimal_read() {
        let config: RawAnnealingConfig = toml::from_str(
            r#"
            step_count = 100
            score_functions = []
            update_function = ""
            temperature = {begin = 10, end = 0}
            "#,
        )
        .unwrap();

        assert_eq!(config.step_count, 100);
        assert_eq!(config.seed, None);
        assert_eq!(
            config.score_functions,
            <Vec<RawAnnealingScoreFunction>>::new()
        );
        assert_eq!(config.update_function, "");
        assert_eq!(
            config.temperature,
            RawTemperatureKey {
                begin: 10.0,
                end: 0.0
            }
        );
    }
}
