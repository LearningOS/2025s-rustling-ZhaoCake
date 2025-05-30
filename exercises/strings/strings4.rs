// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned()); // to_owned() : converts a &str to String 
    string("nice weather".into());  // into : converts a &str to String, 与 to_owned() 类似，但区别在于 to_owned() 是一个方法，而 into() 是一个 trait
    string(format!("Interpolation {}", "Station"));  // format!() : creates a String from a formatted string
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));  // replace() : replaces a substring with another substring
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
