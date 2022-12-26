# Rust Travelling



## 第一章 认识 Rust



### 1. 环境及工具

#### 1.1 安装 Rust 环境

- 安装 Rust 

rustup 是 Rust 安装程序，也是版本管理程序

在 macOS 上安装 rustup

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

- 安装成功

```bash
Rust is installed now. Great!
```

- 安装 C 语言编译器（非必需）

macOS

```bash
xcode-select --install
```

- 卸载 Rust

```bash
rustup self uninstall
```

- 检查版本

```bash
cargo -V
cargo 1.65.0 (4bc8f24d3 2022-10-20)
rustc -V
rustc 1.65.0 (897e37553 2022-11-02)
```

- 本地参考文档

  安装 Rust 后同时在本地安装了一个文档服务，运行 rustup doc 让浏览器打开文档，可以参考API



#### 1.2 VSCode

- 安装 VSCode 的 Rust 插件
  - `Rust-analyzer`
  - `Even Better TOML`，支持 .toml 文件完整特性
  - `Error Lens`, 更好的获得错误展示
  - `One Dark Pro`, 非常好看的 VSCode 主题
  - `CodeLLDB`, Debugger 程序

#### 1.3 认识 Cargo

`cargo` 提供了一系列的工具，从项目的建立、构建到测试、运行直至部署，为 Rust 项目的管理提供尽可能完整的手段。

- 使用 cargo 创建 world_hello 项目

  ```bash
  cargo new world_hello
  ```

- 运行项目

  - debug 模式

    ```bash
    cargo run
    ```

    实际上运行了两个命令分别为：

    ```bash
    cargo build
    ```

    ```bash
    ./target/debug/world_hello
    ```

  - 更高性能 release 模式

    ```bash
    cargo run --release
    ```

- 更快验证代码

  ```bash
  cargo check
  ```

- Cargo.toml, Cargo.lock

  `Cargo.toml` 和 `Cargo.lock` 是 `cargo` 的核心文件，它的所有活动均基于此二者。

  - `Cargo.toml` 是 `cargo` 特有的**项目数据描述文件**。它存储了项目的所有元配置信息，如果 Rust 开发者希望 Rust 项目能够按照期望的方式进行构建、测试和运行，那么，必须按照合理的方式构建 `Cargo.toml`。
  - `Cargo.lock` 文件是 `cargo` 工具根据同一项目的 `toml` 文件生成的**项目依赖详细清单**，因此我们一般不用修改它。

- Cargo.toml 中的配置项

  - package

    ```toml
    [package]
    name = "world_hello"
    version = "0.1.0"
    edition = "2021"
    ```

    `name` 字段定义了项目名称，`version` 字段定义当前版本，新项目默认是 `0.1.0`，`edition` 字段定义了我们使用的 Rust 大版本。

  - dependencies 依赖项

    ```toml
    [dependencies]
    rand = "0.3"
    hammer = { version = "0.5.0"}
    color = { git = "https://github.com/bjz/color-rs" }
    geometry = { path = "crates/geometry" }
    ```



#### 1.4 依赖下载源

- 新增镜像地址

  **首先是在 `crates.io` 之外添加新的注册服务**，在 `$HOME/.cargo/config.toml` （如果文件不存在则手动创建一个）中添加以下内容：

  ```toml
  [registries]
  ustc = { index = "https://mirrors.ustc.edu.cn/crates.io-index/" }
  ```

  这种方式只会新增一个新的镜像地址，因此在引入依赖的时候，需要指定该地址，例如在项目中引入 `time` 包，你需要在 `Cargo.toml` 中使用以下方式引入:

  ```toml
  [dependencies]
  time = {  registry = "ustc" }
  ```

  **在重新配置后，初次构建可能要较久的时间**，因为要下载更新 `ustc` 注册服务的索引文件，还挺大的...

- 覆盖默认镜像地址

  在 `$HOME/.cargo/config.toml` 添加以下内容：

  ```toml
  [source.crates-io]
  replace-with = 'ustc'
  
  [source.ustc]
  registry = "git://mirrors.ustc.edu.cn/crates.io-index"
  ```

  首先，创建一个新的镜像源 `[source.ustc]`，然后将默认的 `crates-io` 替换成新的镜像源: `replace-with = 'ustc'`。



