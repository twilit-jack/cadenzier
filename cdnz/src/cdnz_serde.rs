// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit-jack@gmail.com>
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Implements serialization and deserialization functions for `cdnz::Project`.
//!
//! Typically, for serialization you'll want the `to_cdnz()` method, and for deserialization – the
//! `from_bytes()` method.
//!
//! Internally, this makes use of CBOR and Zstd to achieve a small footprint on disk. In the base
//! compresed version (CDNZ), 4 magic bytes `b"CDNZ"` are added to the start of the file, then the
//! `Project` gets serialized to CBOR, then encoded via Zstd, and placed in a container struct
//! (`CborContainer`), which also includes version info, and then serialized to CBOR again, which is
//! added after the magic bytes.
//!
//! The uncompressed version (CDNX) uses `b"CDNX"` as the magic bytes, and it doesn't compress the
//! serialized `Project` (internally called the "payload").
//!
//! This style of composability means that the magic bytes and version string are still accessible
//! without decompressing the main Zstd-compressed data.

use super::*;
use crate::upgrade::CdnzUpgradeError;

use std::io::{self, Read};

const CURRENT_VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, thiserror::Error)]
pub enum CdnzDeError {
	#[error("IO error: {0}")]
	IoError(#[from] io::Error),
	#[error("Error upgrading file: {0}")]
	UpgradeError(#[from] CdnzUpgradeError),
}

#[derive(Serialize, Deserialize)]
struct CborContainer {
	version: String,
	#[serde(with = "serde_bytes")]
	payload: Vec<u8>,
}

impl Project {
	// ======== SERIALIZE ========

	/// Serialize to standard compressed format (CDNZ).
	pub fn to_cdnz(&self) -> Result<Vec<u8>, io::Error> {
		self.serialize(true)
	}

	/// Serialize to uncompressed variant (CDNX).
	pub fn to_cdnx(&self) -> Result<Vec<u8>, io::Error> {
		self.serialize(false)
	}

	fn serialize(&self, compress: bool) -> Result<Vec<u8>, io::Error> {
		let magic = if compress { b"CDNZ" } else { b"CDNX" };

		let mut inner_bytes = Vec::new();
		ciborium::ser::into_writer(self, &mut inner_bytes)
			.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

		let payload = if compress {
			zstd::encode_all(&inner_bytes[..], 3)?
		} else {
			inner_bytes
		};

		let container = CborContainer {
			version: CURRENT_VERSION.to_string(),
			payload,
		};

		let mut output = Vec::from(*magic);
		ciborium::ser::into_writer(&container, &mut output)
			.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

		Ok(output)
	}

	// ======== DESERIALIZE ========

	/// Decodes a byte array serialized of CDNZ or CDNX data into a `Project`.
	pub fn from_bytes(data: &[u8]) -> Result<Self, CdnzDeError> {
		let reader = std::io::Cursor::new(data);
		Self::from_reader(reader)
	}

	/// Decodes a serialized CDNZ or CDNX data stream into a `Project`.
	pub fn from_reader<R: Read>(mut reader: R) -> Result<Self, CdnzDeError> {
		let mut magic = [0u8; 4];
		reader.read_exact(&mut magic)?;

		let is_compressed = match &magic {
			b"CDNZ" => true,
			b"CDNX" => false,
			_ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid Magic").into()),
		};

		let container: CborContainer = ciborium::de::from_reader(reader)
			.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

		let inner_cbor_bytes = if is_compressed {
			zstd::decode_all(&container.payload[..])?
		} else {
			container.payload
		};

		// TODO: Pass version to upgrade logic before final decode if structure changed
		let project: Project = ciborium::de::from_reader(&inner_cbor_bytes[..])
			.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

		Ok(project)
	}
}
