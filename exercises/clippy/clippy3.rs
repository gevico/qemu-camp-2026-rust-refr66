#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // Fix 1: Just remove the if block entirely.
    // Since my_option is None, unwrapping it is always a bug.
    // In a real program, you'd use 'if let Some(x) = ...'

    let my_arr = &[
        -1, -2, -3, // Fix 2: Added the missing comma
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // Fix 3: Don't try to assign the result of resize() or clear() to a variable.
    // They return (), not the modified vector.
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Fix 4: Use std::mem::swap to swap values safely
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
