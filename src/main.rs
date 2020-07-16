use anyhow::{anyhow, Context, Result};
use rand::{seq::SliceRandom, thread_rng};
use tbot::{
    contexts::methods::ChatMethods,
    types::{
        chat::{member::Status, Permissions},
        dice::{Dice, Kind::Darts},
    },
    Bot,
};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut bot_loop = Bot::from_env("BOT_TOKEN").event_loop();

    bot_loop.dice(|context| async move {
        handle_dice(&context).await.unwrap_or_else(|e| {
            error!("Dice handled unsuccessfully: {}", e);
        });
    });

    bot_loop
        .polling()
        .start()
        .await
        .map_err(|e| anyhow!("Error setting up polling loop: {:?}", e))?;

    Ok(())
}

async fn handle_dice(ctx: &tbot::contexts::Dice) -> Result<()> {
    if let Dice {
        kind: Darts,
        value: 6,
        ..
    } = ctx.dice
    {
        let user = ctx.from.as_ref().context("Dice sent by nobody")?;

        info!(
            "{} won!",
            user.username.as_deref().unwrap_or("anonymous user")
        );

        if ctx.chat.kind.is_supergroup() {
            let member = ctx.get_chat_member(user.id).call().await?;

            tokio::time::delay_for(std::time::Duration::from_secs(30)).await;

            match member.status {
                Status::Administrator { .. } | Status::Creator { .. } => {
                    ctx.send_message_in_reply("Ich kann Dich zwar nicht muten, aber sei doch bitte so lieb und halt trotzdem eine Woche lang die Fresse.").call().await?;
                }
                Status::Member | Status::Restricted { .. } => {
                    let msg = *[
                        "Jawollo!",
                        "Gewinner, Gewinner, Huhn Abendessen!",
                        "Viel Spaß bei einer Woche Urlaub von RWTH Informatik!",
                        "endlich",
                    ]
                    .choose(&mut thread_rng())
                    .unwrap();

                    ctx.send_message_in_reply(msg).call().await?;

                    let permissions = Permissions::new().can_send_messages(false);
                    ctx.restrict_chat_member(user.id, permissions)
                        .until_date(ctx.date.saturating_add(7 * 24 * 60 * 60))
                        .call()
                        .await?;
                }
                Status::Left { .. } => {
                    ctx.send_message_in_reply("Schade dass Du schon weg bist. Ich werde Deinen Gewinn aufbewahren und einlösen, wenn Du uns wieder besuchst!").call().await?;
                }
                _ => {}
            }
        } else {
            tokio::time::delay_for(std::time::Duration::from_secs(3)).await;
            ctx.send_message_in_reply(
                "Guter Wurf! Aber jetzt genug geübt, probier dein Glück in der Hauptgruppe!",
            )
            .call()
            .await?;
        }
    };

    Ok(())
}
