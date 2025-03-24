// PBERTONI

use std::fmt;

struct Satellite {
    name: String,
    velocity: f64 // miles per second
}

impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is {}", self.name, self.velocity)
    }
}

impl PartialEq for Satellite {
    // Required method
    fn eq(&self, other: &Satellite) -> bool
    {
        self.velocity == other.velocity
    }

    // Provided method
    fn ne(&self, other: &Satellite) -> bool {
        // not (self == other)
        self.velocity != other.velocity
    }
}

impl PartialOrd for Satellite {
    fn partial_cmp(&self, other: &Satellite) -> Option<std::cmp::Ordering> {
        self.velocity.partial_cmp(&other.velocity)
    }
}

/* YOUR CODE GOES HERE */

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    println!("hubble is {}", hubble);

    let iss = Satellite {
        name: String::from("International Space Station"),
        velocity: 2.42
    };
    println!("iss is {}", iss);

    println!("partial_ord {}", hubble > iss);
    println!("partial_eq {}", hubble != iss)
}
