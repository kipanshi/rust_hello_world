use std;
use io::println;

fn factorial(x: uint) -> uint {
    if (x <= 1u) {
        1u
    } else {
        x * factorial(x - 1u)
    }
}

fn print_current_time() {
    let current_time = std::time::get_time();
    println(fmt!("Current timestamp: %?", current_time));
}

fn use_managed_box() {
    let x: @int = @10; // New box, immutable int pointer
    let y = x; // Copy of a pinter to the same box

    // x and y both refer to the same allocation. When both
    // go out of scope, then the allocation will be freed.

    fn print_box(x: @int, y: @int) {
        println(fmt!("%? = %?", x, *x));
        println(fmt!("%? = %?", y, *y));
    }

    print_box(x, y);
}

fn use_owned_box() {
    /** Owned box has unique memory slot **/
    let x = ~10;
    /** Doing this:

        let y = x;

    Gives warning: copying a non-implicitly copyable type
    Instead do it explicitly: **/
    let y = copy x;

    let z = *x + *y;
    assert z == 20;
    println(z.to_str());

    let new_x = move x;
    let z = *new_x + *y; // cause error: use of moved variable
}

fn main() {
    printl(fmt!("Factorial of 5 is %?", factorial(5)))
    use_managed_box();
    use_owned_box()
}