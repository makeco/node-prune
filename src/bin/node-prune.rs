use atty::Stream;
use log::{set_max_level, LevelFilter};
use node_prune::{Config, Prune};
use serde_json::json;
use std::path::Path;
use std::time::Instant;
use structopt::StructOpt;

fn main() {
    set_max_level(LevelFilter::Warn);
    let now = Instant::now();

    let config = Config::from_args();
    if config.verbose {
        set_max_level(LevelFilter::Debug);
    }

    let prune = Prune::new();

    let stats = match prune.run() {
        Ok(s) => s,
        Err(err) => {
            let path = err.path().unwrap_or(Path::new("")).display();
            panic!("failed to access entry {}", path)
        }
    };

    if atty::is(Stream::Stdout) {
        println!();
        println!("\t files total: {}", stats.files_total);
        println!("\t files removed: {}", stats.files_removed);
        println!(
            "\t removed size: {:.2}KB",
            (stats.removed_size as f64) / 1024f64
        );
        println!("\t duration: {}ms", now.elapsed().as_millis());
        println!();
    } else {
        let json_stats = json!(&stats);
        println!("{}", json_stats);
    }
}
