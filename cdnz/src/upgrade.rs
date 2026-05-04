// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit-jack@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This module focuses on upgrading old CDNZ files to the latest version.

use crate::cdnz_serde::VersionInfo;

#[derive(Debug, thiserror::Error)]
pub enum CdnzUpgradeError {
	#[error("Not yet implemented")]
	NotImplemented,
}

pub fn upgrade_json<T: AsRef<[u8]> + Into<String>>(
	json: T,
	have_version: VersionInfo,
	want_version: VersionInfo,
) -> Result<String, CdnzUpgradeError> {
	if format!("{:?}", have_version) == format!("{:?}", want_version) {
		return Ok(json.into());
	}
	Err(CdnzUpgradeError::NotImplemented)
}
