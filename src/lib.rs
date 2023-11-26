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
//! use surreal_bb8::temp::{compiletime_with_config::SurrealConnectionManager, config::Config};
//! use surrealdb::{engine::remote::ws::Ws, opt::capabilities::Capabilities};
//!
//!#[tokio::main]
//!async fn main() {
//!    let config = Config::new()
//!        .capabilities(Capabilities::default().with_guest_access(false))
//!        .strict();
//!
//!
//!    let sur_mgr: SurrealConnectionManager<Ws> =
//!        SurrealConnectionManager::new("127.0.0.1:8000", config);
//!
//!    let pool = Pool::builder().build(sur_mgr).await.expect("build error");
//!
//!    let connection = pool.get().await.expect("pool error");
//!
//!    connection
//!        .health()
//!        .await
//!        .expect("Connection was not healthy");
//!
//!    println!("Connection is healthy")
//!}
//! ```
//!

#![deny(missing_docs, missing_debug_implementations)]

pub use bb8;
pub use surrealdb;

pub mod compiletime;
pub mod runtime;
pub mod temp;
