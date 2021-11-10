use colored::*;
use crypto::{self, Crypto};
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

/// show the usage
pub fn usage() {
    println!(
        "Usage : rustchat [{option}] [{par}]
  {option} :
\t-{serv} {ip} {port}                 Start as server
\t-{cli} {ip} {port}                 Start as Client
\t-{key} {priF} {pubF}        Generate private and public key",
        option = "option".yellow(),
        par = "parameters".cyan(),
        ip = "<ip>".cyan(),
        port = "<port>".cyan(),
        priF = "<privFile>".cyan(),
        pubF = "<pubFile>".cyan(),
        serv = "S".yellow(),
        cli = "C".yellow(),
        key = "K".yellow()
    );
}

/// check the argument to start as Server or Client
/// # argument
///
/// * `args` - Vector of String containing the arguments
///
/// # return value
/// Will return a TcpStream connection to a client or a server
///
/// # Example :
/// ```
/// // setup the arguments to start as Server
/// let args = vec!["program".to_string(), "-S".into(), "127.0.0.1".into(), "6666".into()];
/// // the client -> TcpStream
/// let client = check_args(args);
/// ```
pub fn check_args(args: std::vec::Vec<std::string::String>) -> TcpStream {
    match args.len() {
        4 => {
            // format the ip:port
            let conn_id = format!("{}:{}", &args[2], &args[3]);
            match args[1].parse::<String>().unwrap().as_str() {
                "-S" => {
                    let listener = listen(&conn_id);
                    let client = accept(listener);
                    client
                }
                "-C" => {
                    let client = connect(&conn_id);
                    client
                }
                "-K" => {
                    let (priv_key, pub_key) = crypto::gen_key();
                    write_to_file(priv_key, args[2].to_string());
                    write_to_file(pub_key, args[3].to_string());
                    std::process::exit(0);
                }
                // if arg[1] != Client or Server
                _ => {
                    usage();
                    std::process::exit(0);
                }
            }
        }
        // args < 4 > args
        _ => {
            usage();
            std::process::exit(0);
        }
    }
}

/// listen to the connection
/// # argument
///
/// * `conn_id` - reference to a String in this form "<ip>:<port>"
///
/// # return value
/// return a TcpListener to the given address
///
/// # Example :
/// ```
/// // setup the variable containing the address to listen to
/// let conn_id = format!("{}:{}", "127.0.0.1".to_string(), "6666".to_string());
/// // start the tcp listener
/// let listener = listen(&conn_id);
/// ```
pub fn listen(conn_id: &String) -> TcpListener {
    match TcpListener::bind(conn_id) {
        Ok(sock) => {
            println!(
                "{} Listening on : {}",
                "[+]".cyan(),
                sock.local_addr().unwrap()
            );
            sock
        }
        Err(_) => {
            println!("{} Cannot listen to {}...", "[-]".red(), conn_id);
            std::process::exit(0);
        }
    }
}

/// connect to the Server
/// call this function if you're a client
/// # argument
///
/// * `conn_id` - reference to a String in this form "<ip>:<port>"
///
/// # return value
///
/// return a TcpStream connection to the given address
///
/// # Example :
/// ```
/// // setup the variable containing the address to connect to
/// let conn_id = format!("{}:{}", "127.0.0.1".to_string(), "6666".to_string());
/// // start the tcp connection
/// let conn = listen(&conn_id);
/// ```
pub fn connect(conn_id: &String) -> TcpStream {
    match TcpStream::connect(conn_id) {
        Ok(sock) => {
            println!("{} Connected to {}", "[+]".cyan(), conn_id);
            sock
        }
        Err(_) => {
            println!("{} Cannot connect to {}...", "[-]".red(), conn_id);
            std::process::exit(0);
        }
    }
}

/// accept a connection and return the client
/// call this function if you're a server
/// # argument
///
/// * `listener` - a  TcpListener
///
/// # return value
///
/// return a TcpStream connection to the given address
///
/// # Example :
/// ```
/// // setup the variable containing the address to connect to
/// let conn_id = format!("{}:{}", "127.0.0.1".to_string(), "6666".to_string());
/// // start the tcp connection
/// let conn = listen(&conn_id);
/// // accept a client
/// let client = accept(listener);
/// ```
pub fn accept(listener: TcpListener) -> TcpStream {
    match listener.accept() {
        Ok((sock, addr)) => {
            println!("{} New client on {}", "[+]".cyan(), addr);
            sock
        }
        Err(e) => {
            println!("{} Err : {}", "[-]".red(), e);
            std::process::exit(0);
        }
    }
}

/// read data from client
/// # argument
///
/// * `client` - reference to a TcpStream
/// * `data`   - reference to a mutable String, will contain the data
///
/// # return value
///
/// return the number of bytes received from the connection
///
/// # Example :
/// ```
/// // setup the variable containing the address to connect to
/// let conn_id = format!("{}:{}", "127.0.0.1".to_string(), "6666".to_string());
/// // start the tcp connection
/// let conn = listen(&conn_id);
/// // accept a client
/// let client = accept(listener);
/// let mut client_data = String::new();
/// read(&client, &mut client_data);
/// ```
pub fn read(client: &std::net::TcpStream, data: &mut String) -> usize {
    let mut reader = BufReader::new(client);
    data.clear();
    match reader.read_line(data) {
        Ok(bytes) => bytes,
        Err(_) => 0,
    }
}

