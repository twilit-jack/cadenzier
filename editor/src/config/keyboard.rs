// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

use iced::keyboard::{self as kb, Event};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Display};

pub type Keymap = BTreeMap<Context, BTreeMap<Keybind, Command>>;

pub fn default_keymap() -> Keymap {
	BTreeMap::from([
		(Context::Global, BTreeMap::from([])),
		(
			Context::View,
			BTreeMap::from([
				(
					Keybind {
						key: "h".into(),
						modifiers: Modifiers::none(),
					},
					Command::View(ViewCmd::FocusLeft),
				),
				(
					Keybind {
						key: "j".into(),
						modifiers: Modifiers::none(),
					},
					Command::View(ViewCmd::FocusDown),
				),
				(
					Keybind {
						key: "k".into(),
						modifiers: Modifiers::none(),
					},
					Command::View(ViewCmd::FocusUp),
				),
				(
					Keybind {
						key: "l".into(),
						modifiers: Modifiers::none(),
					},
					Command::View(ViewCmd::FocusRight),
				),
				(
					Keybind {
						key: "z".into(),
						modifiers: Modifiers::none(),
					},
					Command::View(ViewCmd::SplitFocusedVertical),
				),
				(
					Keybind {
						key: "z".into(),
						modifiers: Modifiers {
							shift: true,
							..Default::default()
						},
					},
					Command::View(ViewCmd::SplitFocusedHorizontal),
				),
			]),
		),
		(Context::Render, BTreeMap::from([])),
		(Context::Setup, BTreeMap::from([])),
		(Context::Write, BTreeMap::from([])),
		(Context::Help, BTreeMap::from([])),
	])
}

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

impl Display for Keybind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}{}{}{}",
			if self.modifiers.command {
				if cfg!(target_os = "macos") {
					"Cmd+"
				} else {
					"Ctrl+"
				}
			} else {
				""
			},
			if self.modifiers.alt { "Alt+" } else { "" },
			if self.modifiers.shift { "Shift+" } else { "" },
			self.key.0.to_uppercase(),
		)
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
			kb::Key::Character(char) => Self(char.to_lowercase()),
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
	pub command: bool,
	pub alt: bool,
	pub shift: bool,
}

impl From<kb::Modifiers> for Modifiers {
	fn from(mods: kb::Modifiers) -> Self {
		Self {
			command: mods.command(),
			alt: mods.alt(),
			shift: mods.shift(),
		}
	}
}

impl Modifiers {
	/// Semantic wrapper for `default()`.
	pub fn none() -> Self {
		Self::default()
	}
}

/// Dictates a certain context in which a keybind is active.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Context {
	Global,
	View,

	// Panes
	Blank,
	Render,
	Setup,
	Write,
	Help,
}

/// Commands that get triggered by keybinds, and are usually mapped to messages.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Command {
	Global(GlobalCmd),
	View(ViewCmd),

	// Panes
	Blank(BlankCmd),
	Render(RenderCmd),
	Setup(SetupCmd),
	Write(WriteCmd),
	Help(HelpCmd),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GlobalCmd {
	DebugPrint,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ViewCmd {
	FocusLeft,
	FocusRight,
	FocusDown,
	FocusUp,
	SplitFocusedVertical,
	SplitFocusedHorizontal,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlankCmd {}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RenderCmd {}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SetupCmd {}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WriteCmd {}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HelpCmd {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn keybind_display() {
		let keybind = Keybind {
			key: "a".into(),
			modifiers: Modifiers {
				command: true,
				shift: true,
				..Default::default()
			},
		};
		assert_eq!(format!("{keybind}"), "Ctrl+Shift+A");
	}
}
