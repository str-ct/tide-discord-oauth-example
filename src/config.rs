use {serde::Deserialize, std::path::PathBuf, structopt::StructOpt};

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(parse(from_os_str))]
    pub config: PathBuf,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub secret_key: String,
    pub discord_oauth: DiscordOAuthConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DiscordOAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
}
