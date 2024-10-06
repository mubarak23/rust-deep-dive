
struct Ticket {
    title: String,
    description: String,
    status: String,
}

struct Configuration {
    version: u32,
    active: bool
}

/// instantiating a struct
let ticket = Ticket {
    name: "Payment Processor".into(),
    description: "Building a payment processor with rust".into(),
    status: "Open".into()
}

/// Access field on a struct using the DOT Notation
let name = ticket.name;


// method on struct that describe the struct behvaior 
impl Ticket {
    fn is_open(self) -> bool {
        self.status == "Open"
    }
}


/// two key differences  of method from normal function
// method must be define with the impl block
// method may use self as the first params referring to the instance of the struct, the method is call upon.



// if a method does not take SELF as the first param, it called static method

impl Configuration {
    fn default() -> Configuration{
        Configuration {
            version: 0.1,
            active: true
        }
    }
}

/// The only way to call a static method is by using the function call syntax:
let default_string = Configuration::default();


fn main() {
    println!("Hello, world!");
}
