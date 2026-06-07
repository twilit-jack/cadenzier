// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit-jack@gmail.com>
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Implements CDNZ -> LilyPond capabilities.
//!
//! > WARNING: Currently a work-in-progress. API in flux. Barely functions.
//!
//! LilyPond -> CDNZ is not planned in the near future, but might be implemented much later down the
//! line.

use super::*;

use heck::{ToLowerCamelCase, ToSnakeCase};
use std::{
	fs::File,
	io::{self, BufWriter, Error, ErrorKind, Write},
};

fn indent<W: Write>(w: &mut W, level: usize) -> io::Result<()> {
	for _ in 0..level {
		w.write_all(b"\t")?;
	}
	Ok(())
}

impl Project {
	const HEADER: &str = "\
		% LilyPond file generated from CDNZ.\n\
		% If you see any issues or bugs, write an issue on CDNZ to LilyPond conversion at:\n\
		%   https://codeberg.org/twilit-jack/cadenzier/\n\
		\\version \"2.26.0\"\n\n";

	pub fn part_to_lilypond(&self, part_name: PartName) -> io::Result<File> {
		let file = File::create(part_name.to_snake_case())?;
		let mut w = BufWriter::new(file);

		let part = self.parts.get(&part_name).ok_or(Error::new(
			ErrorKind::InvalidInput,
			"Nonexistant `part_name`",
		))?;

		for voice in &part.voices {
			write!(w, "{}", Self::HEADER)?;
			write!(w, "{} = {{\n\t", part_name.to_lower_camel_case())?;
			write!(w, "}}")?;
		}

		let file = w.into_inner()?;
		Ok(file)
	}

	pub fn write_lilypond<W: Write>(&self, w: &mut W) -> io::Result<()> {
		write!(w, "{}", Self::HEADER)?;

		self.global.write_lilypond(w)?;

		for (name, part) in &self.parts {
			for (voice_i, voice) in part.voices.iter().enumerate() {
				if part.voices.len() == 1 {
					write!(w, "{name} = ")?;
				} else {
					write!(w, "{name}{voice_i} = ")?;
				};
				voice.write_lilypond(w)?;
				write!(w, "\n\n")?;
			}
		}

		for (_, layout) in &self.layouts {
			write!(w, "\\book {{\n")?;
			layout.header.write_lilypond(w, 1)?;
			layout.paper.write_lilypond(w, 1)?;
			layout.layout.write_lilypond(w, 1)?;
			write!(w, "}}\n\n")?;
		}

		Ok(())
	}
}

// =========================== GLOBAL DATA ===========================

impl GlobalData {
	pub fn write_lilypond<W: Write>(&self, w: &mut W) -> io::Result<()> {
		todo!()
	}
}

// =========================== PART ===========================

impl Voice {
	pub fn write_lilypond<W: Write>(&self, w: &mut W) -> io::Result<()> {
		todo!()
	}
}

// =========================== LAYOUT ===========================

impl Header {
	pub fn write_lilypond<W: Write>(&self, w: &mut W, level: usize) -> io::Result<()> {
		indent(w, level)?;
		write!(w, "\\header {{\n")?;
		todo!();
		indent(w, level)?;
		write!(w, "}}\n")
	}
}

impl PaperSettings {
	pub fn write_lilypond<W: Write>(&self, w: &mut W, level: usize) -> io::Result<()> {
		indent(w, level)?;
		write!(w, "\\paper {{\n")?;

		Self::write_mm(w, level + 1, self.paper_height, "paper-height")?;
		Self::write_mm(w, level + 1, self.paper_width, "paper-width")?;
		Self::write_mm(w, level + 1, self.top_margin, "top-margin")?;
		Self::write_mm(w, level + 1, self.bottom_margin, "bottom-margin")?;
		Self::write_mm(w, level + 1, self.left_margin, "left-margin")?;
		Self::write_mm(w, level + 1, self.right_margin, "right-margin")?;
		Self::write_mm(w, level + 1, self.inner_margin, "inner-margin")?;
		Self::write_mm(w, level + 1, self.outer_margin, "outer-margin")?;
		Self::write_mm(w, level + 1, self.indent, "indent")?;
		Self::write_mm(w, level + 1, self.short_indent, "short-indent")?;
		Self::write_mm(w, level + 1, self.binding_offset, "binding-offset")?;
		Self::write_bool(w, level + 1, self.ragged_bottom, "ragged-bottom")?;
		Self::write_bool(w, level + 1, self.ragged_last_bottom, "ragged-last-bottom")?;

		indent(w, level)?;
		write!(w, "}}\n")
	}

	fn write_mm<W: Write>(
		w: &mut W,
		level: usize,
		field: Option<f64>,
		name: &str,
	) -> io::Result<()> {
		if let Some(val) = field {
			indent(w, level)?;
			write!(w, "{name} = {val}\\mm\n")?;
		}
		Ok(())
	}

	fn write_bool<W: Write>(
		w: &mut W,
		level: usize,
		field: Option<bool>,
		name: &str,
	) -> io::Result<()> {
		if let Some(val) = field {
			let val_str = if val { "##t" } else { "##f" };
			indent(w, level)?;
			write!(w, "{name} = {val_str}\n")?;
		}
		Ok(())
	}
}

impl LayoutElement {
	pub fn write_lilypond<W: Write>(&self, w: &mut W, level: usize) -> io::Result<()> {
		Ok(())
	}
}
