use std::io;

pub fn get_string_input() -> Option<String> {
    let mut buffer = String::new();

    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter the input again. Something went wrong.");
    }

    let input = buffer.trim().to_owned();
    match input.is_empty() {
        false => Some(input),
        true => None,
    }
}

pub fn get_amount_input() -> Option<f64> {
    println!("Enter amount:");

    loop {
        let amount_string = match get_string_input() {
            Some(amount) => amount,
            None => return None,
        };

        if amount_string.is_empty() {
            return None;
        }

        match amount_string.parse::<f64>() {
            Ok(result) => return Some(result),
            Err(_) => println!("Please enter a number:"),
        }
    }
}

pub fn get_quantity_input() -> Option<u32> {
    println!("Enter quantity:");

    loop {
        let quantity_string = match get_string_input() {
            Some(quantity) => quantity,
            None => return None,
        };

        if quantity_string.is_empty() {
            return None;
        }

        match quantity_string.parse::<u32>() {
            Ok(result) => return Some(result),
            Err(_) => println!("Please enter a number:"),
        }
    }
}
