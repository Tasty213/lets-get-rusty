use std::io;

fn main() {
    println!("Fibonacci calculator");
    loop {
        let input = match get_input(
            "Please enter the index of the fibonaci sequence you wish to calculate",
        ) {
            Ok(num) => num,
            Err(_) => {
                println!("Couldn't read inputed number");
                return;
            }
        };

        let fibonacci_number = fibonaci(input);

        if fibonacci_number.overflow {
            println!("Result overflowed maximum index reached {0}", fibonacci_number.index)
        }
        println!("The {0} fibonacci number is {1}", fibonacci_number.index, fibonacci_number.result);
    }
}

fn get_input<A: std::str::FromStr>(prompt: &str) -> Result<A, A::Err> {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("No data entered");

    input.trim().parse()
}

struct FibonaciResult {
    result: u64,
    index: u64,
    overflow: bool,
}

fn fibonaci(index: u64) -> FibonaciResult {
    if index == 0 {
        return FibonaciResult {
            result: 0,
            index: 0,
            overflow: false,
        };
    }

    let mut previous_number = 0;
    let mut total: u64 = 1;
    let mut overflow = false;
    for i in 1..index {
        (total, overflow) = total.overflowing_add(previous_number);

        if overflow {
            return FibonaciResult {
                result: u64::MAX,
                index: i,
                overflow: overflow,
            };
        }

        previous_number = total - previous_number;
    }

    return FibonaciResult {
        result: total,
        index: index,
        overflow: overflow,
    };
}
