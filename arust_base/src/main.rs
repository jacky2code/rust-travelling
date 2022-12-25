/*
 * @Author: GKing
 * @Date: 2022-12-05 10:28:33
 * @LastEditors: GKing
 * @LastEditTime: 2022-12-17 10:45:13
 * @Description: Rust 基础，初步认识语法
 * @TODO: 
 */

// struct Struct {
//     e: i32
// }

// Rust 程序入口函数，目前无返回值
fn main() {
    /* 1.0 Rust 基础，初步认识语法
    // 使用 let 来声明变量，进行绑定，a 是不可变的
    // 没有指定 a 的类型，编译器会默认根据 a 的值为 a 推断类型：i32，有符号32位整数
    // 语句末尾必须以 ; 结尾
    let a = 10;
    
    // 主动指定 b 的类型为 i32
    let b: i32 = 20;
    
    // 注意2点
    // 1. 可以在数值中带上类型 30i32 表示数值是 30，类型是 i32
    // 2. c 是可变的，mut 是 mutable 缩写
    let mut c = 30i32;
    // 可以在数值和类型之间加 _ ，让可读性更好
    let d = 30_i32;

    // 可以使用一个函数的返回值作为另一个函数的参数
    let e = add(add(a, b), add(c, d));

    // println! 是宏调用，像是函数但是返回的是宏定义的代码块
    // {} 是占位符
    // 字符串使用双引号，单引号给单字符类型 char 使用
    println!("( a + b ) + ( c + d ) = {}", e);
    */
    
    // 2.1 变量绑定与解构

    // 变量的可变性
    // let mut x = 5;
    // println!("The value of x is : {}", x);
    // x = 6;
    // println!("The value of x is : {}", x);

    // 使用下划线忽略未使用的变量
    // let _x = 5;
    // let y = 6;

    // 变量解构
    // let (a, mut b): (bool, bool) = (true, false);
    // // a 不可变，b 可变
    // println!("a = {:?}, b = {:?}", a, b);
    // b = true;
    // assert_eq!(a,b);
    // println!("a = {:?}, b = {:?}", a, b);

    // 解构式赋值
    // let (a, b, c, d, e);
    // (a, b) = (1, 2);
    // [c, .., d, _] = [1, 2, 3, 4, 5];
    // Struct {e, ..} = Struct{e: 5};
    // assert_eq!([1,2,1,4,5],[a,b,c,d,e]);
    
}


/* 1.0 Rust 基础，初步认识语法
// 定义一个函数，输入两个 i32 类型的32位有符号整数，返回它们的和
fn add(i: i32, j: i32) -> i32 {
    // 返回值相加值，可以省略 return
    // 返回值不添加 ; ，这会改变语法导致函数返回(), 而不是i32
    i + j
}
 */