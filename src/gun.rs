// #![no_std]
#![forbid(unsafe_code)]
#![feature(non_ascii_idents)]
#![allow(unknown_lints, uncommon_codepoints)]

use anyhow::Result;

mod dedup;
mod get;
mod ham;
mod socket;
mod util;

pub fn foobar() {}

pub async fn start() -> Result<()> {
	Ok(socket::boot_socket().await?)
}
