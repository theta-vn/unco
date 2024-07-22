mod conn;
pub use conn::{db_conn, DB};
use serde::{Deserialize, Serialize};
pub use unco_derives::schema;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct UncoId {
    tb: String,
    id: StringId,
}

impl UncoId {
    pub fn to_string(&self) -> String {
        let table = self.tb.clone();
        let id = self.id.to_string();
        format!("{}:{}", table, id)
    }
    pub fn get_id(&self) -> String {
        self.id.to_string()
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct StringId {
    String: String,
}

impl StringId {
    pub fn to_string(&self) -> String {
        self.String.clone()
    }
}
