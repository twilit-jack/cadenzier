// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit-jack@gmail.com>
// SPDX-License-Identifier: LGPL-3.0-or-later

use cdnz::*;
use num::Rational32 as Fraction;
use std::{fs, path::PathBuf};

fn create_beethoven_project() -> Project {
	Project {
		cdnz: Metadata {
			score_title: "Symphony No. 5".into(),

			composer: PersonInfo {
				name: "Ludwig van Beethoven".into(),
				email: None,
				is_person: true,
			},
			arranger: PersonInfo {
				name: "Twilit Jack".into(),
				email: Some("twilit.jack@proton.me".into()),
				is_person: true,
			},
			engraver: PersonInfo {
				name: "Twilit Jack".into(),
				email: Some("twilit.jack@proton.me".into()),
				is_person: true,
			},

			description: "Symphony No. 5 in C minor of Ludwig van Beethoven, Op. 67, written \
				between 1804 and 1808.\n\n\
				It is one of the best-known compositions in classical music."
				.into(),

			music_license: "Public-Domain".into(),
			engraving_license: "CC0-1.0".into(),
		},

		global: GlobalData {
			mod_events: [(
				[].into(),
				[
					GlobalModEvent::KeyChange {
						note: Pitch {
							step: PitchStep::C,
							octave: 0,
							alteration: Alteration::Natural,
						},
						mode: KeyMode::Minor,
					},
					GlobalModEvent::TimeChange { count: 2, unit: 4 },
				]
				.into(),
			)]
			.into(),
		},

		parts: [(
			"Piano".into(),
			Part {
				voices: [Voice {
					instrument: Instrument::Piano,
					rhythmic_events: [
						RhythmicEvent::Rest {
							duration: Duration { base: 3, dots: 0 },
						},
						RhythmicEvent::Note {
							duration: Duration { base: 3, dots: 0 },
							pitches: [Pitch {
								step: PitchStep::G,
								octave: 1,
								alteration: Alteration::Natural,
							}]
							.into(),
						},
						RhythmicEvent::Note {
							duration: Duration { base: 2, dots: 0 },
							pitches: [Pitch {
								step: PitchStep::G,
								octave: 1,
								alteration: Alteration::Natural,
							}]
							.into(),
						},
						RhythmicEvent::Note {
							duration: Duration { base: 2, dots: 0 },
							pitches: [Pitch {
								step: PitchStep::G,
								octave: 1,
								alteration: Alteration::Natural,
							}]
							.into(),
						},
						// ---
						RhythmicEvent::Note {
							duration: Duration { base: 0, dots: 0 },
							pitches: [Pitch {
								step: PitchStep::E,
								octave: 1,
								alteration: Alteration::Flat,
							}]
							.into(),
						},
						// ---
						RhythmicEvent::Rest {
							duration: Duration { base: 3, dots: 0 },
						},
						RhythmicEvent::Note {
							duration: Duration { base: 3, dots: 0 },
							pitches: [Pitch {
								step: PitchStep::F,
								octave: 1,
								alteration: Alteration::Natural,
							}]
							.into(),
						},
						RhythmicEvent::Note {
							duration: Duration { base: 2, dots: 0 },
							pitches: [Pitch {
								step: PitchStep::F,
								octave: 1,
								alteration: Alteration::Natural,
							}]
							.into(),
						},
						RhythmicEvent::Note {
							duration: Duration { base: 2, dots: 0 },
							pitches: [Pitch {
								step: PitchStep::F,
								octave: 1,
								alteration: Alteration::Natural,
							}]
							.into(),
						},
						// ---
						RhythmicEvent::Note {
							duration: Duration { base: 0, dots: 0 },
							pitches: [Pitch {
								step: PitchStep::D,
								octave: 1,
								alteration: Alteration::Natural,
							}]
							.into(),
						},
					]
					.into(),
					mod_events: [].into(),
				}]
				.into(),
			},
		)]
		.into(),

		layouts: [(
			"Final Score".into(),
			Layout {
				header: Header {
					..Default::default()
				},
				paper: PaperSettings {
					..Default::default()
				},
				layout: LayoutElement::Staff {
					voices: [LayoutVoice {
						referenced_voice: "Piano".into(),
						mod_events: [].into(),
					}]
					.into(),
				},
			},
		)]
		.into(),
	}
}

#[test]
fn round_trip() {
	let original = create_beethoven_project();

	let serialized = original.to_cdnz().expect("Serialization failed");
	let deserialized = Project::from_reader(&serialized[..]).expect("Deserialization failed");

	assert_eq!(original, deserialized);
}

#[test]
fn create_beethoven_cdnz_file() {
	let project = create_beethoven_project();
	let cdnz = project.to_cdnz().expect("CDNZ serialization failed");

	// Construct path relative to crate root
	let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
		.join("examples")
		.join("beethoven_symphony_no_5.cdnz");

	let parent = path.parent().unwrap();
	fs::create_dir_all(parent).unwrap();

	fs::write(path, cdnz).expect("Failed to write example file");
}

#[test]
fn create_beethoven_cdnx_file() {
	let project = create_beethoven_project();
	let cdnz = project.to_cdnx().expect("CDNX serialization failed");

	// Construct path relative to crate root
	let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
		.join("examples")
		.join("beethoven_symphony_no_5.cdnx");

	let parent = path.parent().unwrap();
	fs::create_dir_all(parent).unwrap();

	fs::write(path, cdnz).expect("Failed to write example file");
}
