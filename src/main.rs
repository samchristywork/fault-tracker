use log::info;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

fn main() {
    log4rs::init_file("src/log.yaml", Default::default()).unwrap();

    info!("Hello, World!");

    let files = vec!["data/a", "data/b"];

    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();

    for file in files {
        println!("watching {}", file);

        watcher
            .watch(file.as_ref(), RecursiveMode::NonRecursive)
            .unwrap();
    }

    for res in rx {
        match res {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
