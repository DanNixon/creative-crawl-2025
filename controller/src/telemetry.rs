use defmt::{debug, warn};
use embassy_net::{
    dns::DnsSocket,
    tcp::client::{TcpClient, TcpClientState},
    Stack,
};
use embassy_rp::clocks::RoscRng;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub::PubSubChannel};
use embassy_time::Duration;
use heapless::String;
use reqwless::{
    client::{HttpClient, TlsConfig, TlsVerify},
    headers::ContentType,
    request::{Method, RequestBuilder},
    response::StatusCode,
};

pub(crate) type TelemetryString = String<512>;

pub(crate) static TELEMETRY_QUEUE: PubSubChannel<
    CriticalSectionRawMutex,
    TelemetryString,
    8,
    1,
    3,
> = PubSubChannel::new();

const URL: &str = "https://creative-crawl-telemetry.makerspace.org.uk";
const USERNAME: &str = "creative-crawl-2025";
const PASSWORD: &str = env!("TELEGRAF_PASSWORD");

#[embassy_executor::task]
pub(crate) async fn task(stack: Stack<'static>) -> ! {
    let mut rng = RoscRng;

    let client_state = TcpClientState::<1, 1024, 1024>::new();
    let tcp_client = TcpClient::new(stack, &client_state);
    let dns_client = DnsSocket::new(stack);
    let mut tls_read_buffer = [0; 16640];
    let mut tls_write_buffer = [0; 16640];
    let tls_config = TlsConfig::new(
        rng.next_u64(),
        &mut tls_read_buffer,
        &mut tls_write_buffer,
        TlsVerify::None,
    );
    let mut http_client = HttpClient::new_with_tls(&tcp_client, &dns_client, tls_config);
    let mut rx_buffer = [0; 8192];

    let mut msg_rx = TELEMETRY_QUEUE.subscriber().unwrap();

    loop {
        // Get the next formatted message to send
        let msg = msg_rx.next_message_pure().await;

        // Ensure the network is up before trying to send it
        stack.wait_config_up().await;

        // Send the message to Telegraf
        debug!("Submitting telemetry to {}", &URL);
        debug!("Formatted string: `{}`", msg);

        let mut request = match embassy_time::with_timeout(
            Duration::from_secs(3),
            http_client.request(Method::POST, URL),
        )
        .await
        {
            Ok(Ok(request)) => request
                .basic_auth(USERNAME, PASSWORD)
                .content_type(ContentType::TextPlain)
                .body(msg.as_bytes()),
            Ok(Err(e)) => {
                warn!("Metrics submission failed: {}", e);
                continue;
            }
            Err(_) => {
                warn!("Metrics submission failed: timeout");
                continue;
            }
        };

        match embassy_time::with_timeout(Duration::from_secs(3), request.send(&mut rx_buffer)).await
        {
            Ok(Ok(response)) => {
                if response.status == StatusCode(204) {
                    debug!("Metrics submission success: status={}", response.status);
                } else {
                    warn!("Metrics submission failed: status={}", response.status);

                    if response.status == StatusCode(400) {
                        warn!("Telegraf reports bad request");
                    }

                    continue;
                }
            }
            Ok(Err(e)) => {
                warn!("Metrics submission failed: {}", e);
                continue;
            }
            Err(_) => {
                warn!("Metrics submission failed: timeout");
                continue;
            }
        };
    }
}
