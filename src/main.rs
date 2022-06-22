use tokio;
use twitch_api2::{helix::channels::GetChannelInformationRequest, TwitchClient};
use twitch_oauth2::{tokens::errors::AppAccessTokenError, AppAccessToken, Scope, TwitchToken, ClientId, ClientSecret};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client: TwitchClient<reqwest::Client> = TwitchClient::default();
    let token = AppAccessToken::get_app_access_token(
        client.get_client(), 
        std::env::var("TWITCH_CLIENT_ID")
            .map(ClientId::new)
            .expect("Please set the TWITCH_CLIENT_ID env var."),
        std::env::var("TWITCH_CLIENT_SECRET")
            .map(ClientSecret::new)
            .expect("Please set the TWITCH_CLIENT_SECRET env var."),
        vec![Scope::ChannelModerate]
    ).await?;

    println!("{:?}", &client.helix.get_channel_from_id("723826498", &token).await?.unwrap());

    Ok(())
}
