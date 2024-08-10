use std::io;

fn main() {
    println!("Celsius to Farenheit converter");
    loop {
        println!("Press 1 to convert Celsius -> Farenheit");
        println!("Press 2 to convert Farenheit -> Celsius");

        let celsius_or_farenheit = match get_int_input("Please enter your choice: ") {
            Ok(num) => num,
            Err(_) => {
                println!("Couldn't convert inputed number to int");
                continue
            }
        };

        let input: f32 = match get_int_input("Please enter the number you wish to convert: ") {
            Ok(num) => num,
            Err(_) => {
                println!("Couldn't convert inputed number to float");
                continue
            }
        };

        match celsius_or_farenheit {
            1 => {
                let farenheit = (input * 1.8) + 32.0;
                println!("{input} Celsius is {farenheit} in Farenheit")
            }
            2 => {
                let celsius = (input - 32.0) / 1.8;
                println!("{input} Farenheit is {celsius} in Celsius")
            }
            _ => println!("Unknown input"),
        }
    }
}

fn get_int_input<A: std::str::FromStr>(prompt: &str) -> Result<A, A::Err> {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("No data entered");

    input.trim().parse()
}
