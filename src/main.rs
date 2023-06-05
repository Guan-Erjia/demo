use demo;
fn main() {
    if let Err(e) = demo::run() {
        println!("执行失败，错误为{}", e);
    }
}
