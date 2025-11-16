use igpay::Platin;

use clap::Parser;

#[derive(Parser)]
#[command(about = "Lightweight pig latin translator", version, long_about = None)]
struct ClapArgs {
    /// String to be translated
    plain: String,
}

fn main() {
    let clap_args = ClapArgs::parse();

    println!("\n{}\n", &clap_args.plain.to_platin());
}
