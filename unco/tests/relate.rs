use serde::{Deserialize, Serialize};
use serde_json::json;
use unco_derives::*;
schema! {
  Post {
    id: UncoId,
    title: String ~{require: true},
    des: String ~{require: true},
    cat: Category
  }
}

schema! {
  Category {
    id: UncoId,
    title: String ~{require: true},
  }
}

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

#[test]
fn test_create() {
    let post = Post::default();
    dbg!(&post);

    let json = json!({
      "title": "Title",
      "des": "Description",
      "id": {
        "tb": "Account",
        "id": {
          "String": "xfvpol2uogqmdrznflb4"
        }
      },
      "cat": {
        "title": "Drupal",
        "id": {
          "tb": "Account",
          "id": {
            "String": "xfvpol2uogqmdrznfl34"
          }
        }
      }
    });

    let post_json = Post::from(json);
    dbg!(&post_json);

    let cat_value = post_json.cat_value();
    dbg!(&cat_value);

    // let cat = post_json.cat();
    // dbg!(&cat);

    let cat = serde_json::from_value::<Category>(cat_value.clone());
    let a = match cat {
        Ok(c) => c,
        Err(_) => Category::from(cat_value),
    };
    dbg!(&a);

    let cat = post_json.cat();
    dbg!(&cat);

    // let id_string = id.to_string();
    // dbg!(&id_string);
}
