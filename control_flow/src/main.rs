
use std::io;

fn main() {
    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Oops!");
    let n: u32 = n.trim().parse().expect("Parsing error!");

    // if/else if/else
    if n > 15 {
        println!("Too big");
    } else if n < 5 {
        println!("Too small");
    } else {
        println!("Good");
    }

    // 在 let 语句中使用 if/else if/else
    let x = if n > 15 {
        1
    } else if n < 5 {
        -1
    } else {
        0
    };

    println!("x = {}", x);

    // loop 循环，相当于 while true
    let mut i = 0;
    loop {
        println!("{}", i);
        i += 1;
        if i > 5 {
            break;
        }
    }

    // while 循环
    i = 0;
    while i <= 5 {
        println!("{}", i);
        i += 1;
    }

    let a = [1,1,2,3,5,8,13,21];

    // foreach - iterator
    for e in a.iter() {
        println!("{}", e);
    }

    // foreach - range
    for num in 1..5 {
        println!("{}", num);
    }

    // foreach - range rev
    for num in (1..5).rev() {
        println!("{}", num);
    }
}
