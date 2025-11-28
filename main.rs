use std::io;
use rand::Rng;

fn main() {
    let answer = rand::rng().random_range(1..=10);
    println!("{}", answer);
    let mut runden = 0;
    loop {
        runden += 1;
        if runden == 4 {
            println!("du hast verloren");
            break;
        }
        println!("gib eine zahl zwischen 0-10 ein");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("failed to readline");

        print!("You entered {}", guess);
        let guess = guess.trim();
        let x:i32 = guess.parse().unwrap();
        println!("1");
        if x == answer {
            println!("du hast gewonen");
            break;
        }

        else {
            println!("versuche es nochmal")
        }
    }

}