### 2. Rust 基础入门

会接触到新的概念如：

- 所有权、借用、生命周期
- 宏编程
- 模式匹配

本章介绍基础语法、数据类型、项目结构等

通过代码浏览 Rust 语法

```rust
/*
 * @Author: GKing
 * @Date: 2022-12-05 10:28:33
 * @LastEditors: GKing
 * @LastEditTime: 2022-12-05 10:52:15
 * @Description: Rust 基础，出入认识语法
 * @TODO: 
 */
// Rust 程序入口函数，目前无返回值
fn main() {
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
}

// 定义一个函数，输入两个 i32 类型的32位有符号整数，返回它们的和
fn add(i: i32, j: i32) -> i32 {
    // 返回值相加值，可以省略 return
    // 返回值不添加 ; ，这会改变语法导致函数返回(), 而不是i32
    i + j
}
```



#### 2.1 变量绑定与解构

声明可变变量为编程提供灵活性，声明不可变变量为编程提供安全性，提高运行性能。

- 变量命名

  在命名方面，和其它语言没有区别，不过当给变量命名时，需要遵循 [Rust 命名规范](https://course.rs/practice/naming.html)。

  > Rust 语言有一些**关键字**（*keywords*），和其他语言一样，这些关键字都是被保留给 Rust 语言使用的，因此，它们不能被用作变量或函数的名称。在 [附录 A](https://course.rs/appendix/keywords.html) 中可找到关键字列表。

- 变量绑定

  其他语言中的赋值，Rust 中，给这个赋值的过程叫做：变量绑定；涉及到 Rust 最核心的原则 -- 所有权。

  绑定就是把这个对象绑定给一个变量，让这个变量称为这个对象的主人。

- 变量可变性

  为了安全和性能，变量默认是不可变的，可以使用 mut 关键字声明可变变量

  如错误代码：

  ```rust
  fn main() {
      let x = 5;
      println!("The value of x is: {}", x);
      x = 6;
      println!("The value of x is: {}", x);
  }
  ```

  运行后抛出错误：

  ```bash
  error[E0384]: cannot assign twice to immutable variable `x`
    --> src/main.rs:39:5
     |
  37 |     let x = 5;
     |         -
     |         |
     |         first assignment to `x`
     |         help: consider making this binding mutable: `mut x`
  38 |     println!("The value of x is : {}", x);
  39 |     x = 6;
     |     ^^^^^ cannot assign twice to immutable variable
  
  For more information about this error, try `rustc --explain E0384`.
  error: could not compile `rust_base` due to previous error
  ```

  具体的错误原因是 `cannot assign twice to immutable variable x`（无法对不可变的变量进行重复赋值），因为我们想为不可变的 `x` 变量再次赋值。

  为了让变量声明可变，可以更改为：

  ```rust
  fn main() {
      let mut x = 5;
      println!("The value of x is: {}", x);
      x = 6;
      println!("The value of x is: {}", x);
  }
  ```

- 使用下划线开头忽略未使用的变量

  Rust 会警告未使用的变量，可以使用 "_" 下划线避免警告，如：

  ```rust
  fn main() {
      let _x = 5;
      let y = 10;
  }
  ```

  警告 y 变量未使用，而不警告 _x:

  ```bash
  warning: unused variable: `y`
    --> src/main.rs:44:9
     |
  44 |     let y = 6;
     |         ^ help: if this is intentional, prefix it with an underscore: `_y`
     |
     = note: `#[warn(unused_variables)]` on by default
  
  warning: `rust_base` (bin "rust_base") generated 1 warning
  ```

  

- 变量解构

  `et` 表达式不仅仅用于变量的绑定，还能进行复杂变量的解构：从一个相对复杂的变量中，匹配出该变量的一部分内容：

  ```rust
  let (a, mut b): (bool, bool) = (true, false);
  // a 不可变，b 可变
  println!("a = {:?}, b = {:?}", a, b);
  b = true;
  assert_eq!(a,b);
  println!("a = {:?}, b = {:?}", a, b);
  ```

  - 解构式赋值
  
    在 [Rust 1.59](https://course.rs/appendix/rust-versions/1.59.html) 版本后，我们可以在赋值语句的左式中使用元组、切片和结构体模式了。
  



## 第二章 猜数游戏

### 2.1 一次猜测

提示玩家输入一个猜测数字

* mut 关键字：可变变量；

* :: 表示关联函数，类似 Java 中的静态方法

  ```rust
  use std::io;
  
  fn main() {
      println!("猜数");
      println!("猜测一个数");
  
      // mut 可变变量
      // :: 表示关联函数，类似java中的静态方法
      let mut guess = String::new();
      // 通过io:stdin() 进行逐行读取
      io::stdin().read_line(&mut guess).expect("Can not read line!");
  
      println!("Your guess number is: {}", guess);
  }
  ```

### 2.2 生成神秘数字

生成一个神秘数字，用来和用户输入的数字进行比较

- 生成随机数的库：Rand

- 管理依赖：在 Cargo.toml 中管理依赖，如：

  ```toml
  [dependencies]
  rand = "^0.7.0"
  ```

- 代码
  ```rust
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
  ```




## 第三章 通用编程概念

### 3.1 变量与可变性

- 声明变量用 let 关键字

- 默认情况下，变量是不可变的（Immutable）

  - 例子 variables

    ```rust
    let x = 5;
    // x = 10; 不能再绑定其他值
    ```

- 声明变量时，在变量前面加 mut，就可以使变量可变

  ```rust
  let mut y = 10;
  y = 11;
  ```

- 变量与常量

  常量在绑定值以后也是不可变的，但与不可变的变量有很多区别：

  - 不可以使用 mut 关键字，常量是永远不可变的。
  - 声明常量使用 const 关键字，它的类型必须被标注。
  - 常量可以在任何作用域声明，包括全局作用域。
  - 常量只能绑定到常量表达式，无法绑定到函数调用结果或只能运行时才能计算出的值。

  在程序运行期间，常量在其声明的作用域内一直有效。

  常量声明使用大写字母，下划线分割：MAX_POINT

- Shadowing（隐藏）

  可以使用相同的名字声明新的变量，新的变量就会 shadow 之前声明的同名变量。

  - 在后续的代码中这个变量名代表的就是新的变量

  ```rust
  let x = 5;
  // 使用相同变量名 shadow 之前的变量
  let x = x + 1;
  ```

  shadow 和把变量标记为 mut  是不一样的：

  - 如果不使用 mut 关键字，给非 mut 变量赋值会导致编译错误。
  - 使用 let 声明的同名新变量，也是不可变的。
  - 使用 let 声明的同名新变量，它的类型可以与之前的不同。

  ```rust
  let spaces = "    ";
  let spaces = spaces.len();
  ```

  

### 3.2 数据类型

- 标量和复合类型

  Rust 是静态编译语言，在编译时必须知道所有变量的类型

  - 基于使用的值，编译器通常能够推断出它的具体类型

  - 但如果可能的类型比较多（例如把 String 转为整数的 parse 方法），就必须添加类型的标注，否则编译会报错。

    ```rust
    let x: u32 = "43".parse().expect("Not a number!");	// 43 被转成整数时，可能是u32或者i32都可以，所以 x 必须标注类型。
    ```

     

- 标量类型

  一个标量类型代表一个单个的值，Rust 有四个主要的标量类型：

  - 整数类型

    - 整数类型没有小数部分

    - 例如 u32 就是一个无符号的整数类型，占据32位空间
    - 无符号整数类型以 u 开头
    - 有符号整数类型以 i 开头
    - 有符号整数范围：-(2的N次方 - 1) 到 2的N次方 -1
    - 无符号整数范围：0 到 2的N次方 - 1

  - 浮点类型

    Rust 有两种基础的浮点类型，也就是含有小数部分的类型

    - f32 32位，单精度
    - f64 64位，双精度

    Rust 的浮点类型使用了 IEEE-754 标准来表述

    f64是默认类型，因为在现代 CPU上 f64和f32的速度差不多，而且精度更高。

    数值操作

  - 布尔类型

    - Rust 的布尔类型也有两个值：true 和 false
    - 一个字节大小
    - 符号是bool

  - 字符类型

    - Rust 语言中 char 类型被用来描述语言中最基础的单个字符
    - 字符类型的字面值使用单引号
    - 占用4字节大小
  
- 复合类型

  复合类型可以将多个值放在一个类型里。

  Rust 提供了两种基础的复合类型：元祖（Tuple）、数组。

  - Tuple

    可以将多个类型的值放在一个类型里面
  
    Tuple 的长度是固定的：一旦声明就无法改变 
  
    - 创建 Tuple

      - 在小括号里，将值用逗号分开

      - 每个位置对应一个类型，各元素类型不必相同
      
        ```rust
        let tup: (i32, f64, u8) = (500, 6,4, 3);
        ```
      
        
    
    - 获取 Tuple 的元素值
    
      - 可以使用模式匹配来解构一个元祖获取元素值
    
        ```rust
        let (x, y, z) = tup;
        ```
    
    - 访问元素
    
      - 使用点标记法，后接元素的索引号
    
        ```rust
        println!("{},{},{}",tup.0, tup.1, tup.2);
        ```
  
- 数组
  
  数组也可以将多个值放在一个类型里面
  
  数组中每个元素的类型必须相同
  
  数组的长度也是固定的
  
  - 声明一个数组
  
    ```rust
    let a = [1, 2, 3];
    ```
  
  - 数组的用处
  
    如果想让你的数组存放在 stack （栈）上而不是heap （堆）上，或者想保证有固定数量的元素，这时使用数组更有好处。
  
    数组没有vector灵活
  
  - 数组的类型：[类型；长度]
  
    ```rust
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    ```
  
  - 另外一种声明数组的方法
  
    如果每个元素值都是相同的；
  
    例如，let a = [3; 5] 相当于 let a = [3, 3, 3, 3, 3];
  
  - 访问数组元素    



### 3.3 函数

声明函数使用 fn 关键字

依照惯例，针对函数和变量名，Rust 使用 snake case 命名规范。

所有字母小写，单词之间使用下划线分开

- 函数参数

  ```rust
  fn main() {
  	// arguments 实参  
    another_function(5, 6);
  }
  // parameters 形参
  fn another_function(x: i32, y: i32) {
    println!("{}, {}", x, y)
  }
  ```
  

### 3.4 注释

### 3.5 控制流程
- if 表达式
  - if 表达式允许您根据条件来执行不同的代码分支
    - 这个条件必须是 bool 类型
  - if 表达式中，与条件相关联的代码快就叫做分支（arm）
  ```rust
  fn main() {
    let number = 3;
    if number < 5 {
      println!("condition was true");
    } else {
      println!("condition was false");
    }
  }
  ```
- Rust 的循环
Rust 提供了3中循环，loop，while 和 for
  - loop 循环
    - loop 关键字告诉 Rust 反复的执行一块代码，直到你喊停
    - 可以在 loop 循环中使用 break 关键字来告诉程序何时停止循环
    ```rust
    fn main() {
      let mut counter = 0;
      let result = loop {
        counter += 1;
        if counter == 10 {
          break counter * 2;
        }
      };
      println!("The result is : {}", result);)
    }
    ```
  - while 条件循环
  另外一种常见的循环模式是每次执行循环体之前都判断一次条件。 
  while 条件循环位这种模式而生
  ```rust
  fn main() {
    let mut number = 3;
    while number != 0 {
      println!("{}", number);
      number = number - 1;
    }
    println!(LIFTOFF!!!);
  }
  ```
  - 使用 for 循环遍历集合
  ```rust
  fn main() {
    let a = [1, 2, 3];
    for element in a.iter() {
      println!("The value is : {}", element);
    }
  }
  ```


