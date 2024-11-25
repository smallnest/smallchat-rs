use std::{
    io::{self, Read, Write},
    net::TcpStream,
    thread,
};

fn main() -> io::Result<()> {
    let addr = "127.0.0.1:8972";
    let mut stream = TcpStream::connect(addr)?;
    println!("Connected to {}", addr);

    // 克隆stream用于在不同线程中读写
    let mut read_stream = stream.try_clone()?;

    // 创建一个线程来处理服务器发来的消息
    thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            match read_stream.read(&mut buffer) {
                Ok(n) if n == 0 => {
                    println!("Server closed the connection");
                    break;
                }
                Ok(n) => {
                    let message = String::from_utf8_lossy(&buffer[..n]);
                    print!("{}", message);
                }
                Err(e) => {
                    println!("Failed to receive message: {}", e);
                    break;
                }
            }
        }
    });

    // 主线程处理用户输入
    let mut input = String::new();
    loop {
        input.clear();
        if io::stdin().read_line(&mut input)? == 0 {
            break;
        }
        
        if let Err(e) = stream.write_all(input.as_bytes()) {
            println!("Failed to send message: {}", e);
            break;
        }
    }

    Ok(())
} 