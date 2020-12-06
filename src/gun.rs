// #![no_std]
#![forbid(unsafe_code)]
#![feature(non_ascii_idents)]
#![allow(unknown_lints, uncommon_codepoints)]

use anyhow::Result;

pub mod plugins;
mod dedup;
mod ham;
mod socket;
mod util;

#[derive(Clone)]
pub struct GunFunctions {
	pub start: fn() -> Result<()>,
}

pub struct Gun {
	functions: GunFunctions,
}

impl Gun {
	pub fn start(&self) -> Result<()> {
		(self.functions.start)()
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
