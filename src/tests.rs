use crate::models::hero::HeroWithId;
use super::rocket;
use rocket::http::Status;
use rocket::local::Client;

#[test]
fn test_hello() {
    let client = Client::new(rocket::ignite()).unwrap();
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Hello, world!".into()));
}

#[test]
fn it_works() {
    let hero = HeroWithId {
        id: 1,
        name: String::from("Superman"),
        identity: String::from("Clark Kent"),
        hometown: String::from("Metropolis"),
        age: 32,
        image_url: Some(String::from("https://images.com/superman")),
    };
    let serialized = serde_json::to_string(&hero).unwrap();
    println!("serialized = {}", serialized);
    assert_eq!(
        serialized,
        r#"{"id":1,"name":"Superman","identity":"Clark Kent","hometown":"Metropolis","age":32,"image_url":"https://images.com/superman"}"#
    );

    let deserialized: HeroWithId = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);

    assert_eq!(deserialized.id, 1);
    assert_eq!(deserialized.name, "Superman");
    assert_eq!(deserialized.identity, "Clark Kent");
    assert_eq!(deserialized.hometown, "Metropolis");
    assert_eq!(deserialized.age, 32);
    assert_eq!(
        deserialized.image_url,
        Some(String::from("https://images.com/superman"))
    );
}
