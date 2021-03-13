use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
struct Arguments {
    #[structopt(long)]
    log_in: bool,
}

fn main() {
    let arguments = Arguments::from_args();
    println!("{:?}", arguments);
}
