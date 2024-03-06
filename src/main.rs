use std::env;
use std::net::UdpSocket;
use std::time::Instant;

const BUFFER_SIZE: usize = 1024;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: cargo run --release -- <send/recv> <ip:port> <total_size>");
        return;
    }

    let mode = &args[1];
    let address = &args[2];
    let total_size: usize = args[3].parse().unwrap();

    match mode.as_str() {
        "send" => run_sender(address, total_size),
        "recv" => run_receiver(address, total_size),
        _ => println!("Invalid mode. Use 'send' or 'recv'."),
    }
}

fn run_sender(address: &str, total_size: usize) {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind sender socket");
    socket
        .connect(address)
        .expect("Failed to connect to receiver");

    let start_time = Instant::now();
    let mut total_bytes_sent = 0;

    while total_bytes_sent < total_size {
        let bytes_to_send = (total_size - total_bytes_sent).min(BUFFER_SIZE);
        let buffer = vec![0; bytes_to_send];

        let bytes_sent = socket.send(&buffer).expect("Failed to send data");
        total_bytes_sent += bytes_sent;
    }

    let elapsed_time = start_time.elapsed();
    let throughput = (total_bytes_sent as f64 / elapsed_time.as_secs_f64()) / 1024.0 / 1024.0;

    println!(
        "Sent {} bytes in {:.2?} ({:.2} MB/s)",
        total_bytes_sent, elapsed_time, throughput
    );
}

fn run_receiver(address: &str, total_size: usize) {
    let socket = UdpSocket::bind(address).expect("Failed to bind receiver socket");

    let start_time = Instant::now();
    let mut total_bytes_received = 0;

    while total_bytes_received < total_size {
        let mut buffer = [0; BUFFER_SIZE];
        let bytes_received = socket.recv(&mut buffer).expect("Failed to receive data");
        total_bytes_received += bytes_received;
    }

    let elapsed_time = start_time.elapsed();
    let throughput = (total_bytes_received as f64 / elapsed_time.as_secs_f64()) / 1024.0 / 1024.0;

    println!(
        "Received {} bytes in {:.2?} ({:.2} MB/s)",
        total_bytes_received, elapsed_time, throughput
    );
}