use log::info;

pub mod listen;

fn main() {
    log4rs::init_file("src/log.yaml", Default::default()).unwrap();

    info!("Hello, World!");

    let files = vec!["data/a", "data/b", "data/c", "data/d"];

    listen::listen(files);
}
