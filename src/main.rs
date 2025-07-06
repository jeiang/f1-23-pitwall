#![feature(associated_type_defaults, error_generic_member_access)]
#![feature(duration_constructors_lite)]
#![deny(clippy::all, clippy::pedantic, clippy::cargo)]
#![allow(dead_code, clippy::module_name_repetitions, clippy::multiple_crate_versions)]

use color_eyre::eyre::Result;
use tokio::net::UdpSocket;
use tracing::{debug, error, info, trace};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry, fmt};

use crate::packet::{DeserializeUDP, Packet};

pub(crate) mod packet;

// noinspection RsTraitObligations
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let global_filter = EnvFilter::try_from_default_env()?;

    // layer 1 is the file writer
    let rolling_log = RollingFileAppender::builder()
        .max_log_files(10)
        .rotation(Rotation::DAILY)
        .filename_prefix("f1-23-telemetry")
        .filename_suffix("log")
        .build(std::env::current_dir()?)?;
    let (non_blocking, _guard1) = tracing_appender::non_blocking(rolling_log);
    let layer1 = fmt::Layer::default().with_ansi(false).with_writer(non_blocking);

    // layer 2 is the stdout writer
    let (non_blocking, _guard2) = tracing_appender::non_blocking(std::io::stdout());
    let layer2 = fmt::Layer::default().with_writer(non_blocking);

    Registry::default().with(layer1).with(layer2).with(global_filter).init();
    info!("starting up: logs: {:?}", EnvFilter::try_from_default_env());
    let sock = UdpSocket::bind("0.0.0.0:22023").await?;
    let mut buf = [0; 2048];

    info!("started udp receiver");
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        let buf = &buf[0..len];
        debug!("received {len} bytes of data from {addr}");

        match Packet::deserialize(buf).await {
            Ok(packet) => {
                info!("received packet: {packet:?}");
            }
            Err(err) => {
                error!("an error occurred while parsing packet: {err}");
                trace!("raw packet data: {buf:?}");
            }
        }
    }
}
