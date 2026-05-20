// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
	fs::{File, create_dir_all},
	io::{Error, ErrorKind, Read, Write},
};

// ======== DEFS ========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {}

// ======== DEFAULT ========

impl Default for Config {
	fn default() -> Self {
		Self {}
	}
}

// ======== SAVE & LOAD ========

#[derive(Debug, Serialize, Deserialize)]
struct CborContainer {
	version: String,
	config: Config,
}

const MAGIC_BYTES: &[u8; 4] = b"Conf";

impl Config {
	pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
		let container = CborContainer {
			version: env!("CARGO_PKG_VERSION").to_string(),
			config: self.clone(),
		};

		let mut output = Vec::from(*MAGIC_BYTES);
		ciborium::ser::into_writer(&container, &mut output)
			.map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

		Ok(output)
	}

	pub fn from_bytes(data: &[u8]) -> Result<Self, Error> {
		let reader = std::io::Cursor::new(data);
		Self::from_reader(reader)
	}

	pub fn from_reader<R: Read>(mut reader: R) -> Result<Self, Error> {
		let mut magic = [0u8; 4];
		reader.read_exact(&mut magic)?;

		if &magic != MAGIC_BYTES {
			return Err(Error::new(ErrorKind::InvalidData, "Invalid Magic").into());
		}

		let container: CborContainer = ciborium::de::from_reader(reader)
			.map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

		if container.version != env!("CARGO_PKG_VERSION") {
			return Err(Error::new(ErrorKind::InvalidData, "Outdated Version").into());
		}

		Ok(container.config)
	}
}

impl Config {
	pub fn save_to_disk(&self) -> Result<(), Error> {
		let bytes = self.to_bytes()?;

		let proj_dirs = ProjectDirs::from("org", "twilit-jack", "Cadenza")
			.ok_or(Error::new(ErrorKind::NotFound, "Home directory not found"))?;
		let dir = proj_dirs.config_dir();
		create_dir_all(&dir)?;
		let path = dir.join("config.cbor");

		let mut file = File::create(path)?;
		file.write_all(&bytes)?;

		Ok(())
	}

	pub fn load_from_disk() -> Result<Self, Error> {
		let proj_dirs = ProjectDirs::from("org", "twilit-jack", "Cadenza")
			.ok_or(Error::new(ErrorKind::NotFound, "Home directory not found"))?;
		let dir = proj_dirs.config_dir();
		create_dir_all(&dir)?;
		let path = dir.join("config.cbor");

		let file = File::open(path)?;
		Self::from_reader(file)
	}
}
