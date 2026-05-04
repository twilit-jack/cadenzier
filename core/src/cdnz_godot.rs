// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit-jack@gmail.com>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;

use cdnz::cdnz_serde::CdnzDeError;
use godot::{classes::Json, global::Error, prelude::*};

/// A global class providing access to a few features from the `cdnz` crate in Godot.
#[derive(GodotClass)]
#[class(init, base=RefCounted)]
pub struct CDNZ {}

#[godot_api]
impl CDNZ {
	/// Loads a CDNZ or CDNX file into a Dictionary.
	///
	/// Returns a value from the `Error` enum if an operation failed.
	#[func]
	fn load_from_file(path: String) -> Variant {
		let Ok(bytes) = fs::read(path) else {
			return Error::ERR_FILE_NOT_FOUND.to_variant();
		};
		let json_string = match cdnz::Cdnz::deserialize_json(&bytes[..]) {
			Ok(string) => string,
			Err(CdnzDeError::IoError(_) | CdnzDeError::UpgradeError(_)) => {
				return Error::FAILED.to_variant();
			}
			Err(
				CdnzDeError::SerdeError(_)
				| CdnzDeError::MissingOrEmptyDataJsonZst
				| CdnzDeError::MissingOrEmptyDataJson
				| CdnzDeError::MissingOrIncorrectMimetype,
			) => return Error::ERR_FILE_CORRUPT.to_variant(),
		};
		Json::parse_string(&json_string)
	}

	/// Checks the integrity of a CDNZ Dictionary structure (e.g. when I did an oopsie), returning a
	/// value from the `Error` enum.
	#[func]
	fn validate(dict: VarDictionary) -> Error {
		let json_string = Json::stringify(&dict.to_variant());

		match cdnz::Cdnz::validate(json_string.to_string().as_bytes()) {
			Ok(()) => Error::OK,
			Err(CdnzDeError::IoError(_) | CdnzDeError::UpgradeError(_)) => Error::FAILED,
			Err(
				CdnzDeError::SerdeError(_)
				| CdnzDeError::MissingOrEmptyDataJsonZst
				| CdnzDeError::MissingOrEmptyDataJson
				| CdnzDeError::MissingOrIncorrectMimetype,
			) => Error::ERR_FILE_CORRUPT,
		}
	}

	/// Serializes CDNZ Dictionary structure to tarball.
	#[func]
	fn serialize(dict: VarDictionary) -> Variant {
		let json_string = Json::stringify(&dict.to_variant());

		let cdnz = match cdnz::Cdnz::deserialize(json_string.to_string().as_bytes()) {
			Ok(cdnz) => cdnz,
			Err(CdnzDeError::IoError(_) | CdnzDeError::UpgradeError(_)) => {
				return Error::FAILED.to_variant();
			}
			Err(
				CdnzDeError::SerdeError(_)
				| CdnzDeError::MissingOrEmptyDataJsonZst
				| CdnzDeError::MissingOrEmptyDataJson
				| CdnzDeError::MissingOrIncorrectMimetype,
			) => return Error::ERR_FILE_CORRUPT.to_variant(),
		};

		let Ok(serialized) = cdnz.serialize() else {
			return Error::FAILED.to_variant();
		};

		PackedByteArray::from(serialized).to_variant()
	}
}
