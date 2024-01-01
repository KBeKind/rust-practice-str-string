
fn print_str(a_str: &str) {
    println!("{}", a_str);
}

fn print_string(a_string: String) {
    println!("{}", a_string);
}


fn main() {
    let a_str: &str = "Hello";
    print_str(a_str);

    let a_string: String = String::from("World");
    print_string(a_string);
}
