// main 是 Rust 程序的入口函数，默认返回单元类型 `()`（可省略不写）
fn main() {

    // 1、在 Rust 中，使用 let 关键字来声明变量
    // 2、变量的命名使用 蛇形命名法（短横线连接） 
    let var_num = 10;

    // 3、变量的打印
    // println! 是宏调用，{} 是格式化占位符
    println!("val: {}", var_num);
}
