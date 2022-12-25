/*
 * @Author: GKing
 * @Date: 2022-12-17 10:46:24
 * @LastEditors: GKing
 * @LastEditTime: 2022-12-25 11:21:23
 * @Description: 
 * 生成一个 1 到 100 的随机数
 * 提示玩家输入一个猜测数字
 * 猜完之后，程序会提示猜测的大小
 * @TODO: 
 */
use std::io;    // prelude
use rand::Rng;  // trait
use std::cmp::Ordering;

fn main() {
    println!("猜数");
    
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("神秘数字是: {}", secret_number);

    loop {
        println!("猜测一个数");

        // mut 可变变量
        // :: 表示关联函数，类似java中的静态方法
        let mut guess = String::new();
        // 通过io:stdin() 进行逐行读取
        io::stdin().read_line(&mut guess).expect("Can not read line!");

        // string类型 转换 number 类型
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess number is: {}", guess);

        // 比较大小
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }       
        }
    }
    
}
 