//! A temporary workaround module to allow building connection managers with configurations,
//! despite the missing [Clone] implementation on SurrealDb's [surrealdb::opt::Config].

pub mod compiletime_with_config;
pub mod config;
pub mod runtime_with_config;
