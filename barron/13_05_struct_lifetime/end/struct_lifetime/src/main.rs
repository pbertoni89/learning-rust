struct Shuttle<'a> {
    name: &'a str
}

impl<'a, 'b> Shuttle<'a> {
    fn send_transmission_begin(&'a self, msg: &'b str) -> &'a str {
        println!("Transmitting B message: {}", msg);
        self.name
    }

    fn send_transmission_end(&'a self, msg: &'b str) -> &'b str {
        println!("Transmitting E message: {}", msg);
        msg
    }
}

fn main() {
    let vehicle = Shuttle {
        name: "Endeavour"
    };

    for i in 0..1 {
        let sender_b = vehicle.send_transmission_begin("Greetings from orbit!");
        println!("{} sender_b is {}", i, sender_b);
        let sender_e = vehicle.send_transmission_end("Greetings from orbit!");
        println!("{} sender_e is {}", i, sender_b);
    }
}