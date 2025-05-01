// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut res = 42;
    let option = Some(12);
    // 修改: 不应在Option上使用for循环，改为if let模式匹配
    // 原因: Option不是一个真正的集合类型，for循环在这里不是惯用法
    // Clippy建议使用if let或while let来处理Option类型
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
