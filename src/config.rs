#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Config {
    github_token: String,
}
