use colored::*;
use structopt::StructOpt;
use failure::ResultExt;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meow!")]
    message: String,

    #[structopt(short = "d", long = "dead")]
    dead: bool,

    #[structopt(short ="f", long = "file", parse(from_os_str))]
    catfile: Option<std::path::PathBuf>,
}

fn main() -> Result<(), failure::Error> {
    let options = Options::from_args();
    let message = options.message;    

    let eye = if options.dead { "x" } else { "o" };
    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog!");
    }

    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path).with_context(|_| format!("could not read file {:?}", path))?;
            let cat_picture = cat_template.replace("{eye}", eye);
            println!("{}", message.bright_yellow().underline().on_blue());
            println!("{}", cat_picture);
            Ok(())
        }
        None => {
            print_cat(&message, eye);
            Ok(())
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