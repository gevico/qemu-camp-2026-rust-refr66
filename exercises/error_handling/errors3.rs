use std::num::ParseIntError;

// 1. Change main to return a Result so we can use the `?` operator.
fn main() -> Result<(), ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // This now works because main returns a Result
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }

    // 2. Return Ok(()) at the end of main
    Ok(())
}

// 3. Change Result<()> to Result<i32> because we want to return the cost.
pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // 4. Use `?` here to get the i32 out of the Result returned by parse()
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
