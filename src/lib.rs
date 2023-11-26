//! # SURREAL_BB8
//!
//! SurrealDB support for the `bb8` connection pool.
//!
//! ## Basic Usage
//!
//! ```
//! use bb8::Pool;
//! use surreal_bb8::surreal::SurrealConnectionManager;
//! use surrealdb::engine::remote::mem::Mem;
//!
//! let sur_mgr = SurrealConnectionManager::<_, Mem>::new(());
//!
//! let pool = Pool::builder().build(sur_mgr).await.expect("build error");
//!
//! let connection = pool.get().await.expect("pool error");
//!
//! connection
//!    .health()
//!    .await
//!    .expect("Connection was not healthy");
//!
//! println!("Connection is healthy")
//! ```
//!
//! ## With a configuration
//!
//! ```
//! use bb8::Pool;
//! use surreal_bb8::surreal::SurrealConnectionManager;
//! use surrealdb::engine::remote::mem::Mem;
//!
//! let sur_mgr = SurrealConnectionManager::<_, Mem>::new(());
//!
//! let pool = Pool::builder().build(sur_mgr).await.expect("build error");
//!
//! let connection = pool.get().await.expect("pool error");
//!
//! connection
//!    .health()
//!    .await
//!    .expect("Connection was not healthy");
//!
//! println!("Connection is healthy")
//! ```
//!
//! ## Limitations
//!
//! The current implementation of SurrealDb's [surrealdb::opt::Config] struct lacks
//! a [Clone] implementation. However, in order to manage multiple connections in
//! a connection pool, we need to be able to clone the configuration. For this reason,
//! it currently isn't possible to use this connection pool with a custom configuration.
//! This limitation will be lifted when SurrealDb version 1.1.0 is released, as this
//! new version includes an implementation of [Clone] on `Config`.
//!

#![deny(missing_docs, missing_debug_implementations)]

pub use bb8;
pub use surrealdb;

pub mod compiletime;
pub mod runtime;
