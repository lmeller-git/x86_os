use crate::{print, println};

pub async fn print_stuff() {
    println!("pritn stuff");
    for _ in 0..100 {
        print!("| ");
    }
    println!("done");
}
