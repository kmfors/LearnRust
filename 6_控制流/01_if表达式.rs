/*
  - Rust 并不会尝试自动地将非布尔值转换为布尔值，必须总是显式地使用布尔值作为 if 的条件
    if number {...} 必须显式 if number != 0 {...}

  - if 是一个表达式，因此可以在 let 语句右侧使用
    let number = if condition { 5 } else { 6 };  

*/

fn main() {
    
    let number = 6;
    // if 表达式使用
    if number % 4 == 0 {
        println!("number is devisible by 4");
    } else if number % 3 ==  {
        println!("number is devisible by 3");
    } else {
        println!("number is not devisible");
    }

    // if 表达式赋值，注意：if与else分支的值的类型必须是相同的！
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is:{}", number);

}