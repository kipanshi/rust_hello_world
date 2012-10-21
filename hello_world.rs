use std;
use io::println;
use float::sqrt;

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

fn managed_box() {
    let x: @int = @10; // New box, immutable int pointer
    let y = x; // Copy of a pinter to the same box

    // x and y both refer to the same allocation. When both
    // go out of scope, then the allocation will be freed.

    fn print_box(x: @int, y: @int) {
        println("");
        println("Managed box usage:");
        println("");
        println(fmt!("    Variable x with address %? has value %?", x, *x));
        println(fmt!("    Variable y with address %? has value %?", y, *y));
    }

    print_box(x, y);
}

fn owned_box() {
    /* Owned box has unique memory slot */
    let x = ~10;
    /* Doing this:

        let y = x;

    Gives warning: copying a non-implicitly copyable type
    Instead do it explicitly: */
    let y = copy x;

    let z = *x + *y;
    assert z == 20;

    /*
    let new_x = move x;
    let new_z = *x + *y; // cause error: use of moved variable
    assert new_z == 20;
    */
}

fn borrowed_pointers() {
    struct Point {
        name: ~str,
        x: float, y: float
    };

    let on_the_stack : Point  =  Point {name: ~"on_the_stack", x: 3.0, y: 4.0};
    let shared_box   : @Point = @Point {name: ~"shared_box", x: 5.0, y: 1.0};
    let unique_box   : ~Point = ~Point {name: ~"unique_box", x: 7.0, y: 9.0};

    /* We want to compute distance between two points
    no matter how these points are stored */
    fn compute_distance(p1: &Point, p2: &Point) -> float {
        let x_d = p1.x - p2.x;
        let y_d = p1.y - p2.y;
        sqrt(x_d * x_d + y_d * y_d)
    }

    fn print_distance(p1: &Point, p2: &Point) {
        let distance = compute_distance(p1, p2);
        println("");
        println(
            fmt!("    Distance from \n    %?\n    to\n    %?\n    equals %f",
                 p1, p2, distance)
        );
    }

    println("");
    println("Borrowed pointers example:");
    print_distance(&on_the_stack, shared_box);
    print_distance(shared_box, unique_box);
    
}

fn main() {
    println(fmt!("Factorial of 5 is %?", factorial(5)));
    managed_box();
    owned_box();
    borrowed_pointers();
}