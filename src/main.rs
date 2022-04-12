use std::io::stdin;

fn sum_multiples_of_3_or_5(&max: &u16) -> u32 {
    let mut sum: u32 = 0;
    let mut i: u16 = 1;

    while i < max {
        if i % 3 == 0 || i % 5 == 0 {
            sum = sum + u32::from(i); 
        }
        i = i + 1;
    }

    return sum;
}

fn main() {
    println!("Please insert a limit: ");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error input");

    let result:u32;
    let max: u16;

    match input.to_string().trim().parse::<u16>() {
        Ok(i) => {
            max = i;
            result = sum_multiples_of_3_or_5(&i);
        },
        Err(err) => { 
            println!("Error: {}", err);
            return;
        }
    };

    println!("Sum of multiples of 3 or 5 less than {} is {}.", max, result);
}
