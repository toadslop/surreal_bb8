//! A connection manager for SurrealDB that determines the kind of connection
//! at compile time.

use async_trait::async_trait;
use std::fmt::{self, Debug};
use std::marker::PhantomData;
use surrealdb::opt::IntoEndpoint;
use surrealdb::{Error, Surreal};

/// A `bb8::ManageConnection` for `Surreal`. Use this implementation if you
/// know at compile time what kind of Surreal database client you need.
#[derive(Clone)]
pub struct SurrealConnectionManager<Config, Scheme>
where
    Config: IntoEndpoint<Scheme>,
{
    /// A valid Surreal configuration, which is any type that implements
    /// `IntoEndpoint`. https://docs.rs/surrealdb/latest/surrealdb/opt/trait.IntoEndpoint.html
    config: Config,
    scheme: PhantomData<Scheme>,
}

impl<Config, Scheme> SurrealConnectionManager<Config, Scheme>
where
    Config: IntoEndpoint<Scheme>,
{
    /// Create a new `SurrealConnectionManager` with the specified configuration
    /// For possible configuration options, see the Surreal documentation:
    /// https://docs.rs/surrealdb/latest/surrealdb/opt/trait.IntoEndpoint.html.
    pub fn new(config: Config) -> SurrealConnectionManager<Config, Scheme> {
        Self {
            config,
            scheme: PhantomData,
        }
    }
}

#[async_trait]
impl<Config, Scheme> bb8::ManageConnection for SurrealConnectionManager<Config, Scheme>
where
    Config: IntoEndpoint<Scheme> + Send + Sync + 'static + Clone,
    Scheme: Send + Sync + 'static,
{
    type Connection = Surreal<<Config as IntoEndpoint<Scheme>>::Client>;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Ok(Surreal::new(self.config.clone()).await?)
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.health().await
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}

impl<Config, Scheme> fmt::Debug for SurrealConnectionManager<Config, Scheme>
where
    Config: IntoEndpoint<Scheme> + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SurrealConnectionManager")
            .field("config", &self.config)
            .finish()
    }
}
