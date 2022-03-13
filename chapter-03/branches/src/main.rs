fn main() {
    let x = 1;
    let y = x * 5;
    let a = [1, 2, 3, 4, 5];
    let b = [0; 5];

    for i in (0..y).rev() {
        println!("a: {}, b: {}", a[i], b[i]);
    }
    println!("Done!");
}
