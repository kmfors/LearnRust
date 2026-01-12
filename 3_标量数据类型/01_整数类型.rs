/*

- Rust 中有两类数据类型子集：标量（scalar）和复合（compound）
- Rust 是 静态类型语言，即在编译时就必须知道所有变量的类型

- 标量类型代表一个单独的值。Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。

- Rust 整型默认使用 i32，isize 和 usize 的主要应用场景是用作集合的索引。

*/

fn print_type_of<T>(_: &T) {
    println!("Type of num is: {}", std::any::type_name::<T>());
}

fn main() {
    // 有符号整型变量的类型标记与打印
    let num: i8 = 10;
    print_type_of(&num); 

    let num: i16 = 11;
    print_type_of(&num); 

    let num: i32 = 12;
    print_type_of(&num); 

    let num: i128 = 13;
    print_type_of(&num); 

    let num: isize = 14; // 与架构有关
    print_type_of(&num); 

    // 无符号整型变量的类型标记与打印
    let num: u8 = 20;
    print_type_of(&num); 

    let num: u16 = 21;
    print_type_of(&num); 

    let num: u32 = 22;
    print_type_of(&num); 

    let num: u128 = 23;
    print_type_of(&num); 

    let num: usize = 24; // 与架构有关
    print_type_of(&num); 

    
}