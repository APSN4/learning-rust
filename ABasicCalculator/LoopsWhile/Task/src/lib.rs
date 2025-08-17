// Rewrite the factorial function using a `while` loop.
pub fn factorial(n: u32) -> u32 {
    // The `todo!()` macro is a placeholder that the compiler
    // interprets as "I'll get back to this later", thus
    // suppressing type errors.
    // It panics at runtime.
    if n == 0 {
        return 1
    }
    let mut result: u32 = n;
    let mut value: u32 = n;
    while value > 1 {
        result *= value-1;
        value -= 1;
    }
    result
}
