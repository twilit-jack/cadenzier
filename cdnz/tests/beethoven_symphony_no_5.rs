// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit-jack@gmail.com>
// SPDX-License-Identifier: LGPL-3.0-or-later

use cdnz::*;

use num::Rational32;
use std::collections::{BTreeMap, HashMap};

fn beethoven_create_cdnz() -> Cdnz {
	Cdnz {
		cdnz: Metadata {
			score_title: "Symphony No. 5".to_string(),

			composer: PersonInfo {
				name: "Ludwig van Beethoven".to_string(),
				email: None,
				is_person: true,
			},
			arranger: PersonInfo {
				name: "Twilit Jack".to_string(),
				email: Some("twilit.jack@proton.me".to_string()),
				is_person: true,
			},
			engraver: PersonInfo {
				name: "Twilit Jack".to_string(),
				email: Some("twilit.jack@proton.me".to_string()),
				is_person: true,
			},

			description: "Symphony No. 5 in C minor of Ludwig van Beethoven, Op. 67, written \
				between 1804 and 1808.\n\n\
				It is one of the best-known compositions in classical music."
				.to_string(),

			music_license: "Public-Domain".to_string(),
			engraving_license: "CC0-1.0".to_string(),
		},

		global: GlobalData {
			modifier_events: BTreeMap::from([(
				Position {
					measure: 0,
					pos: Rational32::default(),
					grace_index: 0,
				},
				Vec::from([
					GlobalModEvent::KeyChange {
						note: Pitch {
							step: 0,
							alteration: Rational32::default(),
						},
						mode: KeyMode::Minor,
					},
					GlobalModEvent::TimeChange { count: 2, unit: 4 },
				]),
			)]),
		},

		parts: HashMap::from([(
			"piano".to_string(),
			Part {
				rhythmic_events: BTreeMap::from([
					(
						Position {
							measure: 0,
							pos: Rational32::default(),
							grace_index: 0,
						},
						RhythmicEvent::Rest {
							duration: Duration { base: 3, dots: 0 },
						},
					),
					(
						Position {
							measure: 0,
							pos: Rational32::new(1, 4),
							grace_index: 0,
						},
						RhythmicEvent::Note {
							pitch: Pitch {
								step: 4,
								alteration: Rational32::default(),
							},
							duration: Duration { base: 3, dots: 0 },
						},
					),
					(
						Position {
							measure: 0,
							pos: Rational32::new(2, 4),
							grace_index: 0,
						},
						RhythmicEvent::Note {
							pitch: Pitch {
								step: 4,
								alteration: Rational32::default(),
							},
							duration: Duration { base: 3, dots: 0 },
						},
					),
					(
						Position {
							measure: 0,
							pos: Rational32::new(3, 4),
							grace_index: 0,
						},
						RhythmicEvent::Note {
							pitch: Pitch {
								step: 4,
								alteration: Rational32::default(),
							},
							duration: Duration { base: 3, dots: 0 },
						},
					),
					(
						Position {
							measure: 1,
							pos: Rational32::default(),
							grace_index: 0,
						},
						RhythmicEvent::Note {
							pitch: Pitch {
								step: 2,
								alteration: Rational32::new(-1, 2),
							},
							duration: Duration { base: 0, dots: 0 },
						},
					),
				]),
				modifier_events: BTreeMap::from([]),
			},
		)]),

		books: Vec::from([Book {
			label: "Final Score".to_string(),
			header: Header {},
			layout: Layout::Staff(Staff {
				parts: Vec::from(["piano".to_string()]),
			}),
		}]),
	}
}

#[test]
fn beethoven_round_trip() {
	let original = beethoven_create_cdnz();

	let serialized = original.serialize().expect("Serialization failed");
	let deserialized = Cdnz::deserialize(&serialized[..]).expect("Deserialization failed");

	assert_eq!(format!("{:?}", original), format!("{:?}", deserialized));
}

#[test]
fn beethoven_create_json_example_file() {
	let original = beethoven_create_cdnz();

	let json = serde_json::to_string_pretty(&original).expect("JSON serialization failed");
	println!("{}", json);
}
