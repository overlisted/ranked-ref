use crate::system::Leaderboard;
use dashmap::DashMap;
use serenity::client::{Context, EventHandler};
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::framework::StandardFramework;
use serenity::model::channel::Message;
use serenity::model::id::GuildId;
use serenity::{async_trait, Client};
use std::sync::Arc;

mod system;
mod view;

#[group]
#[commands(score)]
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

#[command]
async fn score(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let winner = args.single_quoted::<String>()?;
    let loser = args.single_quoted::<String>()?;

    let mut data = ctx.data.write().await;
    let lbs = data
        .get_mut::<Leaderboard>()
        .expect("There's no leaderboard storage!");
    let mut lb = match lbs.get_mut(&msg.guild_id.expect("Guild ID is None!")) {
        Some(lbs) => lbs,
        None => {
            msg.reply(ctx, "This guild doesn't have a leaderboard!")
                .await?;
            return Ok(());
        }
    };

    let (winner, loser) = lb.score(winner, loser);
    msg.reply(
        ctx,
        format!(
            "**Ok**!\n{}\n{}",
            lb.format_player(&winner),
            lb.format_player(&loser)
        ),
    )
    .await?;

    Ok(())
}
