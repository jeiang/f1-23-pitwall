#![feature(associated_type_defaults, error_generic_member_access)]
#![feature(duration_constructors)]
#![deny(clippy::all, clippy::pedantic, clippy::cargo)]
#![allow(dead_code, clippy::module_name_repetitions)]

use color_eyre::eyre::Result;
use tokio::net::UdpSocket;
use tracing::{
    debug,
    error,
    info,
    trace,
    Level,
};

use crate::packet::{
    DeserializeUDP,
    Packet,
};

pub(crate) mod packet;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    // TODO: configure
    let subscriber = tracing_subscriber::fmt().with_max_level(Level::DEBUG).finish();
    tracing::subscriber::set_global_default(subscriber)?;
    let sock = UdpSocket::bind("0.0.0.0:22023").await?;
    let mut buf = [0; 2048];

    info!("started udp receiver");
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        let buf = &buf[0..len];
        debug!("received {len} bytes of data from {addr}");

        match Packet::deserialize(buf).await {
            Ok(packet) => {
                debug!("received packet: {packet:?}");
            }
            Err(err) => {
                error!("an error occurred while parsing packet: {err}");
                trace!("raw packet data: {buf:?}");
            }
        }
    }
}
