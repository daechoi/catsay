use colored::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meow!")]
    message: String,

    #[structopt(short = "d", long = "dead")]
    dead: bool,

    #[structopt(short ="f", long = "file", parse(from_os_str))]
    catfile: Option<std::path::PathBuf>,
}

fn main() {
    let options = Options::from_args();
    let message = options.message;    

    let eye = if options.dead { "x" } else { "o" };
    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog!");
    }

    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path).expect("Could not read file");
            let cat_picture = cat_template.replace("{eye}", eye);
            println!("{}", message.bright_yellow().underline().on_blue());
            println!("{}", cat_picture);
        }
        None => {
            print_cat(&message, eye);
        }
    }

}

fn print_cat(message: &str, eye: &str) {
    println!("{}", message.bright_yellow().underline().on_blue());
    println!("  \\");
    println!("   \\");
    println!("    /\\_/\\");
    println!("   ( {eye} {eye} )",eye=eye.red().bold());
    println!("   ==_Y_==");
}