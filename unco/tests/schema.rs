use serde_json::json;
use std::collections::HashMap;
use unco::*;
use unco_derives::*;
schema! {
  Post {
    id: UncoId,
    title: String ~{require: true},
    des: String ~{require: true},
    count: i64,
    valid: bool,
    array_number: Vec<i64>,
    array_string: Vec<String>,
  }
}

#[test]
fn test_create() {
    let post = Post::default();
    dbg!(&post);

    let null = post.title();
    dbg!(null);

    let json = json!({
      "title": "Title",
      "des": "Description",
      "count": 1,
      "valid": true,
      "array_number": [1,2],
      "array_string": ["aaa", "bb"],
      "id": {
        "tb": "Post",
        "id": {
          "String": "xfvpol2uogqmdrznflb4"
        }
      },
    });

    let post_json = Post::from(json);
    dbg!(&post_json);

    let title = post_json.title();
    dbg!(title);

    let i_64 = post_json.count();
    dbg!(i_64);

    let valid = post_json.valid();
    dbg!(valid);

    let arrayn = post_json.array_number();
    dbg!(arrayn);

    let arrays = post_json.array_string();
    dbg!(arrays);

    let id = post_json.id();
    dbg!(id);
}

#[test]
fn test_cast() {
    let json_id = json!({
      "title": "Title json",
      "des": "Description json",
    });

    let post = Post::from(json_id);
    dbg!(&post);

    let mut hmap = HashMap::new();
    hmap.insert(
        "title".to_string(),
        serde_json::Value::String("Title Change".to_string()),
    );

    hmap.insert(
        "no_title".to_string(),
        serde_json::Value::String("Dont cast".to_string()),
    );

    let changes = post.cast(hmap);
    dbg!(&changes);

    let validate = changes.unco_validate();
    dbg!(&validate);

    let changeset = validate.changeset();
    dbg!(&changeset);

    let data = validate.data();
    dbg!(&data);

    let valid = validate.is_valid();
    dbg!(valid);
}
