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

use serde::{Serialize, de::DeserializeOwned};
use shared::Context;

/// Reads a value from [`shuttle_shared_db::SerdeJsonOperator`] (if the value
/// exists), or writes the default of the value (if the value does not exist).
pub async fn read_or_write_default<T>(ctx: Context<'_>, key: &str) -> Result<T, poise_error::anyhow::Error>
where
    T: DeserializeOwned + Serialize + Default,
{
    let data = ctx.data();

    if let Err(e) = data.db.compare_and_swap(key, None::<[u8; 0]>, Some(postcard::to_stdvec(&T::default())?))? {
        Ok(postcard::from_bytes(&e.current.unwrap())?)
    } else {
        Ok(T::default())
    }
}
