use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    
    println!("\n*---WELL COME TO THE GUESS GAME---*\n");
    println!("Guess The Number Between 1 to 20-->\n");
    let secret_number=rand::thread_rng().gen_range(1..=20);
    loop {
        println!("Please input your guess:");
    
        let mut guess=String::new();
    
        io::stdin().read_line(&mut guess).expect("Failed to read line!!");
    
        let guess:u32=match guess.trim().parse() {
       	    Ok(num) => num,
       	    Err(_) => continue
       	};
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("To big!!"),
            Ordering::Equal => { 
                println!("You win!!");
                break;
            }
        }
    }
}
