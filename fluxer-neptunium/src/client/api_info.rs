use zeroize::Zeroizing;

#[derive(Debug)]
pub struct ApiInfo {
    pub token: Zeroizing<String>,
    pub base_path: String,
    pub client: reqwest::Client,
    pub user_agent: String,
}
