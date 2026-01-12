/*

- Rust 中有两类数据类型子集：标量（scalar）和复合（compound）
- Rust 是 静态类型语言，即在编译时就必须知道所有变量的类型

- 标量类型代表一个单独的值。Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。

- Rust 整型默认使用 i32，isize 和 usize 的主要应用场景是用作集合的索引。

*/

fn main() {
    
    // 各个进制下的数字，打印时都转换为十进制的数字
    let dec: i32 = 98_222;
    println!("decimal(十进制):{}", dec);


    let hex: i32 = 0xff;
    println!("hex(十六进制):{}", hex);

    let oct: i32 = 0o77;
    println!("octal(八进制):{}", oct);

    let bin: i32 = 0b1111_0000;
    println!("binary(二进制):{}", bin);

    let byte: u8 = b'A'; // 仅限于 u8 类型
    println!("byte(单字节字符):{}", byte);

}