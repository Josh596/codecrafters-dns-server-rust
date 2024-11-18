#[allow(unused_imports)]
use std::net::UdpSocket;
use std::time::Duration;

use codecrafters_dns_server::{answer::DNSAnswer, message::DNSMessage};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    let args = std::env::args().collect::<Vec<_>>();
    let resolver = args[1] == "--resolver";
    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let mut dns_message = DNSMessage::from(&buf[..]);

                // Forward  dns_message, and get the response.
                if resolver {
                    dns_message = forward_dns_request(&udp_socket, &args[2], dns_message)
                        .expect("Bad Error Handling guy");
                }
                println!("Creating response");
                let mut response_dns = dns_message.clone();
                response_dns.header.qr = true;
                let response = response_dns.encode();
                println!("Response created");
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
                println!("Sent response to {source}")
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}

fn forward_dns_request(
    upstream_socket: &UdpSocket, // Keep the original socket for client communication
    upstream_addr: &String,
    mut dns_message: DNSMessage,
) -> Result<DNSMessage, ()> {
    // Create a new socket for upstream communication
    let upstream_socket =
        UdpSocket::bind("127.0.0.1:2052").expect("Failed to bind upstream socket");

    // upstream_socket.connect(upstream_addr);

    let mut dns_messages = Vec::new();
    for m in &mut dns_message.split() {
        upstream_socket
            .send_to(&m.encode(), upstream_addr)
            .expect("Failed to forward query to upstream");
        println!("Sent message to {upstream_addr}");
        // upstream_socket
        //     .set_read_timeout(Some(Duration::from_secs(2)))
        //     .expect("Failed to set read timeout");

        let mut recv_buf = [0; 512];
        println!("Waiting to receive data");
        match upstream_socket.recv_from(&mut recv_buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from forwarder {}", size, source);
                let mut dns_message_from_upstream = DNSMessage::from(&recv_buf[..]); // Note: using size here
                if dns_message_from_upstream.header.qdcount == 0 {
                    dns_message_from_upstream.questions.questions =
                        vec![m.questions.questions[0].clone()];

                    dns_message_from_upstream.answer = DNSAnswer::new();
                }
                dns_messages.push(dns_message_from_upstream);
            }
            Err(e) => {
                eprintln!("Error receiving data from forwader {}", e);
                return Err(());
            }
        };
    }

    println!("Merging {} DNS Messages", dns_messages.len());
    let dns_message = DNSMessage::merge(dns_messages);

    Ok(dns_message)
}
