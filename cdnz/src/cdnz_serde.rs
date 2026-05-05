// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit-jack@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::upgrade::CdnzUpgradeError;

use super::*;

use std::io::{self, Read};

use tar::{Archive, Builder, Header};

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionInfo<'a> {
	pub cdnz_version: &'a str,
	pub cadenza_version: &'a str,
}

const CURRENT_VERSION: VersionInfo = VersionInfo {
	cdnz_version: "0.1.0",
	cadenza_version: env!("CARGO_PKG_VERSION"),
};

#[derive(Debug, thiserror::Error)]
pub enum CdnzSerError {
	#[error("IO error: {0}")]
	IoError(#[from] io::Error),

	#[error("Serde error: {0}")]
	SerdeError(#[from] serde_json::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum CdnzDeError {
	#[error("IO error: {0}")]
	IoError(#[from] io::Error),

	#[error("Serde error: {0}")]
	SerdeError(#[from] serde_json::Error),

	#[error("Error upgrading file: {0}")]
	UpgradeError(#[from] CdnzUpgradeError),

	#[error("Missing or empty `data.json.zst` for CDNZ file")]
	MissingOrEmptyDataJsonZst,

	#[error("Missing or empty `data.json` for CDNX file")]
	MissingOrEmptyDataJson,

	#[error("Missing or incorrect `mimetype`")]
	MissingOrIncorrectMimetype,
}

impl Cdnz {
	/// Serializes `Cdnz` struct to main tarball format, returning tarball bytes.
	pub fn serialize(&self) -> Result<Vec<u8>, CdnzSerError> {
		let mut buffer = Vec::new();
		{
			let mut tar = Builder::new(&mut buffer);

			// Write `data.json.zst`
			let data_json = serde_json::to_string_pretty(self)?;
			let data_json_zstd = zstd::encode_all(data_json.as_bytes(), 0)?;
			let mut header = Header::new_gnu();
			header.set_path("data.json.zst")?;
			header.set_size(data_json_zstd.len() as u64);
			header.set_cksum();
			tar.append_data(&mut header, "data.json.zst", &data_json_zstd[..])?;

			// Write `version.json`
			let version_json = serde_json::to_string_pretty(&CURRENT_VERSION)?;
			let mut header = Header::new_gnu();
			header.set_path("version.json")?;
			header.set_size(version_json.len() as u64);
			header.set_cksum();
			tar.append_data(&mut header, "version.json", version_json.as_bytes())?;

			// Write `mimetype`
			let mimetype = "application/vnd.cadenza.cdnz";
			let mut header = Header::new_gnu();
			header.set_path("mimetype")?;
			header.set_size(mimetype.len() as u64);
			header.set_cksum();
			tar.append_data(&mut header, "mimetype", mimetype.as_bytes())?;

			tar.finish()?;
		}
		Ok(buffer)
	}

	/// Serializes `Cdnz` struct to `.cdnx` tarball format, returning tarball bytes.
	///
	/// `.cdnx` features an uncompressed `data.json` file.
	pub fn serialize_no_compress(&self) -> Result<Vec<u8>, CdnzSerError> {
		let mut buffer = Vec::new();
		{
			let mut tar = Builder::new(&mut buffer);

			// Write `data.json`
			let data_json = serde_json::to_string_pretty(self)?;
			let mut header = Header::new_gnu();
			header.set_path("data.json")?;
			header.set_size(data_json.len() as u64);
			header.set_cksum();
			tar.append_data(&mut header, "data.json", data_json.as_bytes())?;

			// Write `version.json`
			let version_json = serde_json::to_string_pretty(&CURRENT_VERSION)?;
			let mut header = Header::new_gnu();
			header.set_path("version.json")?;
			header.set_size(version_json.len() as u64);
			header.set_cksum();
			tar.append_data(&mut header, "version.json", version_json.as_bytes())?;

			// Write `mimetype`
			let mimetype = "application/vnd.cadenza.cdnz";
			let mut header = Header::new_gnu();
			header.set_path("mimetype")?;
			header.set_size(mimetype.len() as u64);
			header.set_cksum();
			tar.append_data(&mut header, "mimetype", mimetype.as_bytes())?;

			tar.finish()?;
		}
		Ok(buffer)
	}

	/// Serializes `Cdnz` struct to only JSON.
	///
	/// Mainly used as a small helper/wrapper for `cadenza_core`.
	pub fn serialize_json(&self) -> Result<String, CdnzSerError> {
		Ok(serde_json::to_string_pretty(self)?)
	}

	/// Deserializes from zstd-compressed CDNZ tarball.
	pub fn deserialize<R: Read>(reader: R) -> Result<Self, CdnzDeError> {
		let data_json = Cdnz::deserialize_json(reader)?;
		Ok(serde_json::from_str(&data_json)?)
	}

	/// Extracts the JSON data file from a CDNZ or CDNX file.
	///
	/// INFO: Also performs needed upgrades to the format.
	pub fn deserialize_json<R: Read>(reader: R) -> Result<String, CdnzDeError> {
		let mut archive = Archive::new(reader);

		// Define targets
		let mut version_json = String::new();
		let mut mimetype = String::new();
		let mut data_json = String::new();
		let mut data_json_zst: Vec<u8> = Vec::new();

		for entry in archive.entries()? {
			let mut entry = entry?;
			let path = entry.path()?;
			let path_str = path.to_str().unwrap_or("");

			match path_str {
				"mimetype" => {
					entry.read_to_string(&mut mimetype)?;
				}
				"version.json" => {
					entry.read_to_string(&mut version_json)?;
				}
				"data.json" => {
					entry.read_to_string(&mut data_json)?;
				}
				"data.json.zst" => {
					entry.read_to_end(&mut data_json_zst)?;
				} // Read as binary
				_ => continue,
			}
		}

		// Match file type
		if mimetype.trim() == "application/vnd.cadenza.cdnz" {
			if data_json_zst.is_empty() {
				return Err(CdnzDeError::MissingOrEmptyDataJsonZst);
			}
			let decompressed = zstd::decode_all(&data_json_zst[..])?;
			data_json = String::from_utf8(decompressed)
				.map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "JSON not UTF-8"))?;
		} else if mimetype.trim() == "application/vnd.cadenza.cdnx" {
			if data_json == "" {
				return Err(CdnzDeError::MissingOrEmptyDataJson);
			}
		} else {
			return Err(CdnzDeError::MissingOrIncorrectMimetype);
		}

		// Upgrade if needed
		let version_info: VersionInfo = serde_json::from_str(&version_json)?;
		upgrade::upgrade_json(&data_json, version_info, CURRENT_VERSION)?;

		Ok(data_json)
	}

	/// Validates a CDNZ tarball.
	///
	/// INFO: Considers upgradable tarballs as valid.
	pub fn validate<R: Read>(reader: R) -> Result<(), CdnzDeError> {
		let mut archive = Archive::new(reader);

		// Define targets
		let mut data_json = String::new();
		let mut data_json_zst = String::new();
		let mut version_json = String::new();
		let mut mimetype = String::new();

		for entry in archive.entries()? {
			let mut entry = entry?;

			let target = match entry.path()?.to_str() {
				Some("mimetype") => &mut mimetype,
				Some("version.json") => &mut version_json,
				Some("data.json") => &mut data_json,
				Some("data.json.zst") => &mut data_json_zst,
				// This is for allowing extra files, e.g. vendor extensions or future additions.
				_ => continue,
			};
			entry.read_to_string(target)?;
		}

		// Match file type
		if mimetype.trim() == "application/vnd.cadenza.cdnz" {
			if data_json_zst == "" {
				return Err(CdnzDeError::MissingOrEmptyDataJsonZst);
			}
			// Decompress and carry on
			let decompressed = zstd::decode_all(data_json_zst.as_bytes())?;
			data_json = String::from_utf8_lossy(&decompressed).to_string();
		} else if mimetype.trim() == "application/vnd.cadenza.cdnx" {
			if data_json == "" {
				return Err(CdnzDeError::MissingOrEmptyDataJson);
			}
		} else {
			return Err(CdnzDeError::MissingOrIncorrectMimetype);
		}

		// Upgrade if needed
		let version_info: VersionInfo = serde_json::from_str(&version_json)?;
		upgrade::upgrade_json(&data_json, version_info, CURRENT_VERSION)?;

		Ok(())
	}
}
