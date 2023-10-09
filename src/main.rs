use std::primitive;


fn _print(){
    println!("No Way For You");
}
fn _x2( _a : i32) -> i32{
    _a * 2
}


fn main() {
// Data type   
    println!("Hello, world!");
    let _x = 10;
    let _y = 15.3;
    let _boo = false;
    // array : fixed size - cannot be edited, the values of the elements must be of the same data type
    let _array = [10.0, 20.0, 30.0];
    println!("array 1: {}", _array[0]);

    let _tuple = (10 ,10.0, "str");
    println!("tuple 1 : {}", _tuple.0);

// Variable
    // default --> immutable --> variable cannot be changed
    let _a = 10;
    // mutable --> variable can change
    let mut _b = 10;
    _b = _b + 10;
    println!("{}",_b);
    // For const, it is necessary to declare its data type.
    const _PI : f64 = 3.14;

    let mut _s = String::new();
    println!("String is empty: {}", _s.is_empty());
    _s.push('h');
    println!("String is empty: {}", _s.is_empty());
    let mut _str = String::from("Hello World");
    println!("String is: {}",_str); 
    // reference string - only have read prermission
    let _s_two = " Hello";
    _str = _str + _s_two;
    println!("String is: {}",_str); 
    // &str --> String
    let _conversions_string =  "AIBC".to_string();
    // String --> &str
    let _conversions_str = _str.as_str();

    let _a = 1;
    if _a == 1{
        println!("Hello");
    }
    else{
        println!("Bye");
    }
    match _a {
        0 => println!("Jerry"),
        1 => println!("Tom"),
        _ => println!("Dog")
    }

    let _vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    // for value in _vec{
    //     println!("Value: {}",value);
    // }
    for i in _vec.iter(){
       println!("Value: {}",i);
    }
    let _max = _vec.iter().max().unwrap();
    println!("Max : {}",_max);

    _print();
    println!("{}",_x2(3));
}
