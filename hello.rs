fn main(){
    // let x: i32 = -55;
    // let y: u64 = 109;
    // println!("Signed Integer: {}",x);
    // println!("UnSigned Integer: {}",y);

    let numbers: [i32; 5] = [1,2,3,4,5];

    println!("Number Array: {:?}", numbers);

    let fruits: [&str; 3] = ["apple", "banana", "orange"];
    println!("All the fruits in the basket: {:?}", fruits);
    println!("First fruit in the basket: {}", fruits[0]);
    println!("Second fruit in the basket: {}", fruits[1]);
    println!("Third fruit in the basket: {}", fruits[2]);

    let human: (&str, i32, bool)= ("Alice", 35, true);
    println!("Data of human: {:?}", human);

    //slices:
    let _human_slices: &[i32] = &[1,2,3,4,5]; 
    println!("slices example: {:?}", _human_slices);

    let _animal_slices: &[&str] = &["apple","banana","orange"]; 
    println!("slices example: {:?}", _animal_slices);

    // This includes borrowing: 
    let _slices: &[&String] = &[&"apple".to_string(),&"banana".to_string(),&"orange".to_string()]; 
    println!("slices example: {:?}", _slices);

    //Strings VS String Slices(&str):
    //Strings[growable, mutable, owned,string types](Stored in heap)

    let mut stone_cold: String = String::from("Hell, ");   
    stone_cold.push_str("Yeah");
    println!("Stone Cold says, {}", stone_cold);

    //String slices (&str)[Stored in Stack]: It is not a string itself, it is a portion of a string.

    let string: String = String::from("Boom, Baam!");
    let slice: &str = &string[0..5];
    println!("Slice is: {}", slice);
}