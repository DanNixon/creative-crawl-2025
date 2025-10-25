use crate::stories::Genre;
use defmt::{debug, info, warn, Format};
use embassy_net::{
    dns::DnsSocket,
    tcp::client::{TcpClient, TcpClientState},
    Stack,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};
use embassy_time::Duration;
use reqwless::{
    client::HttpClient,
    headers::ContentType,
    request::{Method, RequestBuilder},
    response::StatusCode,
};

const URL: &str = "http://192.168.8.249/json/state";

#[derive(Debug, Format, Clone, Copy)]
pub(crate) enum Preset {
    DoorClosed,
    DoorOpened,
    Genre(Genre),
}

impl Preset {
    fn json(&self) -> &'static str {
        match self {
            Preset::DoorClosed => include_str!("./wled_presets/door_closed.json"),
            Preset::DoorOpened => include_str!("./wled_presets/door_opened.json"),
            Preset::Genre(Genre::Maker) => include_str!("./wled_presets/genre_maker.json"),
            Preset::Genre(Genre::Hacker) => include_str!("./wled_presets/genre_hacker.json"),
            Preset::Genre(Genre::Gruesome) => include_str!("./wled_presets/genre_gruesome.json"),
            Preset::Genre(Genre::Dystopian) => include_str!("./wled_presets/genre_dystopian.json"),
            Preset::Genre(Genre::Cyberpunk) => include_str!("./wled_presets/genre_cyberpunk.json"),
        }
    }
}

pub(crate) static APPLY_PRESET: Watch<CriticalSectionRawMutex, Preset, 1> = Watch::new();

#[embassy_executor::task]
pub(crate) async fn task(stack: Stack<'static>) -> ! {
    let client_state = TcpClientState::<1, 1024, 1024>::new();
    let tcp_client = TcpClient::new(stack, &client_state);
    let dns_client = DnsSocket::new(stack);
    let mut http_client = HttpClient::new(&tcp_client, &dns_client);

    let mut preset_rx = APPLY_PRESET.receiver().unwrap();

    loop {
        let preset = preset_rx.changed().await;
        info!("Setting lighting to preset: {}", preset);

        stack.wait_config_up().await;

        apply_preset(&mut http_client, preset).await.unwrap();
    }
}

async fn apply_preset<'a>(
    http_client: &mut HttpClient<'a, TcpClient<'a, 1>, DnsSocket<'a>>,
    preset: Preset,
) -> Result<(), ()> {
    info!("Applying preset: {}", preset);

    let json = preset.json().as_bytes();

    let mut rx_buffer = [0; 8192];

    let mut request = match embassy_time::with_timeout(
        Duration::from_secs(3),
        http_client.request(Method::POST, URL),
    )
    .await
    {
        Ok(Ok(request)) => Ok(request
            .content_type(ContentType::ApplicationJson)
            .body(json)),
        Ok(Err(e)) => {
            warn!("WLED set failed: {}", e);
            Err(())
        }
        Err(_) => {
            warn!("WLED set failed: timeout");
            Err(())
        }
    }?;

    match embassy_time::with_timeout(Duration::from_secs(3), request.send(&mut rx_buffer)).await {
        Ok(Ok(response)) => {
            if response.status == StatusCode(200) {
                debug!("WLED set success: status={}", response.status);
                Ok(())
            } else {
                warn!("WLED set failed: status={}", response.status);
                Err(())
            }
        }
        Ok(Err(e)) => {
            warn!("WLED set failed: {}", e);
            Err(())
        }
        Err(_) => {
            warn!("WLED set failed: timeout");
            Err(())
        }
    }
}
