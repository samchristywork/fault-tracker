use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};

pub fn listen(files: Vec<&str>) {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();

    for file in files {
        println!("watching {}", file);

        watcher
            .watch(file.as_ref(), RecursiveMode::NonRecursive)
            .unwrap();
    }

    let mut offsets: HashMap<String, u64> = HashMap::new();

    for res in rx {
        match res {
            Ok(event) => {
                for path in event.paths {
                    let path = path.to_str().unwrap();
                    let offset = match offsets.get(path) {
                        Some(offset) => *offset,
                        None => 0,
                    };

                    let file = File::open(path).unwrap();
                    let mut reader = BufReader::new(file);
                    reader.seek(SeekFrom::Start(offset)).unwrap();

                    let mut s = String::new();
                    reader.read_to_string(&mut s).unwrap();

                    let initial = offset;
                    let offset = reader.seek(SeekFrom::Current(0)).unwrap();
                    offsets.insert(path.to_string(), offset);

                    if initial != offset {
                        println!("{}: {}", path.to_string(), s.strip_suffix("\n").unwrap());
                    }
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
