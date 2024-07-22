use once_cell::sync::Lazy;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Database,
    Surreal,
};

pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub async fn db_conn() -> Result<(), anyhow::Error> {
    // Connect to the database
    DB.connect::<Ws>("127.0.0.1:8000").await?;
    // Signin as a namespace, database, or root user
    DB.signin(Database {
        namespace: "test",
        database: "test",
        username: "root",
        password: "root",
    })
    .await?;
    Ok(())
}
