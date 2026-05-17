// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::gui::GlobalState;

use iced::{
	Element, Length,
	widget::{button, checkbox, column, row, rule, scrollable, text},
};
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct Write {
	/// Set of all parts in the active project which are hidden in write viewport.
	///
	/// Parts which aren't in this set are shown.
	pub hidden_parts: HashSet<cdnz::PartName>,
}

#[derive(Debug, Clone)]
pub enum Message {
	/// If `bool` is `true`, part will be visible. If `bool` is `false`, part will be hidden.
	TogglePartVisibility(cdnz::PartName, bool),
	JumpToPart(cdnz::PartName),
}

#[derive(Debug, Clone)]
pub enum Action {
	None,
}

impl Write {
	pub fn update(&mut self, _global: &mut GlobalState, message: Message) -> Action {
		match message {
			Message::TogglePartVisibility(name, is_visible) => {
				if is_visible {
					self.hidden_parts.remove(&name);
				} else {
					self.hidden_parts.insert(name);
				}
				Action::None
			}
			Message::JumpToPart(_name) => todo!(),
		}
	}

	pub fn view<'a>(&'a self, global: &'a GlobalState) -> Element<'a, Message> {
		// Side panel: Shows a list of all parts for toggling which ones are shown, as well as a jump
		// to a specific part.
		let mut side_panel = column![text("Parts"), rule::horizontal(2)].spacing(10);

		for name in global.project.parts.keys() {
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

		let status_bar = row![];

		column![row![viewport, side_panel], status_bar].into()
	}
}
