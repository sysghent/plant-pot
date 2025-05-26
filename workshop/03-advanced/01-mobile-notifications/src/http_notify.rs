use core::fmt::Write;

use embassy_net::{
    dns::DnsSocket,
    tcp::client::{TcpClient, TcpClientState},
};
use heapless::String;
use reqwless::client::HttpClient;

pub async fn notify_http(net_stack: &mut embassy_net::Stack<'_>, message: &str) {
    let state = TcpClientState::<1, 4096, 4096>::new();
    let tcp_client = TcpClient::new(*net_stack, &state);
    let dns_socket = DnsSocket::new(*net_stack);
    let mut _client = HttpClient::new(&tcp_client, &dns_socket);

    let mut body = String::<64>::new();
    body.write_str(message).unwrap();
    let mut _buffer = [0u8; 64];

    todo!("Create an HTTP request to ntfy.sh with the message in the body.");
}
