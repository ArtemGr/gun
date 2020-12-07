#![cfg_attr(not(feature = "std"), no_std)]
#![feature(non_ascii_idents)]
#![allow(unknown_lints, uncommon_codepoints)]

use anyhow::Result;

pub mod plugins;
pub mod util;
mod dedup;
mod get;
mod ham;

#[cfg(feature = "std")]
use std::rc::Rc;

use crate::{
	get::GunGet,
	plugins::GunPlugin,
};

#[derive(Clone)]
pub struct GunOptions<'a> {
	pub peers: &'a [&'a str],
	pub radisk: bool,
	pub local_storage: bool,
	pub uuid: fn() -> String,
	pub port: u16,
}

impl Default for GunOptions<'_> {
	fn default() -> Self {
		Self {
			peers: &[],
			radisk: true,
			local_storage: true,
			#[cfg(feature = "default-uuid")]
			uuid: util::uuid,
			#[cfg(not(feature = "default-uuid"))]
			uuid: || "".to_owned(),
			port: 8080,
		}
	}
}

#[derive(Clone)]
pub struct GunBuilder<'a> {
	#[cfg(feature = "std")]
	pub plugin: Rc<dyn GunPlugin + 'a>,
	pub options: GunOptions<'a>,
}

impl<'a> GunBuilder<'a> {
	pub fn new() -> Self {
		Self {
			plugin: Rc::new(plugins::tungstenite::Tungstenite::new()),
			options: GunOptions::default(),
		}
	}

	pub fn new_with_options(options: GunOptions<'a>) -> Self {
		Self {
			plugin: Rc::new(plugins::tungstenite::Tungstenite::new()),
			options,
		}
	}

	pub fn peers(&self, peers: &'a [&str]) -> Self {
		let mut gun = self.clone();
		gun.options.peers = peers;
		gun
	}

	pub fn radisk(&self, radisk: bool) -> Self {
		let mut gun = self.clone();
		gun.options.radisk = radisk;
		gun
	}

	pub fn local_storage(&self, local_storage: bool) -> Self {
		let mut gun = self.clone();
		gun.options.local_storage = local_storage;
		gun
	}

	pub fn uuid(&self, uuid: fn() -> String) -> Self {
		let mut gun = self.clone();
		gun.options.uuid = uuid;
		gun
	}

	pub fn port(&self, port: u16) -> Self {
		let mut gun = self.clone();
		gun.options.port = port;
		gun
	}

	pub fn build(&self) -> Gun {
		let gun = Gun {
			plugin: self.plugin.clone(),
			options: self.options.clone(),
		};

		drop(self);

		gun
	}
}

pub struct Gun<'a> {
	#[cfg(feature = "std")]
	plugin: Rc<dyn GunPlugin + 'a>,
	options: GunOptions<'a>,
}

impl<'a> Gun<'a> {
	pub fn start(&self) -> Result<()> {
		self.plugin.start(&self.options)
	}

	pub fn opt(&mut self, options: GunOptions<'a>) {
		self.options = options;
	}

	pub fn get(&self, key: &'a str) -> GunGet<'a> {
		GunGet::new(self.plugin.clone(), key)
	}

	pub fn block(&self) {
		loop {}
	}
}
