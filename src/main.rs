use std::io::{self, Write, BufReader, BufRead};
use std::net::{TcpListener,TcpStream};
use std::{thread, env};

// show the usage
fn usage(arg: &String){
    println!("Usage : {} [-S as Server/-C as Client] <ip> <port>", arg);
}

// check the argument to start as Server or Client
fn check_args(args: std::vec::Vec<std::string::String>) -> TcpStream {
    match args.len() {
        4 => {
            // format the ip:port
            let conn_id = format!("{}:{}", args[2], args[3]);
            match args[1].parse::<String>().unwrap().as_str() {
                "-S" => {
                    let listener = listen(&conn_id);
                    let client = accept(listener);
                    client
                },
                "-C" => {
                    let client = connect(&conn_id);
                    client
                },
                // if arg[1] != Client or Server
                _ => {
                    usage(&args[0]);
                    std::process::exit(0);
                }
            }
        }
        // args < 4 > args
        _ => {
            usage(&args[0]);
            std::process::exit(0);
        }
    }
}

// listen to the connection
fn listen(conn_id: &String) -> TcpListener {
    match TcpListener::bind(conn_id){
        Ok(sock) => {
            println!("[+] Listening on : {}", sock.local_addr().unwrap());
            sock
        }
        Err(_) => {
            println!("Cannot listen to {}...", conn_id);
            std::process::exit(0);
        }
    }
}

// connect to the Server
fn connect(conn_id: &String) -> TcpStream {
    match TcpStream::connect(conn_id) {
        Ok(sock) => {
            println!("[+] Connected to {}", conn_id);
            sock
        }
        Err(_) => {
            println!("Cannot connect to {}...", conn_id);
            std::process::exit(0);
        }
    }
}

// accept a connection and return the client
fn accept(listener: TcpListener) ->TcpStream {
    match listener.accept(){
        Ok((sock,addr)) => {
            println!("New client on {}", addr);
            sock
        },
        Err(e) => {
            println!("Err : {}",e);
            std::process::exit(0);
        }
    }
}

// read data from client
fn read(client: &std::net::TcpStream, data: &mut String) -> usize {
    let mut reader = BufReader::new(client);
    data.clear();
    match reader.read_line(data){
        Ok(bytes) => {
            print!("\r{}", data);
            bytes
        }
        Err(_) => return 0
    }
}

// write data to the client
fn write(mut client: &std::net::TcpStream, data: &String) -> usize {
    match client.write(data.as_bytes()){
        Ok(bytes) => bytes,
        Err(_) => 0
    }
}

// start a thread to write data to the connection
fn write_thread(client_write: &TcpStream) -> thread::JoinHandle<i32> {
    let client_write = client_write.try_clone().unwrap();
    let write = thread::spawn( move ||{
        loop{
            // ask for a string to send
            let data = ask_string();
            // send data to the client
            println!("{} bytes sent !", write(&client_write, &data));
            if data.trim() == "STOP" {
                client_write.shutdown(std::net::Shutdown::Both).expect("Failed to close the connection...");
                println!("[-] Server closed with success");
                std::process::exit(0);
            }
        };
    });
    return write;
}

// start a thread to read data to the connection
fn read_thread(client_read: &TcpStream) -> thread::JoinHandle<i32> {
    let client_read = client_read.try_clone().unwrap();
    let read = thread::spawn( move || {
        // create the variable to store de write/send data
        let mut data = String::new();
        loop{
            // read data from the client
            println!("{} bytes received !", read(&client_read, &mut data));
            if data.trim() == "STOP" {
                client_read.shutdown(std::net::Shutdown::Both).expect("Failed to close the connection...");
                println!("[-] Server closed with success");
                std::process::exit(0);
            }
        };
    });
    return read;
}

// ask the user for a string to send
fn ask_string() -> String{
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("read stdin err");
    return data;
}

fn main() {
    // take the argument to check Server or Client
    let args: Vec<String> = env::args().collect();

    // check the argument
    let client: TcpStream = check_args(args);

    // thread to send data
    let write = write_thread(&client);
    
    // thread to write data
    let read = read_thread(&client);

    // wait for the threads to finish/exit
    write.join().unwrap();
    read.join().unwrap();
}