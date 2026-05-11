// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit-jack@gmail.com>
// SPDX-License-Identifier: LGPL-3.0-or-later

//! This module focuses on upgrading old CDNZ files to the latest version.

#[derive(Debug, thiserror::Error)]
pub enum CdnzUpgradeError {
	#[error("Not yet implemented")]
	NotImplemented,
}

pub fn upgrade_json<T>(
	json: T,
	have_version: &str,
	want_version: &str,
) -> Result<String, CdnzUpgradeError>
where
	T: AsRef<[u8]> + Into<String>,
{
	if have_version == want_version {
		return Ok(json.into());
	}
	Err(CdnzUpgradeError::NotImplemented)
}
