# SURREAL_BB8

Surreal_BB8 is an async connection pool for SurrealDB implemented on top of the BB8 library.
Refer to the [bb8 documentation](https://docs.rs/bb8/latest/bb8/) for more information about
how to use BB8 connection pools.

## Usage

### Basic Usage

```rust
use bb8::Pool;
use surreal_bb8::surreal::SurrealConnectionManager;
use surrealdb::engine::remote::mem::Mem;

let sur_mgr = SurrealConnectionManager::<_, Mem>::new(());

let pool = Pool::builder().build(sur_mgr).await.expect("build error");

let connection = pool.get().await.expect("pool error");

connection
   .health()
   .await
   .expect("Connection was not healthy");

println!("Connection is healthy")
```

### With a configuration

```rust
use bb8::Pool;
use surreal_bb8::temp::{compiletime_with_config::SurrealConnectionManager, config::Config};
use surrealdb::{engine::remote::ws::Ws, opt::capabilities::Capabilities};

#[tokio::main]
async fn main() {
   let config = Config::new()
       .capabilities(Capabilities::default().with_guest_access(false))
       .strict();


   let sur_mgr: SurrealConnectionManager<Ws> =
       SurrealConnectionManager::new("127.0.0.1:8000", config);

   let pool = Pool::builder().build(sur_mgr).await.expect("build error");

   let connection = pool.get().await.expect("pool error");

   connection
       .health()
       .await
       .expect("Connection was not healthy");

   println!("Connection is healthy")
}
```

## Issues

If you have an issue, please let us know [here](https://github.com/toadslop/surreal_bb8/issues).
If you have any improvements to make, please send us a [PR](https://github.com/toadslop/surreal_bb8/issues).
