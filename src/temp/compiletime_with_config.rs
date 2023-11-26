//! A connection manager for SurrealDB that determines the kind of connection
//! at compile time.

use async_trait::async_trait;
use std::fmt::{self};
use std::marker::PhantomData;
use surrealdb::opt::{Config as SurrealConfig, IntoEndpoint};
use surrealdb::{Error, Surreal};

use super::config::Config;

/// A [bb8::ManageConnection] for [surrealdb::Surreal]. Use this implementation if you
/// know at compile time what kind of Surreal database client you need.
#[derive(Clone)]
pub struct SurrealConnectionManager<Scheme> {
    /// A valid Surreal configuration, which is any type that implements
    /// [surrealdb::opt::IntoEndpoint]. Refer to the `IntoEndpoint` documentation
    /// for the valid types.
    config: Config,
    path: String,
    scheme: PhantomData<Scheme>,
}

impl<Scheme> SurrealConnectionManager<Scheme> {
    /// Create a new [SurrealConnectionManager] with the specified configuration
    /// For possible configuration options, see the Surreal documentation for
    /// [surrealdb::opt::IntoEndpoint]
    pub fn new(config: Config, path: &str) -> SurrealConnectionManager<Scheme> {
        Self {
            config,
            path: path.to_owned(),
            scheme: PhantomData,
        }
    }
}

#[async_trait]
impl<Scheme> bb8::ManageConnection for SurrealConnectionManager<Scheme>
where
    Scheme: Send + Sync + 'static,
    (std::string::String, surrealdb::opt::Config): surrealdb::opt::IntoEndpoint<Scheme>,
{
    type Connection = Surreal<<(String, SurrealConfig) as IntoEndpoint<Scheme>>::Client>;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Ok(Surreal::new::<Scheme>((self.path.clone(), self.config.clone().into())).await?)
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.health().await
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}

impl<Scheme> fmt::Debug for SurrealConnectionManager<Scheme> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SurrealConnectionManager")
            .field("config", &self.config)
            .finish()
    }
}
