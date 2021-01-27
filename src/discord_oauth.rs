use {
    crate::config::DiscordOAuthConfig,
    anyhow::Result,
    oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl},
};

// Use the OAuth2 URL Generator at https://discord.com/developers/applications/
static AUTH_URL: &str = "https://discord.com/api/oauth2/authorize?client_id=<client_id>&redirect_uri=http%3A%2F%2Flocalhost%3A8000%2Flogin%2Fauthorized%2F&response_type=code&scope=email";
static TOKEN_URL: &str = "https://discord.com/api/oauth2/token";

pub fn make_client(config: &DiscordOAuthConfig) -> Result<BasicClient> {
    Ok(BasicClient::new(
        ClientId::new(config.client_id.clone()),
        Some(ClientSecret::new(config.client_secret.clone())),
        AuthUrl::new(AUTH_URL.to_string())?,
        Some(TokenUrl::new(TOKEN_URL.to_string())?),
    )
    .set_redirect_url(RedirectUrl::new(config.redirect_url.clone())?))
}
