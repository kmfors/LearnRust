/*
  - 无限循环 loop 

  - 循环标签：在多个循环之间消除歧义。如果存在嵌套循环，break和continue应用于此时
    最内层的循环，可以选择在一个循环上指定一个循环标签，然后将标签与break或continue一起使用
    使这些关键字应用于已标记的循环而不是最内层的循环
*/
fn main() {

    let mut counter = 0;
    // 从循环返回值
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // 循环标签的使用
    let mut count = 0;
    'counting_up: loop { // 定义一个counting_up的循环标签
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
}