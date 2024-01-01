


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

}
