mod test;

use std::io;

fn main() {
    loop {
        println!();
        println!("Bitte gib den Schlüssel ein.");
        let mut message = String::new();

        io::stdin()
            .read_line(&mut message)
            .expect("Leider ist bei der Eingabe was schiefgegangen");

        let message: i128 = match message.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Bitte gib eine Primzahl ein ein.");
                continue;
            }
        };
        println!("Deine Primzahl ist {}", message);
        prim_tester(message);
        println!("Deine Zahl ist eine : {}", prim_tester(message))
    }
}

fn prim_tester(prim: i128) -> bool {
    let mut k: i128 = 2;
    while prim > k {
        if prim % k == 0 {
            return false;
        }
        k = k + 1
    }
    return true;
}
