use std::io::{self, Write, BufReader, BufRead};
use std::net::{TcpListener,TcpStream};
use std::thread;

/// show the usage
/// # usage
/// ```
/// println!("Usage : {} [-S as Server/-C as Client] <ip> <port>");
/// ```
pub fn usage(arg: &String){
    println!("Usage : {} [-S as Server/-C as Client] <ip> <port>", arg);
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
            println!("[+] Connected to {}", conn_id);
            sock
        }
        Err(_) => {
            println!("Cannot connect to {}...", conn_id);
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
pub fn accept(listener: TcpListener) ->TcpStream {
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

/// read data from client
/// # argument
///
/// * `client` - reference to a TcpStream
/// * `data`   - reference to a mutable String
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
    match reader.read_line(data){
        Ok(bytes) => {
            let data = data.decrypt("azerty1234".to_string());
            print!("\r{}", data);
            bytes
        }
        Err(_) => return 0
    }
}

/// write data to the client
/// # argument
///
/// * `client` - reference to a TcpStream
/// * `data`   - reference to a String
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
    let data = data.encrypt("azerty1234".to_string());
    match client.write(data.as_bytes()){
        Ok(bytes) => bytes,
        Err(_) => 0
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
pub fn write_thread(client_write: &TcpStream) -> thread::JoinHandle<i32> {
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
pub fn read_thread(client_read: &TcpStream) -> thread::JoinHandle<i32> {
   let client_read = client_read.try_clone().unwrap();
    let read = thread::spawn( move || {
        // create the variable to store de write/send data
        let mut data = String::new();
        loop{
            // read data from the client
            let bytes = read(&client_read, &mut data);
            if bytes == 0 { // if 0 bytes received
                continue;
            }
            println!("{} bytes received !", bytes);
            if data.trim() == "STOP" {
                client_read.shutdown(std::net::Shutdown::Both).expect("Failed to close the connection...");
                println!("[-] Server closed with success");
                std::process::exit(0);
            }
        };
    });
    return read;
}

// group of function for Crypto
trait Crypto {
    fn encrypt(&self, key: String) -> String;
    fn decrypt(&self, key: String) -> String;
}
// add Crypto's functions for String type
impl Crypto for String {
    /// encrypt the string
    /// # return value
    ///
    /// return an encrypted String
    ///
    /// # Example :
    /// ```
    /// let s = "Hello".to_string();
    /// let s = encrypt(s);
    /// println!("{}",s);
    /// ```
    fn encrypt(&self, _key: String) -> String {
        return self.to_string();
    }
    
    /// decrypt the string
    /// # return value
    ///
    /// return an decrypted String
    ///
    /// # Example :
    /// ```
    /// let s = "Hello".to_string();
    /// let s = decrypt(s);
    /// println!("{}",s);
    /// ```
    fn decrypt(&self, _key: String) -> String {
        return self.to_string();
    }
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
pub fn ask_string() -> String{

    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("read stdin err");
    return data;
}
