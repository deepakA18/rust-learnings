
use unicode_segmentation::UnicodeSegmentation;

use std::collections::HashMap;
fn main() {
    let a = [1,2,3]; //array initialisation
    let mut v: Vec<i32> = Vec::new();  // 'new' keyword will create an empty vector.

    v.push(1);
    v.push(2);
    v.push(3);    

    //another method to initialise vector with values

    {
        let v2= vec![1,2,3];   //the elements in this vector will be dropped as the scope ends
    }
    

    //Accessing elements inside of the vector:
    let v1 = vec![1,2,3,4,5];

    let third = &v1[2];
    println!("The third element is {}", third);

    //the "get" method to access the elements inside the vector:
    //The "get" method returns an Option<T>
    //Helps to access vector elements without crashing program if we go 'out of bound'.
    match v.get(2) {
        Some(third) => println!("third element is {}", third),
        None => println!("There is no third element!")
    }

    //**Note**: If u are accessing any element inside the vector, you cannot modify the vector after that, as it is already borrowed as immutable.

    //Iterating over all elements in vector:

    let mut iv = vec![1,2,3,4,5];

    for i in &iv {
        println!("Elements in iv: {}", i);  //taking immutable reference to each element and then printing it out.
    }

    //we can also take mutable reference:

    for i in &mut iv {
        *i += 50;      //used "deferencing" operator:
        println!("{}", i);
    }

    //storing enum variance inside of a vector:

    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(2.45)
    ];

    match &row[1] {
        SpreadSheetCell::Int(i) => println!("{}", i),
        _ => println!("Not a integer!")
    }

    

    
    //*******************Strings*********************:
    //In rust strings are stored as UTF-8 encoded types(1,0)

    //Note: In UTF-8, single char can be 1-4 bytes long

    //Creating a String:

    let s1 = String::new(); //defining an empty string
    let s2 = "Initial String";
    let s3 = s2.to_string();
    let s4 = String::from("Alright!");

    //Similar to Vector, a String can also grow and shrink in size:

    let mut s = String::from("foo");
    s.push_str("bar");  //push_str() is used to append the string 
    s.push('!');   //push() is used to append at the end of the string

    println!("string is: {:?}", s);

    //we can also use '+' operator to append the string

    let s5 = String::from("Hello");
    let s6 = String::from("world!");
    let s7 = s5 + &s6;  //we cannot add both reference, String on the left needs to be owned

    println!("the string s7 is: {:?}", s7);


    //we can also concate the strings using 'format' macro.
    let s8 = String::from("solana!");

    let s9 = format!("{}{}", s8,s6);

    println!("the string s9 is {:?}", s9);

    //Strings can be in three types:

    let hello = String::from("नमस्ते");
    //1. Bytes
    //[224,164,168,224,164,174,224,164,184,224,165,141,224,164,164,224,165,135]
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    //2. Scalar values (what RUST considers)
    // [न,म,स,्,त,े]

    for b in "नमस्ते".chars() {
        println!("{}", b);
    }

    //3. Grapheme clusters (human assumptions of chars): there is no direct fn to access grapheme clusters in RUST, we have to impot 'Unicode-segmentation
    //["न", "म", "स्", "ते"]

    for g in "नमस्ते".graphemes(true) {
        println!("{}", g);
    }


    //**********************Hashmaps*********************
    // Hashmaps allow you to store key-value pairs, keys and values could be of any type
    // Also, it uses a hashing function to determine how to store keys and values in the memory

    //example

    let blue = String::from("blue");
    let yellow = String::from("yellow");

    let mut scores = HashMap::new(); //Similar to vectors and strings

    scores.insert(blue, 10);  //ownership of blue and yellow is moved here, we are not passing references.
    scores.insert(yellow, 5);

    let team_name = String::from("blue");

    let score = scores.get(&team_name);   // the 'get' method takes a reference to the key and returns an optional value
// score if of type option becoz we cannot guarantee it will return or not

    for (key, value) in &scores {
        println!("{} {}", key, value);
    }


    //***Updating Hashmaps***


    let mut score_board = HashMap::new();

    score_board.insert(String::from("blue"), 10);
    score_board.insert(String::from("blue"), 20);  //Here it will overwrite the value 20 with 10.

    //If don't want to overwrite the existing values:
    //1. 'entry' enum that represents the value of a given key.
    scores.entry(String::from("yellow")).or_insert(30);  //if there isn't any entry then insert value 30. if there is entry then do nothing
    scores.entry(String::from("yellow")).or_insert(40); 

    //***Updating value in Hashmap based on the old value***
    // 'or_insert()' function returns a mutable reference of a value

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}",map);
    


}

