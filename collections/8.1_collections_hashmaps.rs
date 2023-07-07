use std::collections::HashMap;

fn main() {
    let blue = String::from("Blue");
    let red = String::from("Red");

    let mut scores = HashMap::new();
    
    //adding elements
    scores.insert(blue, 10); //the ownership of the string is now moved to scores hashmap
    scores.insert(red, 50);  //therefore we cannot use the variables red and blue anymore
    scores.insert("new_team".to_owned(), 100);
    scores.insert("secret_team".to_owned(), 25);


    //getting a value
    let blue_score = &scores.get("Blue"); // we get an option because we cannot gurantee if a value is present
    

    //iterating over a hashmap
    for (k, v) in &scores {
        println!("{} --> {}", k, v);
    }

    //optional add -> will do nothing if key already exists
    //this uses an entry enum
    scores.entry("Red".to_owned()).or_insert(30); //if key does not exist, insert 30
    scores.entry("Purple".to_owned()).or_insert(90); //since purple doesn't exist, it will be added

    //or_insert will return a mutable reference which can be used in a variety of ways
    let text = "hello world what a wonderul world Bobby bro";
    let mut map = HashMap::new();
    // word counter
    for w in text.split_whitespace() {
        let count = map.entry(w).or_insert(0);
        *count += 1;
    }

    //modifying key if exists
    //method 1 -> deferencing or_insert() result and then modifying directly
    *scores.entry("Red").or_insert(0) += 1;
    //method 2 -> and_modify
    scores.entry("Red").and_modify(|c| c*100); //modifies the value through a closure
    //using both and_modify and or_insert together
    scores.entry("Purple").and_modify(|c| c+=1).or_insert(1); //checks if value exists and modifies; if value doesn't exist, inserts value of 1

    //deleting a key
    scores.remove("Purple"); //the value specified should be a reference. By default strings are reference slices. so it works here

    println!("{:?}", map);


}