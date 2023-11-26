//! A connection manager for SurrealDB that determines the kind of connection
//! at runtime.

use async_trait::async_trait;
use std::fmt::{self, Debug};
use surrealdb::{
    engine::any::{connect, Any, IntoEndpoint},
    Error, Surreal,
};

/// A [bb8::ManageConnection] for [surrealdb::Surreal<Any>]. If you need to determine what kind
/// of SurrealDb connection you need at runtime, use this connection manager.
#[derive(Clone)]
pub struct SurrealRuntimeConnectionManager<Config>
where
    Config: IntoEndpoint,
{
    /// A valid Surreal configuration, which is any type that implements
    /// [surrealdb::engine::any::IntoEndpoint]. Refer to the documentation for various configuration options.
    config: Config,
}

impl<Config> SurrealRuntimeConnectionManager<Config>
where
    Config: IntoEndpoint,
{
    /// Create a new [SurrealRuntimeConnectionManager] with the specified configuration
    /// For possible configuration options, see the Surreal documentation for
    /// [surrealdb::engine::any::IntoEndpoint]
    pub fn new(config: Config) -> SurrealRuntimeConnectionManager<Config> {
        Self { config }
    }
}

#[async_trait]
impl<Config> bb8::ManageConnection for SurrealRuntimeConnectionManager<Config>
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

impl<Config> fmt::Debug for SurrealRuntimeConnectionManager<Config>
where
    Config: IntoEndpoint + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SurrealRuntimeConnectionManager")
            .field("config", &self.config)
            .finish()
    }
}
