use crypto;
use netlib;
use std::env;
use std::net::TcpStream;

fn main() {
    // take the argument to check Server or Client
    let args: Vec<String> = env::args().collect();

    // check the argument
    let mut client: TcpStream = netlib::check_args(args);

    // generate the key
    let (priv_key, mut pub_key) = crypto::gen_key();

    // share the key with the client
    netlib::share_key(&mut client, &mut pub_key);

    // thread to send data
    let write = netlib::write_thread(&client, &pub_key);

    // thread to write data
    let read = netlib::read_thread(&client, &priv_key);

    // wait for the threads to finish/exit
    write.join().unwrap();
    read.join().unwrap();
}
