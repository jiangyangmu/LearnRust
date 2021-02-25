fn fun() {
        println!("standard function.");
}
fn fun_with_return() -> i32 {
        100
}
fn fun_with_args(a : i32, b : i32) -> i32 {
        a + b
}
fn fun_with_recursion(i : i32) -> i32 {
        if i <= 0 {
                0
        } else {
                i + fun_with_recursion(i - 1)
        }
}

fn main() {
        print!("fun() = "); fun();
        println!("fun_with_return() = {}", fun_with_return());
        println!("fun_with_args({}, {}) = {}", 2, 3, fun_with_args(2, 3));
        println!("fun_with_recursion({}) = {}", 10, fun_with_recursion(10));
}