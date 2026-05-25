// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit-jack@gmail.com>
// SPDX-License-Identifier: LGPL-3.0-or-later

use cdnz::*;
use num::Rational32 as Fraction;
use std::{collections::BTreeMap, fs, path::PathBuf};

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
				Position {
					measure: 0,
					pos: Fraction::default(),
					grace_index: 0,
				},
				[
					GlobalModEvent::KeyChange {
						note: Pitch {
							step: 0,
							alteration: Fraction::default(),
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
				voices: vec![Voice {
					instrument: Instrument::Piano,
					rhythmic_events: BTreeMap::from([
						(
							Position {
								measure: 0,
								pos: Fraction::default(),
								grace_index: 0,
							},
							RhythmicEvent::Rest {},
						),
						(
							Position {
								measure: 0,
								pos: Fraction::new(1, 4),
								grace_index: 0,
							},
							RhythmicEvent::Note {
								pitches: vec![Pitch {
									step: 4,
									alteration: Fraction::default(),
								}],
							},
						),
						(
							Position {
								measure: 0,
								pos: Fraction::new(2, 4),
								grace_index: 0,
							},
							RhythmicEvent::Note {
								pitches: vec![Pitch {
									step: 4,
									alteration: Fraction::default(),
								}],
							},
						),
						(
							Position {
								measure: 0,
								pos: Fraction::new(3, 4),
								grace_index: 0,
							},
							RhythmicEvent::Note {
								pitches: vec![Pitch {
									step: 4,
									alteration: Fraction::default(),
								}],
							},
						),
						(
							Position {
								measure: 1,
								pos: Fraction::default(),
								grace_index: 0,
							},
							RhythmicEvent::Note {
								pitches: [Pitch {
									step: 2,
									alteration: Fraction::new(-1, 2),
								}]
								.into(),
							},
						),
					]),
					mod_events: [].into(),
				}],
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
