#![deny(clippy::all, clippy::pedantic, clippy::cargo)]
// TODO: remove
#![allow(clippy::missing_docs_in_private_items, dead_code)]

mod api;

fn main() {
    let _sample: api::packet::Packet;
    println!("Hello, world!");
}
