use std::{env, fs::read_to_string, path::Path, thread, time::Duration};
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Result, FsEventWatcher, Event};

use aurelius::Server;



pub fn create_markdown_server(file: &str) -> FsEventWatcher{

    let mut server = Server::bind(format!(
        "{}:{}",
        "localhost",
        "3456"
    )).unwrap();
    server.set_static_root(env::current_dir().unwrap());
    // server.send("**markdown**".to_string());
    server.set_static_root(env::current_dir().unwrap());
    server.send(read_to_string(file).unwrap()).unwrap();
    let mut watcher = notify::recommended_watcher(move |res: Result<Event>| {
        match res {
           Ok(event) => {
            server.send(read_to_string(event.paths.get(0).unwrap()).unwrap()).unwrap();
           },
           Err(e) => println!("watch error: {:?}", e),
        }
    }).unwrap();
    watcher.watch(Path::new(&file), RecursiveMode::Recursive).unwrap();
    watcher
}