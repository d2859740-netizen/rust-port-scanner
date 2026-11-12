use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use std::io::{self, Write};
use std::thread;
use std::sync::{Arc, Mutex};
use std::fs::OpenOptions;

fn main() {
    println!("Enter target:");
    let mut new = String::new();
    io::stdin().read_line(&mut new).expect("error");
    let new = new.trim();


    let open_ports = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![]; 

    for port in 1..101 {
        let target = new.to_string(); 
        let open_ports_clone = Arc::clone(&open_ports); 

        let handle = thread::spawn(move || { 
            let address = format!("{}:{}", target, port);

            if let Ok(mut addrs) = address.to_socket_addrs() {
                if let Some(socket_addr) = addrs.next() {
                    match TcpStream::connect_timeout(&socket_addr, Duration::from_secs(1)) {
                        Ok(_) => {
                            println!("Порт {}: OPEN", port);
                           
                            open_ports_clone.lock().unwrap().push(port);
                        },
                        Err(_) => println!("Порт {}: close", port),
                    }
                }
            }
        });
        
        handles.push(handle); 
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("scan_results.txt")
        .expect("Cannot open file");

    let ports_to_save = open_ports.lock().unwrap();
    writeln!(file, "=== Scan for {} ===", new).expect("Cannot write to file");
    if ports_to_save.is_empty() {
        writeln!(file, "No open ports found.").expect("Cannot write");
    } else {
        for port in ports_to_save.iter() {
            writeln!(file, "Port {}: OPEN", port).expect("Cannot write");
        }
    }

    println!("Results saved to scan_results.txt");
}