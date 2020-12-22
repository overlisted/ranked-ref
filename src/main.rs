use std::sync::Arc;
use tokio::sync::RwLockWriteGuard;

use dashmap::mapref::one::{Ref, RefMut};
use dashmap::DashMap;
use serenity::client::{Context, EventHandler};
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::framework::StandardFramework;
use serenity::model::channel::Message;
use serenity::model::id::GuildId;
use serenity::{async_trait, Client};

use crate::system::Leaderboard;
use serenity::prelude::TypeMap;

mod system;
mod view;

#[group]
#[commands(score, register, leaderboard)]
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

fn into_leaderboards(
    mut write: RwLockWriteGuard<'_, TypeMap>,
) -> Arc<DashMap<GuildId, Leaderboard>> {
    write
        .get_mut::<Leaderboard>()
        .expect("Leaderboards aren't initialized!")
        .clone()
}

async fn get_mut_leaderboard(
    lbs: &Arc<DashMap<GuildId, Leaderboard>>,
    guild: Option<GuildId>,
) -> Result<RefMut<'_, GuildId, Leaderboard>, ()> {
    lbs.get_mut(&guild.ok_or(())?).ok_or(())
}

async fn get_leaderboard(
    lbs: &Arc<DashMap<GuildId, Leaderboard>>,
    guild: Option<GuildId>,
) -> Result<Ref<'_, GuildId, Leaderboard>, ()> {
    lbs.get(&guild.ok_or(())?).ok_or(())
}

#[command]
async fn score(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let winner = args.single_quoted::<String>()?;
    let loser = args.single_quoted::<String>()?;

    let lbs = &into_leaderboards(ctx.data.write().await);
    if let Ok(mut lb) = get_mut_leaderboard(lbs, msg.guild_id).await {
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
    } else {
        msg.reply(ctx, "This server doesn't have a leaderboard!")
            .await?;
    }

    Ok(())
}

#[command]
async fn register(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let player = args.single_quoted::<String>()?;

    let lbs = &into_leaderboards(ctx.data.write().await);
    if let Ok(mut lb) = get_mut_leaderboard(lbs, msg.guild_id).await {
        if lb.find_player(player.clone()) == None {
            let player = lb.insert_player(player);
            msg.reply(ctx, format!("**Ok**!\n{}", lb.format_player(&player)))
                .await?;
        } else {
            msg.reply(ctx, "The player is already registered!").await?;
        }
    } else {
        msg.reply(ctx, "This server doesn't have a leaderboard!")
            .await?;
    }

    Ok(())
}

#[command]
async fn leaderboard(ctx: &Context, msg: &Message) -> CommandResult {
    let lbs = &into_leaderboards(ctx.data.write().await);
    if let Ok(lb) = get_leaderboard(lbs, msg.guild_id).await {
        msg.reply(ctx, lb.format()).await?;
    } else {
        msg.reply(ctx, "This server doesn't have a leaderboard!")
            .await?;
    }

    Ok(())
}
