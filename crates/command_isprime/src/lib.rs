// Goober Bot, Discord bot
// Copyright (C) 2026 Skye Green
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
use rug::{Complete, Integer, integer::IsPrime};
use shared::Context;
use tokio::task::spawn_blocking;

/// Checks if a number is prime.
#[command(
    slash_command,
    category = "Other",
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel",
    required_bot_permissions = "USE_EXTERNAL_EMOJIS",
)]
pub async fn isprime(
    ctx: Context<'_>,
    #[description = "Number to check"]
    #[min_length = 0]
    #[max_length = 6000]
    number: String,
) -> Result<(), poise_error::anyhow::Error> {
    let Ok(number) = Integer::parse(number) else {
        ctx.say(format!("That doesn't look like a number... {FLOOF_WHAT}")).await?;
        return Ok(());
    };
    let number = number.complete();
    if number.is_negative() {
        ctx.say(format!("That's a negative number... {FLOOF_WHAT}")).await?;
        return Ok(());
    }
    if number == Integer::from(0) {
        ctx.say(format!("Zero is neither prime nor composite! {FLOOF}")).await?;
        return Ok(());
    }
    if number == Integer::from(1) {
        ctx.say(format!("One is neither prime nor composite! {FLOOF}")).await?;
        return Ok(());
    }
    ctx.defer().await?;
    match spawn_blocking(move || number.is_probably_prime(50)).await? {
        IsPrime::No => ctx.say(format!("That number... is composite! {FLOOF}")).await?,
        IsPrime::Probably => ctx.say(format!("I'm 99.99999999999998% sure this is a prime number! {FLOOF}")).await?,
        IsPrime::Yes => ctx.say(format!("That is a prime number! {FLOOF}")).await?,
    };
    Ok(())
}
