// Goober Bot, Discord bot
// Copyright (C) 2025  Valentine Briese
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.


use emoji::*;
use poise::command;
use poise_error::anyhow::Context as _;
use shared::Context;

/// Vote for Goober Bot on Top.gg!
#[command(
    slash_command,
    category = "Other",
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel",
    ephemeral
)]
pub async fn vote(ctx: Context<'_>) -> Result<(), poise_error::anyhow::Error> {
    let has_voted = ctx
        .data()
        .topgg_client
        .has_voted(ctx.author().id)
        .await
        .context("could not check if user has voted")
        .context("top.gg dun goofed")?;
    let message = if has_voted {
        format!("You've already voted today, thank you so much! ily {FLOOF_HEART}")
    } else {
        format!(
            "You're able to vote for <@{bot_id}> on Top.gg today still! You can [do so here](https://top.gg/bot/{bot_id}/vote). Thank you for your consideration! {FLOOF_HAPPY}",
            bot_id = ctx.framework().bot_id,
        )
    };

    ctx.say(message).await?;

    Ok(())
}
