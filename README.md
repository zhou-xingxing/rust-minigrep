## Rust练手项目
一个文本内容匹配命令行工具
## 使用
```
# 单元测试
cargo test
# 使用clippy检测代码
cargo clippy
# 执行
cargo run -- [-i] -q=<query_string> -f=<file_path>
## 一个例子
cargo run -- -i -q=hello -f=./test.txt
```
