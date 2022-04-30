use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;
//Rng defines all me
//thods used by random nbr generators
//io library not included in the prelude

//char literals use single quotes; char string literals
//use double quotes

fn main() {
    eulers_project_one();
}

fn eulers_project_one() -> u32 {
    println!(
        "#############################################################
## Welcome to Game 1 of Project Euler: Multiples of 3 or 5 ##
#############################################################\n"
    );

    print!("Insert a number to receive the sum of all multiples of 3 and 5 below it: ");
    io::stdout().flush().unwrap();
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    let number: u32 = match number.trim().parse() {
        Ok(num) => num,
        Err(u32) => return 0,
    };

    let mut mults = 0;
    for i in 0..number {
        if i % 3 == 0 || i % 5 == 0 {
            mults += i;
        }
    }
    println!("the sume of multiples of 3 and 5 is: {}", mults);
    return mults;
}

/*

fn guessing_game() {
    println!("Guess the number!");

    let secret_nbr = rand::thread_rng().gen_range(1..101);
    //equivalently, range could be (1..=100);
    //to get info on a dependency run cargo doc --open

    //let to declare new variable; mut to make mutable
    //variables are immutable by default
    //::new is an Associated fxn of string type
    //that makes a new, empty string

    loop {
        println!("Please input you guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess) //appends user input to 'guess'
            //references are also immutable by default
            //hence, &mut is required
            .expect("Failed to read line");
        //no error handling = warning
        //this is ONE logical line, broken up for readability
        //io::stdin().read_line(&mut guess).expect("failed");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //match expression for input handling

        println!("You guessed: {}", guess);
        //convert string guess to int for secret_nbr comparison
        //trim eliminates whitespace start and end (user pressing enter to input io)
        //parse parses str into some type of nbr; we tell it u32
        match guess.cmp(&secret_nbr) {
            //match expression made up of arms (patterns to match against)
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
*/
