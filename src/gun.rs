// #![no_std]
#![forbid(unsafe_code)]
#![feature(non_ascii_idents)]
#![allow(unknown_lints, uncommon_codepoints)]

use anyhow::Result;

pub mod plugins;
pub mod util;
mod dedup;
mod ham;

#[derive(Clone)]
pub struct GunFunctions {
	pub start: fn(&[&str]) -> Result<()>,
}

pub struct Gun {
	functions: GunFunctions,
}

impl Gun {
	pub fn start(&self, peers: &[&str]) -> Result<()> {
		(self.functions.start)(peers)
	}
}

pub struct GunBuilder {
	pub functions: GunFunctions,
}

impl GunBuilder {
	pub fn new() -> Self {
		Self {
			functions: GunFunctions {
				start: plugins::websockets::start,
			}
		}
	}

	pub fn build(&self) -> Gun {
		Gun {
			functions: self.functions.clone(),
		}
	}
}
