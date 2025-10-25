use defmt::{info, warn};
use embassy_net::{tcp::TcpSocket, IpAddress, IpEndpoint, Ipv4Address, Stack};
use heapless::String;

pub(crate) const ENDPOINT: IpEndpoint =
    IpEndpoint::new(IpAddress::Ipv4(Ipv4Address::new(192, 168, 1, 50)), 5038);
const USERNAME: &str = "halloween";
const PASSWORD: &str = env!("AMI_PASSWORD");

pub(crate) async fn place_call<'a>(stack: Stack<'a>, filename: &'static str) {
    // Create a TCP socket for AMI communication
    let mut rx_buffer = [0; 1024];
    let mut tx_buffer = [0; 1024];
    let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);

    // Try to connect to the AMI server
    info!("Connecting to AMI server at {}", crate::ami::ENDPOINT);
    match socket.connect(ENDPOINT).await {
        Ok(_) => {
            info!("Connected to AMI server");
            if let Err(e) = place_call_inner(socket, filename).await {
                warn!("Failed to place call: {:?}", e);
            }
            info!("AMI call origination complete");
        }
        Err(e) => {
            warn!("Failed to connect to AMI server: {:?}", e);
        }
    }
}

async fn place_call_inner<'a>(
    mut socket: TcpSocket<'a>,
    filename: &'static str,
) -> Result<(), AmiError> {
    info!("AMI: Logging in as {}", USERNAME);

    // Send login action
    let login_msg = format_login_message(USERNAME, PASSWORD);
    write_all(&mut socket, login_msg.as_bytes())
        .await
        .map_err(|_| AmiError::WriteError)?;

    // Read response
    let mut buf = [0u8; 512];
    let n = socket
        .read(&mut buf)
        .await
        .map_err(|_| AmiError::ReadError)?;

    info!("AMI: Login response received ({} bytes)", n);

    info!("AMI: Calling originate action");

    // Send originate action with dummy values
    let originate_msg = format_originate_message(filename);
    write_all(&mut socket, originate_msg.as_bytes())
        .await
        .map_err(|_| AmiError::WriteError)?;

    // Read response
    let mut buf = [0u8; 512];
    let n = socket
        .read(&mut buf)
        .await
        .map_err(|_| AmiError::ReadError)?;

    info!("AMI: Originate response received ({} bytes)", n);

    info!("AMI: Logging out");

    // Send logoff action
    let logoff_msg = format_logoff_message();
    write_all(&mut socket, logoff_msg.as_bytes())
        .await
        .map_err(|_| AmiError::WriteError)?;

    // Read response
    let mut buf = [0u8; 512];
    let n = socket
        .read(&mut buf)
        .await
        .map_err(|_| AmiError::ReadError)?;

    info!("AMI: Logout response received ({} bytes)", n);
    Ok(())
}

#[derive(Debug, defmt::Format)]
pub enum AmiError {
    WriteError,
    ReadError,
}

/// Write all bytes to the socket
async fn write_all(
    socket: &mut TcpSocket<'_>,
    mut buf: &[u8],
) -> Result<(), embassy_net::tcp::Error> {
    while !buf.is_empty() {
        let n = socket.write(buf).await?;
        buf = &buf[n..];
    }
    Ok(())
}

/// Format a login message for AMI protocol
fn format_login_message(username: &str, password: &str) -> String<256> {
    let mut msg = String::new();
    let _ = msg.push_str("Action: Login\r\n");
    let _ = msg.push_str("Username: ");
    let _ = msg.push_str(username);
    let _ = msg.push_str("\r\n");
    let _ = msg.push_str("Secret: ");
    let _ = msg.push_str(password);
    let _ = msg.push_str("\r\n");
    let _ = msg.push_str("\r\n");
    msg
}

/// Format an originate message for AMI protocol with dummy values
fn format_originate_message(filename: &'static str) -> String<512> {
    let mut msg = String::new();
    let _ = msg.push_str("Action: Originate\r\n");
    let _ = msg.push_str("Channel: pjsip/7025\r\n");
    let _ = msg.push_str("CallerID: Spooky\r\n");
    let _ = msg.push_str("Application: Playback\r\n");
    let _ = msg.push_str("Data: silence/2&/home/spooky/");
    let _ = msg.push_str(filename);
    let _ = msg.push_str("&silence/1\r\n");
    let _ = msg.push_str("Async: true\r\n");
    let _ = msg.push_str("\r\n");
    msg
}

/// Format a logoff message for AMI protocol
fn format_logoff_message() -> String<64> {
    let mut msg = String::new();
    let _ = msg.push_str("Action: Logoff\r\n");
    let _ = msg.push_str("\r\n");
    msg
}
