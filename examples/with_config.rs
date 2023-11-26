use bb8::Pool;
use surreal_bb8::temp::{compiletime_with_config::SurrealConnectionManager, config::Config};
use surrealdb::{engine::remote::ws::Ws, opt::capabilities::Capabilities};

// You'll need a running Surreal instance for this to work.
#[tokio::main]
async fn main() {
    let config = Config::new()
        .capabilities(Capabilities::default().with_guest_access(false))
        .strict();

    // Note that this version of the connection manager is a temporary workaround until
    // surrealdb version 1.1.0 drops. It is necessary because version 1.0.0 does not
    // implement Clone on Config, but a connection manager must be able to clone the
    // configuration to work properly. As version 1.1.0 provides Clone for Config,
    // this will be become deprecated at that time.
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
