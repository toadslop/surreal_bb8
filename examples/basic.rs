use bb8::Pool;
use surreal_bb8::compiletime::SurrealConnectionManager;
use surrealdb::engine::remote::ws::Ws;

// You'll need a running Surreal instance for this to work.
#[tokio::main]
async fn main() {
    let sur_mgr = SurrealConnectionManager::<_, Ws>::new("127.0.0.1:8000");

    let pool = Pool::builder().build(sur_mgr).await.expect("build error");

    let connection = pool.get().await.expect("pool error");

    connection
        .health()
        .await
        .expect("Connection was not healthy");

    println!("Connection is healthy")
}
