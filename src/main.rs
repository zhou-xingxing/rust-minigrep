use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("缺少必要的参数");
    }
    let query = &args[1];
    let file_path = &args[2];
    let contents = fs::read_to_string(file_path).expect("读取文件失败");
    println!("query: {}", query);
    println!("file contents: {}", contents);
}
