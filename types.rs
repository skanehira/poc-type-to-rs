// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
pub struct Types {
    #[serde(rename = "$schema")]
    pub schema: String,

    #[serde(rename = "definitions")]
    pub definitions: Definitions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Definitions {
    #[serde(rename = "InnerData")]
    pub inner_data: Data,

    #[serde(rename = "UserData")]
    pub user_data: UserData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "properties")]
    pub properties: InnerDataProperties,

    #[serde(rename = "type")]
    pub data_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InnerDataProperties {
    #[serde(rename = "age")]
    pub age: Age,

    #[serde(rename = "free")]
    pub free: Age,

    #[serde(rename = "likes")]
    pub likes: Likes,

    #[serde(rename = "name")]
    pub name: Age,

    #[serde(rename = "skills")]
    pub skills: Skills,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Age {
    #[serde(rename = "type")]
    pub age_type: Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Likes {
    #[serde(rename = "items")]
    pub items: Age,

    #[serde(rename = "type")]
    pub likes_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Skills {
    #[serde(rename = "items")]
    pub items: Items,

    #[serde(rename = "type")]
    pub skills_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Items {
    #[serde(rename = "properties")]
    pub properties: ItemsProperties,

    #[serde(rename = "type")]
    pub items_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemsProperties {
    #[serde(rename = "detail")]
    pub detail: Age,

    #[serde(rename = "name")]
    pub name: Age,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    #[serde(rename = "properties")]
    pub properties: UserDataProperties,

    #[serde(rename = "type")]
    pub user_data_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDataProperties {
    #[serde(rename = "data")]
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "boolean")]
    Boolean,

    #[serde(rename = "number")]
    Number,

    #[serde(rename = "string")]
    String,
}
