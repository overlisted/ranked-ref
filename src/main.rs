use crate::system::Leaderboard;
use dashmap::DashMap;
use serenity::client::{Context, EventHandler};
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::framework::StandardFramework;
use serenity::model::channel::Message;
use serenity::model::id::GuildId;
use serenity::{async_trait, Client};

mod system;

#[group]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fm = StandardFramework::new()
        .configure(|config| config.prefix("/"))
        .group(&GENERAL_GROUP);

    let token = std::env::var("RANKED_REF_TOKEN")?;

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(fm)
        .await?;

    let map = DashMap::new();
    map.insert(GuildId::from(786739598389805057), Leaderboard::new());

    {
        let mut data = client.data.write().await;

        data.insert::<Leaderboard>(Arc::new(map))
    }

    client.start().await?;

    Ok(())
}
