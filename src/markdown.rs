use futures::channel::mpsc::channel;
use notify::{Event, FsEventWatcher, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::{env, fs::read_to_string, path::Path, thread, time::Duration, marker::PhantomData};

use aurelius::Server;
use tokio::net::lookup_host;

#[derive(Debug)]
pub struct MDServer {
    file: String,
    pub(crate) server: Server,
    watcher: Option<FsEventWatcher>,
    phantom: PhantomData<&'static ()>,
}
impl MDServer {
    pub async fn new(file: &str) -> Self {
        let mut server = Server::bind(&lookup_host("localhost:0").await.unwrap().next().unwrap())
            .await
            .unwrap();
        server.set_static_root(env::current_dir().unwrap());
        // server.send("**markdown**".to_string());
        server.set_static_root(env::current_dir().unwrap());
        server
            .send(read_to_string(file).unwrap().as_str())
            .await
            .unwrap();
        Self { 
            file: file.to_string(),
            server: server,
            watcher: None,
            phantom: PhantomData
        }
    }
    pub async fn on_watch(&self, res: Result<Event>) {
        
            match res {
                Ok(event) => {
                    self.server
                        .send(
                            read_to_string(event.paths.get(0).unwrap())
                                .unwrap()
                                .as_str(),
                        )
                        .await
                        .unwrap();
                }
                Err(e) => println!("watch error: {:?}", e),
            }
            println!("{:?}", self.server);
        
    }
    pub async fn watch(&mut self){
        let (mut tx, mut rx) = channel(1);
        let mut watcher = notify::recommended_watcher(move |res: Result<Event>| {
            futures::executor::block_on(async {
                tx.try_send(res).unwrap();
            })
        })
        .unwrap();
        watcher
            .watch(Path::new(&self.file), RecursiveMode::Recursive)
            .unwrap();
        while let Ok(Some(res)) = rx.try_next() {
            self.on_watch(res).await
        }
        self.watcher=Some(watcher)
    }
}
