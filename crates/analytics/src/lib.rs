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

use std::collections::{HashMap, HashSet};

use charts_rs::{HorizontalBarChart, THEME_DARK};
use chrono::{DateTime, TimeDelta, Utc};
use database::read_or_write_default;
use poise::{CreateReply, command, serenity_prelude::CreateAttachment};
use shared::Context;

const KEY: &str = "analytics";

type Analytics = HashMap<String, Vec<DateTime<Utc>>>;

async fn load(ctx: Context<'_>) -> Result<Analytics, poise_error::anyhow::Error> {
    let mut analytics: Analytics = read_or_write_default(ctx, KEY).await?;
    let commands: HashSet<_> = ctx
        .framework()
        .options
        .commands
        .iter()
        .map(|command| command.identifying_name.clone())
        .collect();

    // Ensure all commands are in analytics
    for command in &commands {
        analytics.entry(command.clone()).or_default();
    }

    // Remove commands from analytics that no longer exist
    analytics.retain(|command, _| commands.contains(command));

    // Remove command invocations which were more than a day ago
    for invocations in analytics.values_mut() {
        invocations
            .retain(|date_time| Utc::now().signed_duration_since(date_time) <= TimeDelta::days(1));
    }

    ctx.data().write_serialized(KEY, &analytics)?;

    Ok(analytics)
}

pub async fn increment(ctx: Context<'_>) -> Result<(), poise_error::anyhow::Error> {
    let mut analytics = load(ctx).await?;
    let root_command = ctx
        .parent_commands()
        .first()
        .map_or(ctx.command().identifying_name.clone(), |root_command| {
            root_command.identifying_name.clone()
        });
    let invocations = analytics.entry(root_command).or_default();

    invocations.push(Utc::now());
    ctx.data().write_serialized(KEY, &analytics)?;

    Ok(())
}

/// Displays the the usage of commands in the last 24 hours
#[command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel",
    owners_only,
    ephemeral
)]
pub async fn analytics(ctx: Context<'_>) -> Result<(), poise_error::anyhow::Error> {
    ctx.defer_ephemeral().await?;

    let mut analytics: Vec<(_, _)> = load(ctx).await?.into_iter().collect();

    analytics.sort_by(|(_, invocations_a), (_, invocations_b)| {
        invocations_b.len().cmp(&invocations_a.len())
    });

    let mut series_data = Vec::new();
    let mut x_axis_data = Vec::new();

    for (command, invocations) in analytics {
        series_data.push(invocations.len() as f32);
        x_axis_data.push(format!("/{command}"));
    }

    let mut chart = HorizontalBarChart::new_with_theme(
        vec![("Invocations", series_data).into()],
        x_axis_data,
        THEME_DARK,
    );

    chart.width *= 1.5;
    chart.height *= 1.5;
    chart.margin = charts_rs::Box {
        left: 10.0,
        top: 5.0,
        right: 25.0,
        bottom: 10.0,
    };
    chart.title_text = "Command Invocations in the Last 24 Hours".to_string();
    chart.legend_margin = Some(charts_rs::Box {
        top: 25.0,
        ..Default::default()
    });
    ctx.send(CreateReply::default().attachment(CreateAttachment::bytes(
        charts_rs::svg_to_png(&chart.svg()?)?,
        "analytics.png",
    )))
    .await?;

    Ok(())
}
