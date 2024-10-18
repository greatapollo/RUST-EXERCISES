use std::io;

fn main() -> ! {
    loop {
        //Print a welcome message to the program
        println!("Temperature converter");
        println!("Convert between celsius and fahrenheit with a lot of ease!");
        //Prompt user for input of choice of conversion
        println!("Please enter 0 to convert from fahrenheit to celcius, or 1 to convert from celcius to fahrenheit.");
        let input_f32: f32 = read_input_line_f32();
        println!("");
        //Work on the choice the user input with conditional statements
        if input_f32 == 0.0 {
            println!("Converting from fahrenheit to celcius");
            // Prompt user for the quatity of degrees to convert
            println!("Please enter number of degrees fahrenheit that you want to convert to degrees celcius.");
            let fahrenheit_input: f32 = read_input_line_f32();
            let celcius: f32 = fahrenheit_to_celcius(fahrenheit_input);
            //Give output of the calculation
            println!(
                "{} degrees fahrenheit is {} degrees celcius.",
                fahrenheit_input, celcius
            );
        } else { // Does the alternative 
            println!("Converting from celcius to fahrenheit.");
            //Prompt user for input
            println!("Please enter celcius degrees that you want to convert.");
            let celcius_input: f32 = read_input_line_f32();
            let fahrenheit: f32 = celcius_to_fahrenheit(celcius_input);
            //Give output for the calculation
            println!(
                "{} degrees celcius is {} fahrenheit.",
                celcius_input, fahrenheit
            );
        }
        println!("");
    }
}

fn read_input_line_f32() -> f32 {
    loop {
        let mut input_string = String::new();
        println!("Enter number:");

         io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read line");

        let _input_string: f32 = match input_string.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };

    }
}

fn celcius_to_fahrenheit(celcius: f32) -> f32 {
    let fahrenheit = (celcius * 1.8) + 32.0;
    return fahrenheit;
}

fn fahrenheit_to_celcius(fahrenheit: f32) -> f32 {
    let celcius = (fahrenheit - 32.0) / 1.8;
    return celcius;
}