/// write data to the client
/// # argument
///
/// * `client` - reference to a TcpStream
/// * `data`   - reference to a String, will contain the data
///
/// # return value
///
/// return the number of bytes sent to the connection
///
/// # Example :
/// ```
/// // setup the variable containing the address to connect to
/// let conn_id = format!("{}:{}", "127.0.0.1".to_string(), "6666".to_string());
/// // start the tcp connection
/// let conn = listen(&conn_id);
/// // accept a client
/// let client = accept(listener);
/// let data = "Hello you\n".to_string();
/// write(&client, &data);
/// ```
pub fn write(mut client: &std::net::TcpStream, data: &String) -> usize {
    match client.write(data.as_bytes()) {
        Ok(bytes) => bytes,
        Err(_) => {
            println!("{} message could not be sent", "[-]".red());
            0
        }
    }
}

/// start a thread to write data to the connection
/// # argument
///
/// * `client_write` - reference to a TcpStream
///
/// # return value
///
/// return a number allowing the caller to wait for the thread to finish
///
/// # Example :
/// ```
/// // setup the variable containing the address to connect to
/// let conn_id = format!("{}:{}", "127.0.0.1".to_string(), "6666".to_string());
/// // start the tcp connection
/// let conn = listen(&conn_id);
/// // accept a client
/// let client = accept(listener);
/// // start the thread
/// let t_write = write_thread(&client);
/// t_write.join().unwrap();
/// ```
pub fn write_thread(client_write: &TcpStream, key: &String) -> thread::JoinHandle<i32> {
    let client_write = client_write.try_clone().unwrap();
    let key = key.clone();
    let write = thread::spawn(move || {
        loop {
            // ask for a string to send
            let data = ask_string();
            let buff = data.trim();
            // encrypt the string
            let mut data = data.encrypt(key.to_string());
            // add newline
            data.push('\n');
            // send data to the client
            println!(
                "{} bytes sent !",
                write(&client_write, &data).to_string().magenta()
            );
            if buff == "STOP" {
                println!("{} Connection closed with success", "[+]".cyan(),);
                std::process::exit(0);
            }
        }
    });
    return write;
}

/// start a thread to read data from the connection
/// # argument
///
/// * `client_read` - reference to a TcpStream
///
/// # return value
///
/// return a number allowing the caller to wait for the thread to finish
///
/// # Example :
/// ```
/// // setup the variable containing the address to connect to
/// let conn_id = format!("{}:{}", "127.0.0.1".to_string(), "6666".to_string());
/// // start the tcp connection
/// let conn = listen(&conn_id);
/// // accept a client
/// let client = accept(listener);
/// // start the thread
/// let t_read = read_thread(&client);
/// t_read.join().unwrap();
/// ```
pub fn read_thread(client_read: &TcpStream, key: &String) -> thread::JoinHandle<i32> {
    let client_read = client_read.try_clone().unwrap();
    let key = key.clone();
    let read = thread::spawn(move || {
        // create the variable to store de write/send data
        let mut data = String::new();
        loop {
            // read data from the client
            let bytes = read(&client_read, &mut data);
            if bytes == 0 {
                // if 0 bytes received
                continue;
            }
            // remove the newline
            data.pop();
            // check if the string is sent by the program
            if !data.check_string() {
                println!("{} Warning someone is wathing us...", "[-]".red(),);
                continue;
            }
            // decrypt the data received
            let data = data.decrypt(key.to_string());
            println!("{}", data);
            println!("{} bytes received !", bytes.to_string().magenta());
            if data.trim() == "STOP" {
                //client_read.shutdown(std::net::Shutdown::Both).expect("Failed to close the connection...");
                println!("{} Connection closed with success", "[+]".cyan(),);
                std::process::exit(0);
            }
        }
    });
    return read;
}

/// ask the user for a string to send
/// # return value
///
/// return a String
///
/// # Example :
/// ```
/// println!("Your name :");
/// let name = ask_string();
/// println!("Hi {} !", name);
/// ```
pub fn ask_string() -> String {
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("read stdin err");
    return data;
}

/// write the key to the file given in argument
///
/// # Example :
/// ```
/// let (privK,pubK) = gen_key();
/// write_to_file(privK, "privateKey.txt".to_string());
/// ```
fn write_to_file(key: String, file_name: String) {
    // open the file
    let mut f = match OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .open(&file_name)
    {
        Ok(f) => {
            println!("{} File opened with success !", "[+]".cyan(),);
            f
        }
        Err(_) => {
            println!("{} Couldn't open the file...", "[-]".red(),);
            std::process::exit(0)
        }
    };
    // write to the file
    match f.write_all(key.as_bytes()) {
        Ok(()) => {
            println!(
                "{} Key written to the file {} with success !",
                "[+]".cyan(),
                &file_name
            );
            ()
        }
        Err(_) => {
            println!("{} Error when writing to the file...", "[-]".red(),);
            std::process::exit(0);
        }
    }
}

/// share key with the connection
///
/// # Example :
/// ```
/// let key = crypto::gen_key()
/// share_key();
/// ```
pub fn share_key(client: &mut TcpStream, key: &mut String) {
    // add newline to the key to allow the client to read it
    key.push('\n');
    // send the key
    write(client, key);
    //println!("{} key sent with success","[+]".cyan());
    // receive the key
    read(client, key);
    // remove the second newline added with the transfert
    key.pop();
    //println!("{} key received with success","[+]".cyan());
    println!("{} key exchanged with success", "[+]".cyan());
}
