//! The CLI

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// A space-delineated list of (x,y) pairs to plot
    #[structopt(short, long, default_value = "(4,5) (1,2) (7,8)")]
    input: String,
}

fn main() {
    let opt = Opt::from_args();
    let input = opt.input;
    let plot = graphcloset::plot(&input);
    println!("Plotting: {}", input);
    println!("{}", plot);
}
