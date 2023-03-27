fn main() {
    // Pass in arguments to cargo run (e.g., `cargo run foobar`).
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    let mut arg_clone = arg.clone();

    inspect(&arg_clone);

    let changed = change(&mut arg_clone);

    if changed {
        println!("I have added the letter s to your argument: {}!", arg_clone);
    } else {
        println!("I have done nothing to your argument: {}!", arg_clone);
    }

    if is_bananas(arg_clone) {
        println!("Might be bananas");
    } else {
        println!("Definitely not bananas");
    }

    println!("The argument you provided was... {}.", arg);
    bedazzle(&mut arg);
    println!(
        "I've taken the liberty of converting your argument into... {}!",
        arg
    );
}

fn inspect(arg: &str) {
    if arg.ends_with("s") {
        println!("This argument definitely ends in with the letter s.");
    } else {
        println!("This argument certainly does not end with the letter s.");
    }
}

fn change(arg: &mut String) -> bool {
    if !arg.ends_with("s") {
        arg.push_str("s");
        return true;
    }
    false
}

fn is_bananas(arg: String) -> bool {
    if arg.starts_with("b") && arg.contains("a") {
        return true;
    }
    false
}

fn bedazzle(materials: &mut String) {
    *materials = "SPARKLES".to_string();
}
