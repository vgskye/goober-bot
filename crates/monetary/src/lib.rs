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
use poise::{
    CreateReply,
    serenity_prelude::{
        CreateActionRow, CreateButton, CreateEmbed, GuildId, RoleId, SkuId, colours::css::WARNING,
    },
};
use poise_error::anyhow;

const EARLY_ACCESS_SKU_ID: u64 = 1351234259867926671;

/// Returns `Ok(true)` or sends a reply and returns `Ok(false)`.
///
/// May return `Err(_)` if it fails to send a reply.
#[allow(unused)]
pub async fn has_early_access(ctx: shared::Context<'_>) -> Result<bool, anyhow::Error> {
    Ok(true)
    // let author_id = ctx.author().id;
    // let goober_bot_dev_guild = GuildId::new(1250948547403055114);
    // let og_early_access_role = RoleId::new(1337229578472652846);

    // if goober_bot_dev_guild
    //     .member(ctx, author_id)
    //     .await
    //     .is_ok_and(|member| member.roles.contains(&og_early_access_role))
    // {
    //     return Ok(true);
    // }

    // let entitlements = ctx
    //     .http()
    //     .get_entitlements(
    //         Some(author_id),
    //         Some(vec![SkuId::new(EARLY_ACCESS_SKU_ID)]),
    //         None,
    //         None,
    //         None,
    //         None,
    //         Some(true),
    //     )
    //     .await?;

    // if entitlements.is_empty() {
    //     ctx.send(
    //         CreateReply::default()
    //             .embed(
    //                 CreateEmbed::new()
    //                     .title("Early Access command")
    //                     .description(format!(
    //                         "Sorry to stop you, but this command is exclusive to \
    //                         **Early Access**! If you would be so kind as to \
    //                         support development, it'd really help a ton, and I'd \
    //                         reward you with a handful of extra commands that no \
    //                         one else can use! {FLOOF_HEART}",
    //                     ))
    //                     .color(WARNING),
    //             )
    //             .components(vec![CreateActionRow::Buttons(vec![
    //                 CreateButton::new_premium(EARLY_ACCESS_SKU_ID),
    //             ])])
    //             .ephemeral(true),
    //     )
    //     .await?;

    //     return Ok(false);
    // }

    // Ok(true)
}
