/*
  - 转移所有权
    for item in collection

  - 不可借用
    for item in &collection

  - 可变借用
    for item in &mut collection

  - 循环控制可以使用 _ ，含义为忽略该值或类型的意思，如果不使用 _，那么
    编译器会给出变量未使用的警告    
*/

fn main() {

    // 循环输出从 1 到 5 的序列
    for i in 1..5 {
        println!("{}", i);
    }

    // 第一种循环方式
    let collection = [1, 2, 3, 4, 5];
    for i in 0..collection.len() {
        let item = collection[i];
        // ...
    }

    // 第二种循环方式（推荐）
    for item in collection {

    }
}