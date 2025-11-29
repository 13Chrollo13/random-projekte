use std::io;
use rand::Rng;

fn main() {
    let mut  min = 0;
    let mut  max = 10;
    let mut runden = 0;
    let mut rundenanzahl = 3;
    let mut nettorunden = 3;

    println!("wenn du in den experten modus willst tippe 1 ansonsten tippe 2 ein");
    let mut z = String::new();
    io::stdin().read_line(&mut z).expect("failed to readline");
    let experte = z.trim();
    let experte:i32 = experte.parse().unwrap();


    if experte == 1 {
        println!("gebe das minimium ein");
        let mut r = String::new();
        io::stdin().read_line(&mut r).expect("failed to readline");
        let u = r.trim();
        let u:i32 = u.parse().unwrap();
        min = u;


        println!("gebe das maximium ein");
        let mut j = String::new();
        io::stdin().read_line(&mut j).expect("failed to readline");
        let p = j.trim();
        let p:i32 = p.parse().unwrap();
        max = p;


        println!("gebe die runden anzahl an");
        let mut x = String::new();
        io::stdin().read_line(&mut x).expect("failed to readline");
        let b = x.trim();
        let b:i32 = b.parse().unwrap();
        nettorunden = b;
        rundenanzahl = b;
    println!("du hast {} runden", nettorunden);
    }

    rundenanzahl += 1;

    let answer = rand::rng().random_range(min..=max);

    //modus abfrage
    println!("tippe für den easymode 1 und den hardcore mode 2");
    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("failed to readline");
    let mode = y.trim();
    let mode:i32 = mode.parse().unwrap();
    println!("du hast {} runden", nettorunden);
    println!("gib eine zahl zwischen {}-{} ein", min, max);



    loop {
        runden += 1;
        if runden == rundenanzahl {
            println!("du hast verloren");
            break;
        }

        //user guess
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to readline");
        let guess = guess.trim();
        let x:i32 = guess.parse().unwrap();
        println!("1");

        if x == answer {
            println!("du hast gewonen");
            break;
        }

        else {

            println!("versuche es nochmal");
            if x < answer && mode == 1 {
              println!("{} ist kleiner als die lösung ",guess);
            }
            else if x > answer && mode == 1 {
                println!("{} ist größer als die lösung", guess);
            }
            else {
            }
        }
    }

}
