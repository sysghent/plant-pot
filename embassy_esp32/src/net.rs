use core::fmt::Write;

use defmt::info;
use embassy_net::{
    dns::DnsSocket,
    tcp::client::{TcpClient, TcpClientState},
};
use heapless::String;
use reqwless::{client::HttpClient, request::RequestBuilder};

pub async fn notify(net_stack: &mut embassy_net::Stack<'_>, message: &str) {
    let state = TcpClientState::<1, 4096, 4096>::new();
    let tcp_client = TcpClient::new(*net_stack, &state);
    let dns_socket = DnsSocket::new(*net_stack);
    let mut client = HttpClient::new(&tcp_client, &dns_socket);

    let mut body = String::<64>::new();
    body.write_str(message).unwrap();
    let mut buffer = [0u8; 64];
    info!("Sending notification to ntfy.sh");

    let response = client
        .request(
            reqwless::request::Method::POST,
            "http://ntfy.sh/smart-plant-pot",
        )
        .await
        .unwrap()
        .content_type(reqwless::headers::ContentType::TextPlain)
        .body(body.as_bytes())
        .send(&mut buffer)
        .await
        .unwrap()
        .status
        .0;

    info!("Response: {}", response);
}
