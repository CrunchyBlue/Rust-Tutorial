fn main() {
    // Pass in arguments to cargo run (e.g., `cargo run sum`).
    let args: Vec<String> = std::env::args().skip(1).collect();

    for arg in args {
        if arg == "sum" {
            sum();
        } else if arg == "double" {
            double();
        } else {
            count(arg);
        }
    }
}

fn sum() {
    let mut sum = 0;

    for i in 7..=23 {
        sum += i;
    }

    println!(
        "The sum of all numbers between 7 and 23 inclusive is {}",
        sum
    );
}

fn double() {
    let mut count = 0;
    let mut x = 1;

    while x < 500 {
        x *= 2;
        count += 1;
    }

    println!(
        "You can double x {} times until x is larger than 500",
        count
    );
}

fn count(arg: String) {
    let mut count = 0;

    loop {
        if count == 8 {
            break;
        }
        print!("{} ", arg);
        count += 1;
    }

    println!(); // This will output just a newline at the end for cleanliness.
}
