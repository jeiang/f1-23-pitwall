#![feature(associated_type_defaults, error_generic_member_access)]
#![deny(clippy::all, clippy::pedantic, clippy::cargo)]
#![allow(dead_code)]
extern crate core;

use crate::api::packet::{
    DeserializeUDP,
    DeserializeUDPError,
    Packet,
};
use color_eyre::eyre::Result;
use tokio::net::UdpSocket;
use tracing::{
    error,
    info,
    trace,
    Level,
};

mod api;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    // TODO: configure
    let subscriber = tracing_subscriber::fmt().with_max_level(Level::TRACE).finish();
    tracing::subscriber::set_global_default(subscriber)?;
    let sock = UdpSocket::bind("0.0.0.0:22023").await?;
    let mut buf = [0; 2048];

    info!("started udp receiver");
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        let buf = &buf[0..len];

        if buf.len() < size_of::<Packet>() {
            error!("data read was not large enough to contain a single packet");
            continue;
        }

        match Packet::deserialize(buf).await {
            Ok(packet) => {
                info!("parsed packet successfully, frame id: {}", packet.frame_identifier);
            }
            Err(err) => match err {
                DeserializeUDPError::ReadError { source } => {
                    error!(
                        "encountered {source} while trying to deserialize a {} packet, length was {}",
                        buf[6],
                        buf.len()
                    );
                }
                DeserializeUDPError::ExceededValidRange { .. } => {
                    error!("{err}");
                    trace!(
                        "received {} data exc. headers from {}: {:?}",
                        buf[6],
                        addr,
                        &buf[29..len]
                    );
                }
            },
        }
    }
}
