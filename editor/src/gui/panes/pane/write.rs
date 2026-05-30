// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

use iced::{
	Element, Length,
	widget::{button, checkbox, column, row, rule, scrollable, text},
};
use std::collections::HashSet;

use crate::config::Config;

#[derive(Debug, Clone, Default)]
pub struct Write {
	pub mode: Mode,

	/// Set of all parts in the active project which are hidden in write viewport.
	///
	/// Parts which aren't in this set are shown.
	pub hidden_parts: HashSet<cdnz::PartName>,
}

/// Helix-like editing mode.
#[derive(Debug, Clone, Copy, Default)]
pub enum Mode {
	#[default]
	Normal,
	Insert,
	Select,
	Command,

	Space,
	View,
}

#[derive(Debug, Clone)]
pub enum Message {
	SetMode(Mode),

	/// If `bool` is `true`, part will be visible. If `bool` is `false`, part will be hidden.
	TogglePartVisibility(cdnz::PartName, bool),
	JumpToPart(cdnz::PartName),
}

impl Write {
	pub fn update(&mut self, message: Message, project: &mut cdnz::Project) {
		match message {
			Message::SetMode(mode) => self.mode = mode,
			Message::TogglePartVisibility(name, is_visible) => {
				if is_visible {
					self.hidden_parts.remove(&name);
				} else {
					self.hidden_parts.insert(name);
				}
			}
			Message::JumpToPart(_name) => todo!(),
		}
	}

	pub fn view<'a>(
		&'a self,
		_config: &'a Config,
		project: &'a cdnz::Project,
	) -> Element<'a, Message> {
		// Side panel: Shows a list of all parts for toggling which ones are shown, as well as a jump
		// to a specific part.
		let mut side_panel = column![text("Parts"), rule::horizontal(2)].spacing(10);

		for name in project.parts.keys() {
			let is_visible = !self.hidden_parts.contains(name);

			side_panel = side_panel.push(row![
				checkbox(is_visible)
					.on_toggle(|value| Message::TogglePartVisibility(name.clone(), value)),
				button(name.as_str()).on_press(Message::JumpToPart(name.clone()))
			]);
		}

		let viewport = scrollable(
			text("Viewport placeholder")
				.width(Length::Fill)
				.height(Length::Fill),
		);

		let mode_indicator = text(match self.mode {
			Mode::Normal | Mode::Space | Mode::View => "NORMAL",
			Mode::Insert => "INSERT",
			Mode::Select => "SELECT",
			Mode::Command => "COMMAND",
		});
		let status_bar = row![mode_indicator];

		column![row![viewport, side_panel], status_bar].into()
	}
}
