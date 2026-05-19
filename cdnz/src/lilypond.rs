// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit-jack@gmail.com>
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

use std::io::{self, Write};

impl Project {
	pub fn write_lilypond<W: Write>(&self, w: &mut W) -> io::Result<()> {
		write!(w, "\\version \"2.26.0\"\n\n")
	}
}
