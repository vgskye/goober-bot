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

#![allow(clippy::suspicious_doc_comments)]

use paste::paste;

// Hey,
// https://cdn.discordapp.com/emojis/<id>.webp?size=48&quality=lossless
// ;)

// in debug builds, uses emojis from Goober Bot Dev app
// in release builds, uses emojis from Goober Bot app

/// Creates an emoji `&str` constant for debug builds and another for release
/// builds.
///
/// # Examples
///
/// ```
/// emoji!("emojiName", "1234567890987654321" /* debug */, "1234567890987654321" /* release */);
/// emoji!("animatedEmoji", "1234567890987654321" /* debug */, "1234567890987654321" /* release */, true /* gif */);
/// ```
macro_rules! emoji {
    ($name:literal, $debug_id:literal, $release_id:literal, $format:literal, $prefix:literal) => {
        paste! {
            #[allow(clippy::suspicious_doc_comments)]
            #[doc = concat!("![](https://cdn.discordapp.com/emojis/", $debug_id, ".", $format, "?quality=lossless)")]
            #[cfg(debug_assertions)]
            pub const [<$name:snake:upper>]: &str = concat!("<", $prefix, ":", $name, ":", $debug_id, ">");
            #[doc = concat!("![](https://cdn.discordapp.com/emojis/", $release_id, ".", $format, "?quality=lossless)")]
            #[cfg(not(debug_assertions))]
            pub const [<$name:snake:upper>]: &str = concat!("<", $prefix, ":", $name, ":", $release_id, ">");
        }
    };
    ($name:literal, $debug_id:literal, $release_id:literal$(, false)?) => {
        emoji!($name, $debug_id, $release_id, "webp", "");
    };
    ($name:literal, $debug_id:literal, $release_id:literal, true) => {
        emoji!($name, $debug_id, $release_id, "gif", "a");
    };
}

emoji!("floof", "1432623984276537404", "1432623984276537404");
emoji!("floofAngry", "1432623982078591046", "1432623982078591046");
emoji!("floofBlep", "1432623980035965078", "1432623980035965078");
emoji!("floofCat", "1432623978005921852", "1432623978005921852");
emoji!("floofCool", "1432623975518830652", "1432623975518830652");
emoji!("floofCry", "1432623973606359080", "1432623973606359080");
emoji!("floofDrool", "1432623971689435217", "1432623971689435217");
emoji!("floofHappy", "1432623969550204968", "1432623969550204968");
emoji!("floofHeart", "1432623965339259000", "1432623965339259000");
#[rustfmt::skip]
emoji!("floofInnocent", "1432623963086913607", "1432623963086913607");
emoji!("floofLoad", "1432623960591433758", "1432623960591433758");
#[rustfmt::skip]
emoji!("floofLoadAnimated", "1432623912293896262", "1432623912293896262", true);
emoji!("floofLol", "1432623957365751898", "1432623957365751898");
emoji!("floofLurk", "1432623954438131742", "1432623954438131742");
#[rustfmt::skip]
emoji!("floofMischief", "1432623952009760808", "1432623952009760808");
emoji!("floofMug", "1432623949023416390", "1432623949023416390");
emoji!("floofNervous", "1432623946121089095", "1432623946121089095");
emoji!("floofNom", "1432623943453245460", "1432623943453245460");
emoji!("floofOwo", "1432623941578522697", "1432623941578522697");
emoji!("floofPat", "1432623937899986964", "1432623937899986964");
emoji!("floofPeek", "1432623935232413766", "1432623935232413766");
emoji!("floofPlead", "1432623931889549323", "1432623931889549323");
emoji!("floofSad", "1432623930450903150", "1432623930450903150");
emoji!("floofScared", "1432623927212900443", "1432623927212900443");
emoji!("floofTeehee", "1432623925279457321", "1432623925279457321");
emoji!("floofTired", "1432623921622159493", "1432623921622159493");
emoji!("floofSmug", "1432623919696969788", "1432623919696969788");
#[rustfmt::skip]
emoji!("floofSplode", "1432623909441900585", "1432623909441900585", true);
emoji!("floofWhat", "1432623917536641144", "1432623917536641144");
emoji!("floofWoozy", "1432623915590750270", "1432623915590750270");
emoji!("iAmTheLaw", "1432623913942388816", "1432623913942388816");
