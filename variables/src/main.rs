fn main() {
    let mut x = 5;
    println!("x = {}", x);

    // If there are no 'mut' at the definiton line,
    // this line can't be compiled.
    x = 6;
    println!("x = {}", x);

    // Shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("y = {}", y);
}
