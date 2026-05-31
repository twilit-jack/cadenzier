// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit-jack@gmail.com>
// SPDX-License-Identifier: LGPL-3.0-or-later

use num::Rational32 as Fraction;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::BTreeMap;

// =========================== ROOT ===========================

/// The root object of a CDNZ file.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Project {
	pub cdnz: Metadata,
	pub global: GlobalData,
	pub parts: BTreeMap<PartName, Part>,
	pub layouts: BTreeMap<LayoutName, Layout>,
}

/// Metadata about the CDNZ file in question.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct GlobalData {
	pub mod_events: BTreeMap<Offset, Vec<GlobalModEvent>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Bpm {
	pub unit: Fraction,
}

// =========================== PART ===========================

pub type PartName = String;

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Part {
	pub voices: Vec<Voice>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Voice {
	pub instrument: Instrument,
	pub rhythmic_events: Vec<RhythmicEvent>,
	pub mod_events: BTreeMap<Offset, Vec<LocalModEvent>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum RhythmicEvent {
	Note {
		duration: Duration,
		pitches: Vec<Pitch>,
	},
	DrumNote {
		duration: Duration,
	},
	Rest {
		duration: Duration,
	},
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum LocalModEvent {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ClefSign {
	G,
	F,
	C,
}

// =========================== LAYOUT ===========================

pub type LayoutName = String;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Layout {
	pub header: Header,
	pub paper: PaperSettings,
	pub layout: LayoutElement,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Header {}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
#[skip_serializing_none]
pub struct PaperSettings {
	pub paper_height: Option<f64>,
	pub paper_width: Option<f64>,

	pub top_margin: Option<f64>,
	pub bottom_margin: Option<f64>,
	pub left_margin: Option<f64>,
	pub right_margin: Option<f64>,
	pub inner_margin: Option<f64>,
	pub outer_margin: Option<f64>,

	pub indent: Option<f64>,
	pub short_indent: Option<f64>,

	pub binding_offset: Option<f64>,

	/// If this is set to `true`, systems will be set at their natural spacing, neither compressed
	/// nor stretched vertically to fit the page.
	pub ragged_bottom: Option<bool>,
	/// If this is set to `false`, then the last page, and the last page in each section created
	/// with a BookPart, will be vertically justified in the same way as the earlier pages.
	pub ragged_last_bottom: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum LayoutElement {
	Staff { voices: Vec<LayoutVoice> },
	StaffGroup { children: Vec<LayoutElement> },
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LayoutVoice {
	pub mod_events: Vec<LayoutModEvent>,
	pub referenced_voice: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum LayoutModEvent {
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
	Transposition {
		pitch: Pitch,
	},
}

impl LayoutModEvent {
	/// A helper function for creating a basic treble clef.
	pub fn new_treble_clef() -> LayoutModEvent {
		LayoutModEvent::ClefChange {
			sign: ClefSign::G,
			pos: -2,
			octave: 0,
		}
	}
	/// A helper function for creating a basic bass clef.
	pub fn new_bass_clef() -> LayoutModEvent {
		LayoutModEvent::ClefChange {
			sign: ClefSign::F,
			pos: 2,
			octave: 0,
		}
	}
	/// A helper function for creating a basic alto clef.
	pub fn new_alto_clef() -> LayoutModEvent {
		LayoutModEvent::ClefChange {
			sign: ClefSign::C,
			pos: 0,
			octave: 0,
		}
	}
}

// =========================== PRIMITIVES ===========================

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration {
	/// Note type by duration (e.g. quarter note, eighth note, etc.).
	///
	/// Values correspond to a progression like this:
	/// - 0 – Whole note (semibreve)
	/// - 1 – Half note (minim)
	/// - 2 – Quarter note (crotchet)
	/// - 3 – Eighth note (quaver)
	/// - 4 – Sixteenth note (semiquaver)
	///
	/// Or in the opposite direction:
	/// - 0 – Whole note (semibreve)
	/// - -1 – Breve
	/// - -2 – Longa
	/// - -3 – Maxima
	///
	/// Or through these formulas:
	/// - `length = 2^-value`
	/// - `value = -log_2 length`
	pub base: i32,

	/// Represents the number of dots in this duration.
	///
	/// E.g.:
	/// - 0 – No dots
	/// - 1 – Dotted
	/// - 2 – Double-dotted
	pub dots: u32,
}

/// Semantic alias for `Duration`, corresponding to a LilyPond skip/spacer rest.
pub type Skip = Duration;

/// Implies "How many skips does it take to reach this, starting from the very start?"
///
/// This is used to simplify conversion to LilyPond.
///
/// As an example, a clef event near the middle of measure 7 would transpile to something like:
///
/// ``` lilypond
/// {
/// 	s1*6 s2 s16 \clef "treble"
/// }
/// ```
pub type Offset = Vec<Skip>;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Pitch {
	/// A step of the C major scale, beginning at middle C.
	///
	/// Examples:
	/// - C4 (middle C): 0
	/// - A4 (concert pitch): 5
	/// - C5: 7
	/// - C3: -7
	pub step: i32,

	/// Alteration of the note.
	pub alteration: Alteration,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub enum Instrument {
	// Basic assortment of instruments. Will be expanded as time goes.
	// Feel free to suggest your own additions in issues!

	// ==== Strings ====
	Violin,
	Viola,
	Cello,
	DoubleBass,

	Harp,

	Guitar,
	ElectricGuitar,
	BassGuitar,
	ElectricBassGuitar,

	Banjo,
	Mandolin,
	Lute,

	// ==== Woodwinds ====
	Flute,
	Piccolo,

	Oboe,
	EnglishHorn,
	Clarinet,
	BassClarinet,

	Bassoon,
	Contrabassoon,

	SopranoSaxophone,
	AltoSaxophone,
	TenorSaxophone,
	BaritoneSaxophone,

	// ==== Brass ====
	Trumpet,
	Cornet,

	FrenchHorn,

	Trombone,
	BassTrombone,

	Euphonium,
	Tuba,

	// ==== Percussion ====
	SnareDrum,
	BassDrum,
	Cymbals,
	Triangle,
	Tambourine,

	Timpani,

	Xylophone,
	Marimba,
	Glockenspiel,
	Vibraphone,

	// ==== Keyboards ====
	#[default]
	Piano,
	Harpsichord,
	Organ,
	Celesta,
	Accordion,
	Synthesizer,
}
