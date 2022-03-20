// #[macro_use]
// extern crate serde_derive;
#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

pub fn siriarudaaaaa() -> Result<(), Box<dyn std::error::Error>> {
    // pub fn siriarudaaaaa() -> Result<(), Error> {
    let tarou = Person {
        name: "太郎".to_string(),
        age: 18,
    };
    let json = serde_json::to_string(&tarou)?;
    // let json = serde_json::to_string(&tarou);
    println!("{}", json);

    let json = r#"{ "name": "花子", "age": 68 }"#;
    let hanako: Person = serde_json::from_str(json)?;
    // let hanako: Person = serde_json::from_str(json);
    println!("{:?}", hanako);

    Ok(())
}
