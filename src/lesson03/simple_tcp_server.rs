
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::str;
use crate::lesson04::course04::Time;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    let mut buf = [0; 512];//定义一个数组
    for _ in 0..1000 {//每个连接只处理1000次数据
        let bytes_read = stream.read(&mut buf)?;//从流中读取数据并存储在buf中
        if bytes_read == 0 {
            return Ok(());
        }
        print!("从客户端接收到数据：{}",
               str::from_utf8(&buf[..bytes_read]).expect("Could not write buffer as string"));

        stream.write(&buf[..bytes_read])?;//将从客户端接收到的数据发回客户端
    }

    Ok(())
}

pub fn start_server(){

    let result = run();
    match result {
        Result::Ok(..)=>(),
        Result::Err(ee)=>{
            println!("{}", ee);
        }
    }
}

pub fn run() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;//监听8080端口
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    for stream in listener.incoming() {
        let stream = stream.expect("failed!");
        let handle = thread::spawn(move || {
            handle_client(stream)
                .unwrap_or_else(|error| eprintln!("{:?}", error));
        });

        thread_vec.push(handle);
    }

    for handle in thread_vec {
        handle.join().unwrap();
    }

    Ok(())
}