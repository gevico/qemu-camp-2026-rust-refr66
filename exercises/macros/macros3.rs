mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

#[macro_use]
use macros::my_macro;

fn main() {
    my_macro!();
}
