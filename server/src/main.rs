use core::panic;
use std::env::args;

use std::fs;
use std::process::exit;
use lazy_static::lazy_static;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use std::sync::Arc;
use std::io::Read;

lazy_static! {
    static ref BYTES: Arc<Vec<u8>> = {
        let mut args = args().into_iter();
        args.next().unwrap();
        if let Some(path) = args.next() {
            match fs::File::open(path) {
                Ok(mut file) => {
                    let mut buf = vec![];
                    file.read_to_end(&mut buf).ok();
                    Arc::new(buf)
                },
                Err(e) => {
                    panic!("{e}");
                }
            }
        }else {
            println!("A file is required");
            exit(1);
        }
    };
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("localhost:6969").await?;
    loop {
        let (mut socket, _) = listener.accept().await?; 
        tokio::spawn(async move {
            let bytes = &*BYTES;
            socket.write_all(&bytes).await.ok();
        });
    }
}
