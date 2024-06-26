use std::net::{TcpListener,TcpStream};

// define a function to handle client
fn handle_client(stream:TcpStream){
    println!("hello from handle client");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming(){
        handle_client(stream?);
    }
    Ok(())
}
