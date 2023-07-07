use std::cmp::Reverse;

fn main() {
    // 1st version of ascii -> 1 character = 7 bit. But this was before www
    // After www, there were a lot of confusion
    // Hence unicode consordium was created - which created Unicode Transformation Format UTF where characters were stored using 8 bits.
    
    // Post this, UTF-8 came about which is variable length encoding. this means a character could range between 1 to 4 bytes. Hence 1,112,064 characters could be stored.
    /*
        if a character is using 1 byte -> 0xxxxxxx = starts with 0. 
        if a character uses 2 bytes -> 110xxxxx 10xxxxxx = starts with 110 and the next byte will start with 10
        if a character uses 3 bytes -> 1110xxxx 10xxxxxx 10xxxxxx = starts with 1110 and the next byte will start with 10
        if a character uses 4 bytes -> 11110xxxx 10xxxxxx 10xxxxxx = starts with 11110 and the next byte will start with 10


            + The starting tells us how many bytes it is using
            + The next byte's starting - 10 tells us that this is the starting of the next byte
    */
    // UTF-8 is backward compatible with ascii
    // text in Rust are stored as a colection of UTF-8 encoded bytes. This is useful for string manupulations



    // there are 2 types of strings -  &str and String

    /*
    String

    - this can grow and shrink
    - hence it is allocated in the heap
    - three values are tracked
        - adddress pointing to the first byte of the string
        - length of the string
        - capacity of the string
    - the data is owned by the string variable
    - hence we can manupulate the data
    */
    
    // in utf8, strings are stored as variable length bytes, chars or grapheme clusters
    for byte in "नमस्ते".bytes() {
        println!("{}", byte);
    }
    
    for ch in "नमस्ते".chars() {
        print!("{}", ch)
    }

    // for grapheme_char in "नमस्ते".graphemes(true) {
    //     println!(grapheme_char);
    // }


    //1. creating strings
    let s1 = String::new();
    let s2 = 120000.to_string();
    let s4 = String::from("babes");

    // 2. string concatination
    let mut s5 = s1 + &s4 + " " + &s2 + " cool"; // the ownership of s1 is moved to s5 here
    println!("{}", s5);
    println!("{}", s2); // but s2 will still print out because the ownership still stays with s2
    // using a format macro to concat
    let s6 = format!("{} {}", &s2, &s4);
    //using concat macro
    let s11 = concat!("foo", " ", "bar");
    let s7 = ["seven ", "eight"].concat();

    //3. append to a string
    let s10 = s5.push_str(" katana");

    //4. replace a portion of a string
    s5.replace_range(0..5,"baz "); //we are replacing the entire string with

    //5. length of a string
    s4.len()
    
    //6. is empty
    s4.empty()
    
    //7. split the string
    let mut splstr = String::from("hi there, how are you doing ?");
    //will return a iterator of string slices - &str
    splstr.split(",")
    splstr.split_whitespace() //split by space
    splstr.chars() //split into characters - wil return an enum
    
    //8. string contains
    splstr.contains("are")

    //9. replace something in a string
    splstr.replace("you", "you all"); 

    //10. iterate lines in a string
    let mystring:String = String::from("The weather is \nnice today.\nlets go out");
    for line in mystring.lines() {
        println!("[{}]", line)
    };

    /*
    &str

    - &str is a view of the reprsentation of the string in utf8 format on stack or on heap. 
    - Hence we will not know the length in compile time.
    - Therefore we always use the borrowed form &str and not str directly.
    - This is also called a string slice. we do know the length of the string slice because
        - it stores the address pointing to the 1st byte of the string
        - and it also stores the length of the string
    - The string slice does not own the string.
    - So if you just need an immutable view of a string or just a part of the string, then use a string slice 
    */
    let s2 = "initial contents"; //this cannot be manupulated
    let s22 = &s2[..];
    // converting a slice to owned string
    let s8 = "Ambitiouus men".to_owned();
    let s9 = s2.to_owned();
    
    //3. Indexing - we cannot index into a string in constant time. we have to iterate over the bytes, characters or graphemes
    // hence we can use string slices
    let mystr = String::from("Srivatsa");
    println!("{}", &mystr[..3]);
    let first_name = String::from(&mystr[0..3]);
    let last_name = String::from(&mystr[3..]);
    println!("{} {}", first_name, last_name); 
    
    //working with a slice of string
    let blocks = "WBBWWWBBBWW";
    let blocksv: Vec<char> = blocks.chars().collect();
    println!("{:?}", blocksv[0..5].iter().collect::<Vec<&char>>());

    //converting a string to an array and an array back to string
    let sample = "HelloWorld";
    let sample_arr = sample.chars();
    print!("{}", sample.iter().collect::<String>);

    //reversing a string
    let mut rev_str = "Hello";
    rev_str = String::from_iter(rev_str.chars().rev());
    println!("{}", rev_str);



}