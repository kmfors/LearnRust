/*

- 在 Rust 中允许声明相同的变量名，在后面声明的变量会遮蔽掉前面声明的

作用：
- 利用遮蔽的原理，可以重复使用之前的变量名字，并且不必拘泥于类型

*/

fn main() {
    let x = 5;
    let x = x + 1; // 当前作用域下，对上面的 x 进行遮蔽

    {
        // 当前花括号作用域内，遮蔽外层的同名变量
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    let x = "324";

    println!("The value of x is: {}", x);
}