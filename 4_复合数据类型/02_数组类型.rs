/*
  复合类型可以将多个值组合成一个类型。
  Rust 有两个原生的复合类型：元组（tuple）和数组（array）。

  数组的长度固定，且数组中的每个元素的类型必须相同。

*/

fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("The a array is:{:?}", a);

    // 指定元素的类型与数量
    let b: [i32; 5] = [6, 7, 8, 9, 10];
    println!("The b array is:{:?}", b);

    // 指定元素的初始值与数量
    let c = [3; 5];
    println!("The c array is:{:?}", c);

    // 访问数组元素
    println!("The 2 index of b is {}", b[2]);

}