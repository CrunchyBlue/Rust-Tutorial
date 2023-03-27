use std::fmt;

trait Bite {
    fn bite(self: &mut Self);
}

struct Grapes {
    amount_left: i32,
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.amount_left -= 1;
    }
}

impl fmt::Debug for Grapes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Grapes")
            .field("remaining_grapes", &self.amount_left)
            .finish()
    }
}

fn main() {
    let mut carrot = Carrot {
        percent_left: 100.0,
    };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    let mut grapes = Grapes { amount_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    // Challenge: Uncomment the code below. Create a generic `bunny_nibbles`
    // function that:
    // - takes a mutable reference to any type that implements Bite
    // - calls `.bite()` several times
    // Hint: Define the generic type between the function name and open paren:
    //       fn function_name<T: Bite>(...)

    nibble(&mut carrot);
    println!("{:?}", carrot);
}

struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        self.percent_left *= 0.8;
    }
}

impl fmt::Debug for Carrot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Carrot")
            .field("percent_left", &self.percent_left)
            .finish()
    }
}

fn nibble<T: Bite>(object: &mut T) {
    for _ in 0..7 {
        object.bite();
    }
}
