// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

use iced::keyboard::{self as kb, Event};
use serde::{Deserialize, Serialize};

/// Minimized mirror enum of `iced::keyboard::Event` that implements `Serialize` and `Deserialize`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Keybind {
	pub key: Key,
	pub modifiers: Modifiers,
}

impl Keybind {
	pub fn from_event(event: Event) -> Option<Self> {
		match event {
			Event::KeyPressed { key, modifiers, .. } => Some(Self {
				key: key.into(),
				modifiers: modifiers.into(),
			}),
			_ => None,
		}
	}
}

/// Mirror type of `iced::keyboard::Key` that implements `Serialize` and `Deserialize`.
///
/// Also less type-safe, as it doesn't differentiate `Key::Named` from `Key::Character` or
/// `Key::Unidentified`, but in practice, this doesn't matter, as there are no runtime situations
/// that make this a problem.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key(String);

impl From<kb::Key> for Key {
	fn from(key: kb::Key) -> Self {
		match key {
			// This is a very beautiful and stable solution pls no violence
			//
			// In all honesty, this should be watched, and future config conversion scripts should do
			// a bit of string conversion if `iced::keyboard::key::Named` ever changes (though it might
			// not change)
			kb::Key::Named(name) => Self(format!("{name:?}")),
			kb::Key::Character(char) => Self(char.into()),
			kb::Key::Unidentified => Self("".into()),
		}
	}
}

impl From<&str> for Key {
	fn from(str: &str) -> Self {
		Self(str.into())
	}
}

/// Simpler implementation of `iced::keyboard::Modifiers`.
///
/// This implementation is also less-performant, as it uses basic struct fields instead of bit
/// manipulation.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Modifiers {
	pub shift: bool,
	pub command: bool,
	pub alt: bool,
}

impl From<kb::Modifiers> for Modifiers {
	fn from(mods: kb::Modifiers) -> Self {
		Self {
			shift: mods.shift(),
			command: mods.command(),
			alt: mods.alt(),
		}
	}
}

impl Modifiers {
	/// Semantic wrapper for `default()`.
	pub fn none() -> Self {
		Self::default()
	}
}
