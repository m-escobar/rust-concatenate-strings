fn main() {
    let string1 = String::from("Hello");
    let string2 = String::from(" from Rust!");

    concatenate_strings(&string1, &string2);
}


fn concatenate_strings(string1: &String, string2: &String) -> () {
    let concatenated_string = string1.to_owned() + string2;

    println!("{:?}", concatenated_string);
}