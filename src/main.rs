mod netlib;
use std::env;
use std::net::TcpStream;

fn main() {
    // take the argument to check Server or Client
    let args: Vec<String> = env::args().collect();

    // check the argument
    let client: TcpStream = netlib::check_args(args);

    // thread to send data
    let write = netlib::write_thread(&client);
    
    // thread to write data
    let read = netlib::read_thread(&client);

    // wait for the threads to finish/exit
    write.join().unwrap();
    read.join().unwrap();
}
