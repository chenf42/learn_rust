fn main() {
    println!("result: {}", plus_one(five()));
    println!("add({}, {}) = {}", five(), six(), add(five(), six()));
}

fn five() -> i32 {
    5
}

fn six() -> i32 {
    let y = {
        let x = 5;
        x + 1
    };
    y
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn plus_one(x: i32) -> i32 {
    x + 1//;  // 若有分号，会报错。因为返回值不能是语句，只能是表达式。
}
