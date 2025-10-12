fn main() {
    let principal: f64 = 520_000_000.0;
    let rate: f64 = 10.0;
    let time: f64 = 5.0;

    // Calculate amount
    let amount = principal * (1.0 + rate / 100.0).powf(time);

    // Calculate compound interest
    let compound_interest = amount - principal;

    println!("Total Amount (A): ₦{:.2}", amount);
    println!("Compound Interest (CI): ₦{:.2}", compound_interest);
}
