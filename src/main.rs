use core::num;
use std::vec;




fn print_str(a_str: &str) {
    // MAKING A MUTABLE GROWABLE STRING FROM A NON-MUTABLE str
    let mut new_string: String = a_str.to_string();
    new_string.push_str(" added words");


    let another_new_string: String = format!("{} added words via format", a_str);

    println!("{}", a_str);
    println!("{}", new_string);
    println!("{}", another_new_string);
}

fn print_string(a_string: String) {
    println!("{}", a_string);
}



// USE SLICES WHEN:
// YOU WANT TO BORROW A PORTION OF A COLLECTION RATHER THAN THE WHOLE COLLECTION
// YOU WANT OT PASS AROUND A REFERENCE TO A SEQUENCE OF ITEMS WITHOUT COPYING THEM
// YOU WANT TO ACCESS A SUBSET OF A COLLECTION WITHOUT COPYING 
// YOU NEED TO DYNAMICALLY GROW OR SHRINK A COLLECTION

// VECTOR STUFF

fn ownership() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    // CREATES A SLICE OF ALL ELEMENTS IN NUMBERS
    let slice: &[i32] = &numbers[..];
    println!("{:?}", slice);
}

fn modifiable() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    // CREATES A SLICE OF ALL ELEMENTS IN NUMBERS
    let slice: &mut [i32] = &mut numbers[..];
    slice[0] = 10;
    // BELOW WOULD FAIL BECAUSE YOU CANT BORROW AS IMMUTABLE IF IT IS ALREADY BORROWED AS MUTABLE
    // CAN ONLY BORROW ONCE
    // let other_slice = &numbers[..];
    println!("slice: {:?}", slice);
}

fn get_item(index: usize){
    //let index: usize = 3;
    let a_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    // UNWRAPPING THE OPTION CREATED WITH .get()
    let value:&i32 = a_vec.get(index).unwrap();
    println!("the value at index {} is: {:?}", index, value);

}


fn main() {

    // str IS NOT MUTABLE AND GROWABLE
    let a_str: &str = "Hello";
    print_str(a_str);


    // String IS GROWABLE AND MUTABLE
    // String IS OWNED BY THE CODE THAT CREATES IT 
    let a_string: String = String::from("World");
    print_string(a_string);


    // GETTING A SLICE OF A String
    let sentence: String = "the quick fox jumps".to_string();
    println!("{}", &sentence[0..=5]);

    // CONCATENATE USING format()
    let description: String = format!("Title: Quick Story\n{}", sentence);
    println!("{}", description);

    //ITERATE OVER CHARACTERS IN A String
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("vowel: {}", c),
            _ => continue,
        
        }
    }

    // SPLIT AND COLLECT INTO A VECTOR
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let words2: Vec<&str> = sentence.split(' ').collect::<Vec<&str>>();
    println!("{:?}", words); 
    println!("{:?}", words2);

    let reversed: String = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);


    ownership();
    modifiable();


    let a_vec = vec![1, 2, 3, 4, 5];

    // RETRIEVE A VALUE AT A SPECIFIED INDEX

    let third_value = a_vec[2];
    println!("{}", third_value);

    // RETRIEVE THE LAST VALUE IN A VECTOR
    // TWO DIFFERENT WAYS TO ACCESS THE LAST VALUE IN A VECTOR
    let last_value: i32 = a_vec[a_vec.len() - 1];
    // UNWRAP IS USED TO UNWRAP THE Some() FOR THE Option
    // IF IT IS EMPTY THE unwrap() WILL CAUSE A PANIC
    let last_value2: &i32 = a_vec.last().unwrap();
    // THIS IS A SAFE WAY TO ACCESS THE LAST VALUE IN A VECTOR VIA USING Option IN CASE VECTOR IS EMPTY
    // WITHOUT THE UNWRAP IT SHOWS THE Some()
    let last_value3_option: Option<&i32> = a_vec.last();
    println!("last_value1: {}", last_value);
    println!("last_value2: {}", last_value2);
    println!("last_value3: {:?}", last_value3_option);


    match a_vec.first() {
        Some(first_value) => println!("first value: {}", first_value),
        None => println!("no first value"),
    }

    get_item(3);

}
