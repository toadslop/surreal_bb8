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
///
/// ## Known issues
/// Due to the fact that [surrealdb::opt::Config] does not implement clone in version
/// 1.0.0, it is not possible to use a configuration on this [SurrealConnectionManager].
/// If you need to configure your connection, use the temporary workaround in
/// [crate::temp::runtime_with_config::SurrealConnectionManager].
#[derive(Clone)]
pub struct SurrealConnectionManager<Config>
where
    Config: IntoEndpoint,
{
    /// A valid Surreal configuration, which is any type that implements
    /// [surrealdb::engine::any::IntoEndpoint]. Refer to the documentation for various configuration options.
    config: Config,
}

impl<Config> SurrealConnectionManager<Config>
where
    Config: IntoEndpoint,
{
    /// Create a new [SurrealConnectionManager] with the specified configuration
    /// For possible configuration options, see the Surreal documentation for
    /// [surrealdb::engine::any::IntoEndpoint]
    pub fn new(config: Config) -> SurrealConnectionManager<Config> {
        Self { config }
    }
}

#[async_trait]
impl<Config> bb8::ManageConnection for SurrealConnectionManager<Config>
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

impl<Config> fmt::Debug for SurrealConnectionManager<Config>
where
    Config: IntoEndpoint + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SurrealConnectionManager")
            .field("config", &self.config)
            .finish()
    }
}
