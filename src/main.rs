use demo::parser;
fn main() {
    if let Err(e) = parser::run() {
        println!("执行失败，错误为{}", e);
    }
}
