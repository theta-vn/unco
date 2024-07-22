use once_cell::sync::Lazy;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Database,
    Surreal,
};

pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub async fn db_conn(address: &str, credentials: Database<'static>) -> Result<(), anyhow::Error> {
    // Connect to the database
    DB.connect::<Ws>(address).await?;
    // Signin as a namespace, database, or root user
    DB.signin(credentials).await?;
    Ok(())
}
