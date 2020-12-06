// #![no_std]
#![feature(non_ascii_idents)]
#![allow(unknown_lints, uncommon_codepoints)]

use anyhow::Result;

pub mod plugins;
pub mod util;
mod dedup;
mod ham;

use crate::util::Str;

#[derive(Clone)]
pub struct GunFunctions {
	pub start: fn(&[&str]) -> Result<()>,
}

#[derive(Clone)]
pub struct GunOptions<'a> {
	pub peers: &'a [&'a str],
	pub radisk: bool,
	pub local_storage: bool,
	pub uuid: fn() -> Str,
}

#[derive(Clone)]
pub struct GunBuilder<'a> {
	pub functions: GunFunctions,
	pub options: GunOptions<'a>,
}

impl<'a> GunBuilder<'a> {
	pub fn new() -> Self {
		Self {
			functions: GunFunctions {
				start: plugins::websockets::start,
			},
			options: GunOptions {
				peers: &[],
				radisk: true,
				local_storage: true,
				uuid: util::uuid,
			},
		}
	}

	pub fn new_with_options(options: GunOptions<'a>) -> Self {
		Self {
			functions: GunFunctions {
				start: plugins::websockets::start,
			},
			options,
		}
	}

	pub fn peers(&mut self, peers: &'a [&str]) -> Self {
		self.options.peers = peers;
		self.clone()
	}

	pub fn radisk(&mut self, radisk: bool) -> Self {
		self.options.radisk = radisk;
		self.clone()
	}

	pub fn local_storage(&mut self, local_storage: bool) -> Self {
		self.options.local_storage = local_storage;
		self.clone()
	}

	pub fn uuid(&mut self, uuid: fn() -> Str) -> Self {
		self.options.uuid = uuid;
		self.clone()
	}

	pub fn build(&self) -> Gun {
		let gun = Gun {
			functions: self.functions.clone(),
			options: self.options.clone(),
		};

		drop(self);

		gun
	}
}

pub struct Gun<'a> {
	functions: GunFunctions,
	options: GunOptions<'a>,
}

impl Gun<'_> {
	pub fn start(&self) -> Result<()> {
		(self.functions.start)(self.options.peers)
	}
}
