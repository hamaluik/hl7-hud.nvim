use std::io::Read;

use chrono::{DateTime, Local};

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

fn timestamp(args: Vec<String>) {
    let timestamp = match hl7_parser::timestamps::parse_timestamp(args[2].as_str()) {
        Ok(timestamp) => timestamp,
        Err(e) => {
            eprintln!("Error parsing timestamp: {e:#}");
            std::process::exit(5);
        }
    };

    let timestamp: DateTime<Local> = match timestamp.try_into() {
        Ok(timestamp) => timestamp,
        Err(e) => {
            eprintln!("Error converting timestamp: {e:#}");
            std::process::exit(6);
        }
    };

    println!("{}", timestamp.to_rfc2822());
}

fn timestamp_cursor(args: Vec<String>) {
    let Ok(cursor_position) = args[2].parse::<usize>() else {
        eprintln!("Usage: {} tc <cursor_position>", args[0]);
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

    let value = if let Some((_, sub_component)) = location.sub_component {
        sub_component.raw_value()
    }
    else if let Some((_, component)) = location.component {
        component.raw_value()
    }
    else if let Some((_, repeat)) = location.repeat {
        repeat.raw_value()
    }
    else if let Some((_, field)) = location.field {
        field.raw_value()
    }
    else if let Some((_, _, segment)) = location.segment {
        segment.raw_value()
    }
    else {
        eprintln!("Error locating cursor");
        std::process::exit(7);
    };

    let timestamp = match hl7_parser::timestamps::parse_timestamp(value) {
        Ok(timestamp) => timestamp,
        Err(e) => {
            eprintln!("Error parsing timestamp: {e:#}");
            std::process::exit(5);
        }
    };

    let timestamp: DateTime<Local> = match timestamp.try_into() {
        Ok(timestamp) => timestamp,
        Err(e) => {
            eprintln!("Error converting timestamp: {e:#}");
            std::process::exit(6);
        }
    };

    println!("{}", timestamp.to_rfc2822());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} p/q/t/tc <cursor_position>/<query>/<timestamp>/<timestamp cursor_position>", args[0]);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "p" => position(args),
        "q" => query(args),
        "t" => timestamp(args),
        "tc" => timestamp_cursor(args),
        _ => {
            eprintln!("Usage: {} p/q/t/tc <cursor_position>/<query>/<timestamp>/<timestamp cursor_position>", args[0]);
            std::process::exit(1);
        }
    }

}
