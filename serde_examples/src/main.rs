use serde::{Deserialize, Serialize};
use serde_json;
use serde_yaml;
use std::fmt;

fn main() {
    let user = User {
        name: "Foo".to_string(),
        age: 23,
    };

    println!("STRUCT:{}\n", user);
    json(&user);
    yaml(&user);
    toml(&user);
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u8,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\nname: {}\nage: {}", self.name, self.age,)
    }
}

fn json(user: &User) {
    let serialize = serde_json::to_string_pretty(&user).unwrap();
    //let deserialize: User = serde_json::from_str(&serialize).unwrap();
    println!("JSON:\n{serialize}\n");
    //println!("deserialize: {deserialize}");
}

fn yaml(user: &User) {
    println!("YAML:\n{}", serde_yaml::to_string(&user).unwrap());
}

fn toml(user: &User) {
    println!("TOML:\n{}", toml::to_string(&user).unwrap());
}
