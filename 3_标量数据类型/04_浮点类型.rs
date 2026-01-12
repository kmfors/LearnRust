/*
  浮点数类型有： f32 与 f64（默认）
*/

fn main() {
    //
    let sum = 5 + 10;

    let diff = 95.5 - 4.5;

    let pro = 4 * 30;

    let quot = 56.7 / 32.2;
    let trun = -5 / 3; // 结果为-1

    let rema = 43 % 5;

    println!("sum:{}, diff:{}, pro:{}, quot:{}, trun:{}, rema:{}",
        sum, diff, pro, quot, trun, rema);
}