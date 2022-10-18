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
pub struct UserData {
    #[serde(rename = "data")]
    pub data: InnerData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InnerData {
    #[serde(rename = "age")]
    pub age: f64,

    #[serde(rename = "free")]
    pub free: bool,

    #[serde(rename = "likes")]
    pub likes: Vec<String>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "skills")]
    pub skills: Vec<Skill>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Skill {
    #[serde(rename = "detail")]
    pub detail: String,

    #[serde(rename = "name")]
    pub name: String,
}
