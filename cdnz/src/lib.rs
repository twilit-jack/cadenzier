// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit-jack@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

#![doc = include_str!("../README.md")]

pub mod cdnz_serde;
pub mod lilypond;
pub mod upgrade;

pub use cdnz_serde::VersionInfo;

use serde_with::{DisplayFromStr, serde_as, skip_serializing_none};

use std::collections::{BTreeMap, HashMap};

use num::Rational32;

use serde::{Deserialize, Serialize};

// =========================== ROOT ===========================

/// The root object of a CDNZ file.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Cdnz {
	pub cdnz: Metadata,
	pub global: GlobalData,
	pub parts: HashMap<String, Part>,
	pub books: Vec<Book>,
}

impl ToString for Cdnz {
	fn to_string(&self) -> String {
		format!("{:#?}", self)
	}
}

/// Metadata about the CDNZ file in question.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Metadata {
	/// The name/title for the music.
	///
	/// This is separate from the file name, because file names are ambiguous, and are often
	/// discouraged from including certain special/Unicode characters, including spaces.
	///
	/// This does not have to be the same as the title in the header of the books, but it's highly
	/// encouraged. It could be different in, for example, books with text in a different language.
	pub score_title: String,

	/// Info about the composer of the music.
	///
	/// The composer is the author of the base melody. This might also be the name of a folk, for
	/// rhapsodies or folk song arrangements.
	pub composer: PersonInfo,
	/// Info about the arranger of the music.
	///
	/// The arranger is the author of the accompanying bass and secondary melodies. This might be
	/// the same person as the composer for original works.
	pub arranger: PersonInfo,
	/// Info about the engraver of the music.
	///
	/// The engraver is the author of the exact layout and encoding of the music, i.e. the CDNZ file.
	/// This might be the same as the composer or arranger if they're doing their first work via
	/// CDNZ or Cadenza.
	pub engraver: PersonInfo,

	/// A description of the piece, typically given by the composer, including thoughts about the
	/// piece.
	pub description: String,

	/// A SPDX tag describing the licensing of the music/arrangement.
	///
	/// Can also be `Public-Domain`, which is used for expired copyright works (e.g. baroque,
	/// classical, romance works).
	pub music_license: String,
	/// A SPDX tag describing the licensing of the engraving/encoding.
	///
	/// If you want to waive your copyright, it's recommended to use `CC0-1.0` here, instead of
	/// `Public-Domain`.
	pub engraving_license: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[skip_serializing_none]
pub struct PersonInfo {
	/// The name of this person or entity.
	///
	/// See `is_person` for more info about "entity".
	pub name: String,

	/// The email through which this person or entity can be reached.
	///
	/// This is an `Option`, because some works might be composed by a historical composer, or the
	/// melody might be of a folk.
	pub email: Option<String>,

	/// Whether or not this info is about a person.
	///
	/// This may be false for bands, companies or a folk, in which case `name` doesn't mean the name
	/// of a person.
	pub is_person: bool,
}

