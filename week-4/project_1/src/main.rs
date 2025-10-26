use std::io;

fn main() {
    println!("\nFind the roots of the equation");

    // Get input for a
    let mut input1 = String::new();
    println!("Enter the value of a:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a: f64 = input1.trim().parse().expect("Please enter a valid number");

    // Get input for b
    let mut input2 = String::new();
    println!("Enter the value of b:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b: f64 = input2.trim().parse().expect("Please enter a valid number");

    // Get input for c
    let mut input3 = String::new();
    println!("Enter the value of c:");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c: f64 = input3.trim().parse().expect("Please enter a valid number");

    // Compute the discriminant
    let discriminant = b.powi(2) - 4.0 * a * c;
    println!("\nThe discriminant is: {}", discriminant);

    // Determine the nature of the roots
    if discriminant > 0.0 {
        // Two distinct real roots
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Your equation has two distinct real roots:");
        println!("x₁ = {:.2}, x₂ = {:.2}", root1, root2);
    } else if discriminant == 0.0 {
        // One real root (repeated)
        let root = -b / (2.0 * a);
        println!("Your equation has one real root:");
        println!("x = {:.2}", root);
    } else {
        // No real roots (complex)
        println!("Your equation has no real roots.");
    }
}
