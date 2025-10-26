use std::io;

fn main() {
    println!("\nEmployee Database Management System");

    // Get experience input
    let mut input1 = String::new();
    println!("Have you experienced? (yes/no): ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let experience = input1.trim().to_lowercase();

    // Get age input
    let mut input2 = String::new();
    println!("Enter your age: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let age: u32 = input2.trim().parse().expect("Please enter a valid number");

    // Incentive calculation
    let incentive: u32;

    if experience == "yes" {
        if age >= 50 {
            incentive = 1_560_000;
        } else if age >= 40 {
            incentive = 1_200_000;
        } else if age >= 30 {
            incentive = 840_000;
        } else if age >= 20 {
            incentive = 420_000;
        } else {
            incentive = 140_000; // base incentive for less than 20
        }
    } else {
        incentive = 100_000; // for no experience
    }

    println!("\nEmployee Incentive Report");
    println!("Your incentive: ₦{}", incentive);
}
