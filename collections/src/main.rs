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

    

}

