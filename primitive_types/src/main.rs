fn main() {
    // 如果没有 ':u32' 编译报错：
    //  cannot infer type for '_'
    //  consider giving `guess` a type
    let guess: u32 = "42".parse().expect("Not a number!");

    // 10 进制
    // 可以用 '_' 作为分隔符，方便读数；
    // 而且 '_' 可以随意添加，不一定要按 3 位数字来分隔。
    let a = 98_222;
    let a2 = 9_8_2_2_2;
    if (a == a2) {
        println!("equals");
    }

    // 16 进制
    let b = 0x2a;

    // 8 进制
    let c = 0o77;

    // 2 进制
    let d = 0b1111_0000;

    // 字节(Byte, u8)
    let e = b'A';

    let f = 2.0;  // 浮点数默认是 f64
    let g: f32 = 3.0;  // 只有显式指定，才会用 f32

    // 数学运算符
    let sum = 5 + 10;
    let diff = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    // bool 类型
    let ok = true;
    let ng = false;

    // 字符类型
    // Rust 中的 char 代表了一个 Unicode 标量值
    let c = 'z';
    let heart_eyed_cat: char = '😻';
    println!("emoji: {}", heart_eyed_cat);

    // 元组
    let tup = (10, 2.0, false);
    // 解构 (destructuring) 元组
    let (x, y, z) = tup;
    println!("value of y is: {}", y);
    // 也可以用点号 (.) 加在元组中的索引来访问元素
    println!("value of z is: {}", tup.2);

    // 数组
    // 显然长度是不可变的
    let array = [1,1,2,3,5,8,13];

    println!("The 3rd element is: {}", array[2]);

    // 试图访问非法索引的值，导致运行时异常：
    // 'index out of bounds'
    println!("Try to access invalid index: {}", array[100]);
}