// =========================== GLOBAL DATA ===========================

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GlobalData {
	#[serde_as(as = "BTreeMap<DisplayFromStr, _>")]
	pub modifier_events: BTreeMap<Position, Vec<GlobalModEvent>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GlobalModEvent {
	KeyChange {
		/// The note this key change references.
		note: Pitch,
		/// The mode this key change uses, in reference to the note e.g. `Major`.
		mode: KeyMode,
	},

	/// A TimeChange defines a time signature change, changing the beat structure.
	TimeChange {
		/// The number on the top of the key signature.
		count: u16,
		/// The number on the bottom of the key signature.
		unit: u16,
	},

	/// A TempoChange defines a change of tempo, usually a BPM change, but it can
	/// also define a rhythm, e.g. swing.
	TempoChange {
		/// The BPM value this TempoChange changes it to.
		///
		/// While you might not want to always display it, it's still required. See `display_tempo`
		/// for hiding the numeric value.
		bpm: Bpm,

		/// Whether to display the new BPM as a metronome mark or to hide it.
		display_tempo: bool,

		/// What text label to have beside the metronome mark or standalone.
		///
		/// e.g. "Allegro", "Moderato".
		label: Option<String>,
	},
}

#[derive(Debug, Serialize, Deserialize)]
pub enum KeyMode {
	Major,
	Minor,

	// Currently disabled due to "do I need this?".
	// Might be changed in the future, I don't know a lot about church modes myself tbh.
	//Ionian,
	//Aeolian,
	Dorian,
	Phrygian,
	Lydian,
	Mixolydian,
	Locrian,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bpm {
	pub unit: Rational32,
}

// =========================== PART ===========================

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Part {
	#[serde_as(as = "BTreeMap<DisplayFromStr, _>")]
	pub rhythmic_events: BTreeMap<Position, RhythmicEvent>,

	#[serde_as(as = "BTreeMap<DisplayFromStr, _>")]
	pub modifier_events: BTreeMap<Position, Vec<LocalModEvent>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RhythmicEvent {
	Note { pitch: Pitch, duration: Duration },
	Rest { duration: Duration },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LocalModEvent {
	ClefChange {
		sign: ClefSign,

		/// The staff position of the clef, in increments of half a staff line, starting at the
		/// middle line.
		///
		/// Examples:
		/// - Treble: -2
		/// - Bass: 2
		/// - Alto: 0
		pos: i32,

		/// The octave transposition for a clef.
		///
		/// Usually 0, but changes for things like a "treble_8" clef (where it would be `-8`).
		octave: i32,
	},
}

impl LocalModEvent {
	/// A helper function for creating a basic treble clef.
	pub fn new_treble_clef() -> LocalModEvent {
		LocalModEvent::ClefChange {
			sign: ClefSign::G,
			pos: -2,
			octave: 0,
		}
	}
	/// A helper function for creating a basic bass clef.
	pub fn new_bass_clef() -> LocalModEvent {
		LocalModEvent::ClefChange {
			sign: ClefSign::F,
			pos: 2,
			octave: 0,
		}
	}
	/// A helper function for creating a basic alto clef.
	pub fn new_alto_clef() -> LocalModEvent {
		LocalModEvent::ClefChange {
			sign: ClefSign::C,
			pos: 0,
			octave: 0,
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClefSign {
	G,
	F,
	C,
}

// =========================== BOOK ===========================

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
	pub label: String,
	pub header: Header,
	pub layout: Layout,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {}

#[derive(Debug, Serialize, Deserialize)]
pub enum Layout {
	Staff(Staff),
	StaffGroup(StaffGroup),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Staff {
	/// Contains string keys linking to the parts in the root CDNZ struct.
	pub parts: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StaffGroup {
	pub children: Vec<Layout>,
}

// =========================== PRIMITIVES ===========================

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
	/// The measure index this position is in/is relative to.
	pub measure: u32,

	/// The position in the measure as a rational.
	///
	/// (0, 1) would be the start of the measure, (1, 2) – halfway through.
	pub pos: Rational32,

	pub grace_index: u32,
}

impl std::fmt::Display for Position {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}:{}:{}", self.measure, self.pos, self.grace_index)
	}
}

impl std::str::FromStr for Position {
	type Err = String;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts: Vec<&str> = s.split(':').collect();

		if parts.len() != 3 {
			return Err("Format must be measure:pos:grace".to_string());
		}

		let measure = parts[0]
			.parse::<u32>()
			.map_err(|_| "Invalid measure index")?;

		let pos = parts[1]
			.parse::<Rational32>()
			.map_err(|_| "Invalid rational position")?;

		let grace_index = parts[2].parse::<u32>().map_err(|_| "Invalid grace index")?;

		Ok(Position {
			measure,
			pos,
			grace_index,
		})
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pitch {
	/// A whole step, beginning at middle C.
	///
	/// Examples:
	/// - C4 (middle C): 0
	/// - A4 (concert pitch): 5
	/// - C5: 7
	/// - C3: -7
	pub step: i32,

	/// Alteration of the note, with 1/1 being a whole tone.
	///
	/// Examples:
	/// - Natural: (0, 1)
	/// - Sharp: Some((1, ))
	pub alteration: Rational32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Duration {
	/// The `log₂(x)` of the base duration of the note.
	///
	/// Examples:
	/// - Whole note: 0
	/// - Half note: 1
	/// - Quarter note: 2
	/// - Eighth note: 3
	/// - Breve: -1
	pub base: i16,

	/// The number of dots on this note.
	///
	/// Examples:
	/// - Not dotted: 0
	/// - Dotted: 1
	/// - Double-dotted: 2,
	pub dots: u16,
}
