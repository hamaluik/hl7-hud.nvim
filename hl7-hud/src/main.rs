use std::io::Read;

fn position(args: Vec<String>) {
    let Ok(cursor_position) = args[2].parse::<usize>() else {
        eprintln!("Usage: {} p <cursor_position>", args[0]);
        std::process::exit(1);
    };

    // read the hl7 message from stdin until EOF
    let mut message = String::new();
    if let Err(e) = std::io::stdin().read_to_string(&mut message) {
        eprintln!("Error reading from stdin: {e:#}");
        std::process::exit(2);
    }

    // parse the message
    let message = match hl7_parser::parse_message(message.as_str()) {
        Ok(message) => message,
        Err(e) => {
            eprintln!("Error parsing HL7 message: {e:#}");
            std::process::exit(3);
        }
    };
    
    // and locate the cursor
    let Some(location) = message.locate_cursor(cursor_position) else {
        println!("?");
        return;
    };

    println!("{location}");
}

fn query(args: Vec<String>) {
    // read the hl7 message from stdin until EOF
    let mut message = String::new();
    if let Err(e) = std::io::stdin().read_to_string(&mut message) {
        eprintln!("Error reading from stdin: {e:#}");
        std::process::exit(2);
    }

    // parse the message
    let message = match hl7_parser::parse_message(message.as_str()) {
        Ok(message) => message,
        Err(e) => {
            eprintln!("Error parsing HL7 message: {e:#}");
            std::process::exit(3);
        }
    };

    // and locate the query
    let Some(location) = message.query(args[2].to_ascii_uppercase()) else
    {
        eprintln!("Query not found");
        std::process::exit(4);
    };

    let range = location.range();
    println!("{}", range.start);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} p/q <cursor_position>/<query>", args[0]);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "p" => position(args),
        "q" => query(args),
        _ => {
            eprintln!("Usage: {} p/q <cursor_position>/<query>", args[0]);
            std::process::exit(1);
        }
    }

}
