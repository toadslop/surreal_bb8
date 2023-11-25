//! A connection manager for SurrealDB that determines the kind of connection
//! at runtime.

use async_trait::async_trait;
use std::fmt::{self, Debug};
use surrealdb::{
    engine::any::{connect, Any, IntoEndpoint},
    Error, Surreal,
};

/// A `bb8::ManageConnection` for `Surreal<Any>`. If you need to determine what kind
/// of SurrealDb connection you need at runtime.
#[derive(Clone)]
pub struct SurrealAnyConnectionManager<Config>
where
    Config: IntoEndpoint,
{
    /// A valid Surreal configuration, which is any type that implements
    /// `IntoEndpoint`. https://docs.rs/surrealdb/latest/surrealdb/engine/any/trait.IntoEndpoint.html
    config: Config,
}

impl<Config> SurrealAnyConnectionManager<Config>
where
    Config: IntoEndpoint,
{
    /// Create a new `SurrealAnyConnectionManager` with the specified configuration
    /// For possible configuration options, see the Surreal documentation:
    /// https://docs.rs/surrealdb/latest/surrealdb/engine/any/trait.IntoEndpoint.html
    pub fn new(config: Config) -> SurrealAnyConnectionManager<Config> {
        Self { config }
    }
}

#[async_trait]
impl<Config> bb8::ManageConnection for SurrealAnyConnectionManager<Config>
where
    Config: IntoEndpoint + Send + Sync + 'static + Clone,
{
    type Connection = Surreal<Any>;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Ok(connect(self.config.clone()).await?)
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.health().await
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}

impl<Config> fmt::Debug for SurrealAnyConnectionManager<Config>
where
    Config: IntoEndpoint + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SurrealAnyConnectionManager")
            .field("config", &self.config)
            .finish()
    }
}
