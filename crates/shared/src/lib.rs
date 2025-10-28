use sled::Db;
#[cfg(not(debug_assertions))]
use topgg::{Autoposter, autoposter::Serenity};

/// User data, which is stored and accessible in all command invocations
pub struct Data {
    pub db: Db,
    #[cfg(not(debug_assertions))]
    pub topgg_client: topgg::Client,
    // The autoposter is moved here so that it is not dropped, which would stop
    // its thread.
    #[cfg(not(debug_assertions))]
    pub _autoposter: Autoposter<Serenity>,
}

impl Data {
    pub fn write_serialized(&self, key: &str, data: &impl serde::Serialize) -> Result<(), poise_error::anyhow::Error> {
        self.db.insert(key, postcard::to_stdvec(data)?)?;
        Ok(())
    }

    pub fn delete(&self, key: &str) -> Result<(), poise_error::anyhow::Error> {
        self.db.remove(key)?;
        Ok(())
    }
}

pub type Context<'a> = poise::Context<'a, Data, poise_error::anyhow::Error>;
