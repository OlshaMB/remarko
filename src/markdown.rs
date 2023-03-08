use std::env;

use aurelius::Server;


pub fn create_markdown_server() {
    let mut server = Server::bind(format!(
        "{}:{}",
        "localhost",
        "3456"
    )).unwrap();
    server.set_static_root(env::current_dir().unwrap());
    // server.send("**markdown**".to_string());
}