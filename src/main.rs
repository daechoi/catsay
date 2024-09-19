fn main() {
    let message = std::env::args()
        .nth(1)
        .expect("Missing message.  Usage: catsay <message>");
    println!("{}", message);
    println!("  \\");
    println!("   \\");
    println!("    /\\_/\\");
    println!("   ( o o )");
    println!("   ==_Y_==");
}
