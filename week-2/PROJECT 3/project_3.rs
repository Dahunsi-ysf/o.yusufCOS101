use std::io;

fn main() {
    // Given values
    let principal: f64 = 510000.0; // Original cost of the TV
    let rate: f64 = 5.0;           // Depreciation rate per annum
    let years: f64 = 3.0;          // Number of years

    // Formula for depreciation:
    // A = P * (1 - R/100)^n
    let amount = principal * (1.0 - (rate / 100.0)).powf(years);

    println!("The value of the TV after {:.0} years is ₦{:.2}", years, amount);
}
