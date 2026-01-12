/*

  Rust 的 char 类型大小为四个字节，并代表了一个 Unicod e标量值，这意味着它可以比 
  ASCII 表示更多的内容。

*/


fn main() {

    // 布尔类型
    let _t = true;
    let _f: bool = false;

    // 字符类型
    let _c = 'z';
    let z: char = '中';
    println!("{} 占用 {} 字节内存大小", z, size_of_val(&z));
}