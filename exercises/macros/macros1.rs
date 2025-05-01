// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!(); // 添加感叹号! - 在Rust中调用宏需要使用感叹号，这区别于普通函数调用
}
