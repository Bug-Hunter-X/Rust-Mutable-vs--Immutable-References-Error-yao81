fn main() {
    let mut x = 5;
    let y = &mut x; // Mutable reference

    *y += 1;
    println!("x = {}", x); // Output: x = 6

    // Correctly using immutable reference without modification
    let z = &x;
    println!("x = {}", *z); // Output: x = 6
} 