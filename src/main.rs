use log::info;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};

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

    let mut offsets: HashMap<String, usize> = HashMap::new();

    for res in rx {
        match res {
            Ok(event) => {
                for path in event.paths {
                    let path = path.to_str().unwrap();
                    let offset = match offsets.get(path) {
                        Some(offset) => *offset,
                        None => 0 as usize,
                    };

                    let file = File::open(path).unwrap();
                    let mut reader = BufReader::new(file);
                    reader.seek(SeekFrom::Start(offset as u64)).unwrap();

                    let mut s = String::new();
                    let bytes = reader.read_to_string(&mut s).unwrap();
                    offsets.insert(path.to_string(), offset + bytes);

                    println!("{:?}, {}, {}", path, offset, s)
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
