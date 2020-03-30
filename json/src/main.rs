extern crate serde_json;
extern crate serde;

#[macro_use]


extern crate serde_derive;

// Useing the `as` to alias in the use declaration of serde_json. Now I will reference JsonValue and that will be the serde create
use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize)]
struct Person {
	name: String,
	age: u8,
	isMale: bool,
}

fn main() {
	let json_str = r#"
		{
			"name": "Andrew",
			"age": 33,
			"isMale": true
		}
	"#;

	let res = serde_json::from_str(json_str);
	if res.is_ok() {
		let p: Person = res.unwrap();
		println!("the name is {}", p.name);
	} else {
		println!("sorry could not parse")
	}
}
