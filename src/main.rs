use igpay::to_platin;

use clap::Parser;

#[derive(Parser)]
#[command(about = "Lightweight pig latin translator", version, long_about = None)]
struct ClapArgs {
    /// String to be translated
    plain: String,
}

fn main() {
    let clap_args = ClapArgs::parse();

    println!("\n{}\n", to_platin(&clap_args.plain));
}
