#![deny(clippy::all, clippy::pedantic, clippy::cargo)]
#![allow(dead_code)]

use crate::api::packet::{
    reader::PacketDeserialize,
    Header,
};
use color_eyre::eyre::Result;
use tokio::net::UdpSocket;
use tracing::{
    info,
    trace,
    warn,
};

mod api;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    // TODO: configure
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;
    let sock = UdpSocket::bind("0.0.0.0:22023").await?;
    let mut buf = [0; 1024];

    info!("started udp receiver");
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        trace!("received data from {}: {:?}", addr, &buf[0..len]);
        let buf = &buf[0..len];
        if len < size_of::<Header>() {
            warn!("buf is not large enough: size {} bytes", len);
            continue;
        }

        let Ok(header) = Header::create_from(buf).await else {
            warn!("failed to parse header");
            continue;
        };

        info!("parsed header: {:#?}", header);
    }
}
