use igpay::to_platin;

use clap::Parser;

#[derive(Parser)]
#[command(version)]
struct ClapArgs {
    #[arg()]
    plain: String,
}

fn main() {
    let clap_args = ClapArgs::parse();

    println!("\n{}\n", to_platin(&clap_args.plain));
}
