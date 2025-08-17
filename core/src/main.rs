use clap::Parser;

#[derive(clap::Parser)]
#[command(name = "iotame")]
enum IotameCli {
    Start(StartArgs)
}

#[derive(clap::Args)]
#[command(version, about)]
struct StartArgs {

}

fn main() {
    match IotameCli::parse() {
        IotameCli::Start(start_args) => {
            println!("Starting iotame...");
        }
    }
}
