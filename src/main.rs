<<<<<<< HEAD
fn main() {
    let message = std::env::args()
        .nth(1)
        .expect("Missing message.  Usage: catsay <message>");
    println!("{}", message);
    println!("  \\");
    println!("   \\");
    println!("    /\\_/\\");
    println!("   ( o o )");
=======
extern crate structopt;
extern crate colored;

use colored::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meow!")]
    message: String,

    #[structopt(short = "d", long = "dead")]
    dead: bool,
}

fn main() {
    let options = Options::from_args();
    let message = options.message;    

    let eye = if options.dead { "x" } else { "o" };
    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog!");
    }

    println!("{}", message.bright_yellow().underline());
    println!("  \\");
    println!("   \\");
    println!("    /\\_/\\");
    println!("   ( {eye} {eye} )",eye=eye.red().bold());
>>>>>>> 4b1af52 (adds colors)
    println!("   ==_Y_==");
}
