
struct Ticket {
    title: String,
    description: String,
    status: String,
}

struct Configuration {
    version: f32,
    active: bool
}

// instantiating a struct
// let ticket = Ticket {
//     name: "Payment Processor".into(),
//     description: "Building a payment processor with rust".into(),
//     status: "Open".into()
// }

// Access field on a struct using the DOT Notation
// let name = ticket.name;


// method on struct that describe the struct behvaior 
impl Ticket {
    fn new(title: String, description: String, status: String) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 byte");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }

        if status != "TO-DO" && status != "IN_PROGRESS" && status != "Done" {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }

        Ticket {
            title,
            description,
            status
        }
    }
    fn ticket_title(self) -> String {
        self.title
    }
    fn ticket_description(self) -> String {
        self.description
    }
    fn ticket_status(self) -> String {
        self.status
    }
    fn is_open(self) -> bool {
        self.status == "Open"
    }
}


// two key differences  of method from normal function
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

// The only way to call a static method is by using the function call syntax:
// const default_string: Configuration = Configuration::default();


fn main() {
    println!("Hello, world!");
}

// Encapsulation is the practice of hiding the internal representation of an object. 
// Accessor methods are public methods that allow you to read the value of a private field (or fields) of a struct.
