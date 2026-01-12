/*
  不同编译模式下，对整型溢出的处理方式：

- Debug 模式编译下，会检查整型溢出，若存在该问题，则程序 panic

- Release 模式编译下，不检测溢出。并且当检测到整型溢出时，Rust 会按照补码循环溢出的规则处理。
-- 简而言之：大于该类型最大值的数值会被补码转换成该类型能够支持的对应数字的最小值

- 要显式处理可能的溢出，可以使用标准库针对原始数字类型提供的方法：
-- 使用 wrapping_* 方法，在所有模式下都按照补码循环溢出的规则处理
-- 使用 checked_* 方法，发生溢出时，则返回 None 值
-- 使用 overflowing_* 方法，返回该值和一个指示是否存在溢出的布尔值
-- 使用 saturating_* 方法，可以限定计算后的结果不超过目标类型的最大值或低于最小值

*/

fn main() {
    // u8 范围是 0-255
    let a: u8 = 250;

    // wrapping_* 方法，在所有模式下都按照补码循环溢出的规则处理
    let wrapping_result = a.wrapping_add(10);
    println!("Wrapping add: {}", wrapping_result);

    // checked_* 方法，发生溢出时，则返回 None 值
    let checked_result = a.checked_add(10);
    match checked_result {
        Some(value) => println!("Checked add: {}", value),
        None => println!("Checked add: Overflow occurred"),
    }

    // overflowing_* 方法，返回该值和一个指示是否存在溢出的布尔值
    let (overflowing_result, overflowed) = a.overflowing_add(10);
    println!("Overflowing add: {}, Overflowed: {}", overflowing_result, overflowed);

    
    // saturating_* 限定自身的最大值或最小值
    let saturating_result = a.saturating_add(10);
    println!("Saturating add: {}", saturating_result);
    
}