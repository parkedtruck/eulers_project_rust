use std::io;
use std::io::Write;

fn main() {
    println!(
        "#############################################################
## Welcome to Euler's Project in Rust ##
#############################################################\n
Please select the game you would like to run by typing its associated number:\n
1. Multiples of 3 or 5\n
2. Even Fibonacci Numbers"
    );

    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(u32) => return (),
    };

    select_game(input);

    //even_fibonacci();
}

//feed number into game selection function and print selected game
//ask for more input
//pass input into chosen game(??) --> each game runs input ask fxn

fn multilples_3_5() -> u32 {
    print!("please input an integer to pass to the game: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(u32) => return 0,
    };

    let mut mults = 0;
    for i in 0..input {
        if i % 3 == 0 || i % 5 == 0 {
            mults += i;
        }
    }
    println!(
        "the sum of multiples of 3 and 5 less than {} is: {}",
        input, mults
    );
    return mults;
}

fn even_fibonacci() -> u32 {
    let (mut t, mut i, mut prevprev, mut prev, mut current) = (0, 1, 0, 1, 1);

    print!("please input an integer to pass to the game: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(u32) => return 0,
    };

    while i < input {
        prevprev = prev;
        prev = current;
        current = prevprev + prev;
        i += 1;
        if current % 2 == 0 {
            t += current;
        }
    }
    println!(
        "the sum of even fibonacci numbers below {} is: {}",
        input, t
    );
    return t;
}

fn select_game(choice: u32) -> () {
    match choice {
        1 => {
            multilples_3_5();
        }
        2 => {
            even_fibonacci();
        }
        _ => {
            println!("invalid input");
        }
    }
}
