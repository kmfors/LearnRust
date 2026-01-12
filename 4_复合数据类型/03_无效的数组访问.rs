/*
  当尝试用索引访问一个元素时，Rust 会检查指定的索引是否小于数组的长度。
  如果索引超出了数组长度，Rust会panic，程序会带着错误信息退出。
*/

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array is index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim().parse()
        .expect("Index entered was not a number"); 
    
    let element = a[index];

    println!("The value of the element at index {} is {}", index, element);
}