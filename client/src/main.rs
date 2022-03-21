use std::fs::{OpenOptions};
use std::io::Write;
use std::{env::args};

use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = args().into_iter();
    args.next().unwrap();
    if let Some(host) = args.next() {
        let save_as = if let Some(name) = args.next() { name } else { "default.name".into() };
        println!("downloading file from {host}, saving as {save_as}");
        let mut stream = TcpStream::connect(host).await?;
        let mut buf: Vec<u8> = vec![];
        stream.read_to_end(&mut buf).await.ok();
        if let Ok(mut file) = OpenOptions::new().write(true).create(true).truncate(true).open(save_as) {
            if let Err(e) = file.write_all(&mut buf) {
                panic!("{e}");
            }   
        }else {
            panic!("Error");
        }
    }
    Ok(())
}
