use surrealdb::opt::auth::Database;
use unco::*;

#[tokio::test]
async fn test_conn() -> Result<(), anyhow::Error> {
    // async fn test_conn() {
    let db = Database {
        namespace: "test",
        database: "test",
        username: "root",
        password: "root",
    };
    let address = "127.0.0.1:8000";
    dbg!("TEST");
    let r = db_conn(&address, db).await.unwrap();
    dbg!(r);

    let sql = "SELECT * FROM Account";
    let mut result = DB.query(sql).await?;
    let kt: Vec<serde_json::Value> = result.take(0)?;
    dbg!(kt);

    assert!(true);
    Ok(())
}
