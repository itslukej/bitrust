mod tracker;
use structopt::StructOpt;
use tracker::Tracker;

#[derive(Debug, StructOpt)]
struct Config {
    #[structopt(default_value = "127.0.0.1")]
    bind: String,

    #[structopt(default_value = "6969")]
    http_port: u16
}

#[tokio::main]
async fn main() {
    let config = Config::from_args();

    let tracker = Tracker::new();

    let peers = tracker.get_peers([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]).await;

    println!("{:?}", peers);
}