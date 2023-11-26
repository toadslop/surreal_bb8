//! A connection manager for SurrealDB that determines the kind of connection
//! at runtime.

use async_trait::async_trait;
use std::fmt::{self};
use surrealdb::{
    engine::any::{connect, Any},
    Error, Surreal,
};

use super::config::Config;

/// A workaround for current limitations in the SurrealDb codebase. SurrealDb's [surrealdb::opt::Config]
/// struct does not implement [Clone] in the current version, making it impossible to use in a connection
/// pool as we need to be able to clone the configuration for each new connection. This workaround
/// allows you to configure the connection, but as a downside you may only pass the address to your
/// Surreal instance as a string.
#[derive(Clone)]
pub struct SurrealConnectionManager {
    /// A valid Surreal configuration, which is any type that implements
    /// [surrealdb::engine::any::IntoEndpoint]. Refer to the documentation for various configuration options.
    config: Config,
    path: String,
}

impl SurrealConnectionManager {
    /// Create a new [SurrealConnectionManager] with the specified configuration
    /// For possible configuration options, see the Surreal documentation for
    /// [surrealdb::engine::any::IntoEndpoint]
    pub fn new(config: Config, path: &str) -> SurrealConnectionManager {
        Self {
            config,
            path: path.to_owned(),
        }
    }
}

#[async_trait]
impl bb8::ManageConnection for SurrealConnectionManager {
    type Connection = Surreal<Any>;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Ok(connect((self.path.clone(), self.config.clone().into())).await?)
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.health().await
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}

impl fmt::Debug for SurrealConnectionManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SurrealConnectionManager")
            .field("config", &self.config)
            .finish()
    }
}
