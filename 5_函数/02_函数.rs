/*
  - 函数的几大要素：
    - 声明函数的关键字 fn
    - 函数名 funcname
    - 参数名及类型 Parameters
    - 返回类型 return type

  - 函数参数：Rust是静态类型语言，需要为每一个函数参数都标识它的具体类型
  - 函数返回值：函数体最后一条表达式的返回值，可以 return 提前返回  
    - 函数没有返回值，那么返回一个 ()
    - 通过 ; 结尾的语句返回一个 ()
    - () 可以隐式返回或显式返回

  - 当用 ! 作函数返回类型的时间，表示该函数永不返回，这种特殊语法往往用作
    会导致程序崩溃的函数，也称发散函数

*/

//  表示函数参数的类型
fn first_function(x: i32, y: f32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// 返回最后一条表达式的值
fn plus_or_minus(x: i32) -> i32{
    if x > 5 {
        return x - 5
    }
    x + 5
}

// 发散函数 - 崩溃函数
fn dead_end() -> ! {
    panic!("奔溃吧，颤抖吧！")
}

// 无限循环函数
fn forever() -> ! {
    loop {
        // ...
    };
}

fn main() {

}