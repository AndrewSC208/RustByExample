use std::collections::HashMap;

fn main() {
	// create a new HashMap to hold a key/val pair of "subject" and "mark"
	let mut marks = HashMap::new();

	// add two new values to the HashMap
	marks.insert("Business 101", 75);
	marks.insert("Accounting", 50);
	marks.insert("Golf", 100);
	marks.insert("Friends", 0);

	// find length of HashMap
	println!("How many subjects have you studied? {}", marks.len());

	// Get a single value from the HashMap
	match marks.get("Accounting") {
		Some(mark) => println!("you got {} for Accounting", mark),
		None => println!("you did not study Accounting") 	
	}

	// remove a value from the HashMap
	marks.remove("Business 101");

	// loop through HashMap, using a for in loop
	for(subject, mark) in &marks {
		println!("* For {} you got {}%", subject, mark);
	}

	// check for a value in the HashMap using the key
	println!("Did you study Marketing? {}", marks.contains_key("Marketing"));
}
