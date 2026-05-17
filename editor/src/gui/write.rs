// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::gui::GlobalState;

use iced::{
	Element, Length,
	widget::{button, checkbox, column, row, rule, scrollable, text},
};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Write {
	pub shown_parts: HashMap<cdnz::PartName, bool>,
}

#[derive(Debug, Clone)]
pub enum Message {
	TogglePartVisibility(cdnz::PartName, bool),
	JumpToPart(cdnz::PartName),
}

#[derive(Debug, Clone)]
pub enum Action {
	None,
}

impl Write {
	pub fn update(&mut self, global: &mut GlobalState, message: Message) -> Action {
		// Make sure all parts are in `self.shown_parts`
		for name in global.project.parts.keys() {
			self.shown_parts.entry(name.clone()).or_insert(true);
		}
		match message {
			Message::TogglePartVisibility(name, value) => {
				let _ = self.shown_parts.insert(name, value);
				Action::None
			}
			Message::JumpToPart(_name) => todo!(),
		}
	}

	pub fn view<'a>(&'a self, _global: &'a GlobalState) -> Element<'a, Message> {
		// Side panel: Shows a list of all parts for toggling which ones are shown, as well as a jump
		// to a specific part.
		let mut side_panel = column![text("Parts"), rule::horizontal(2)].spacing(10);
		for name in self.shown_parts.keys() {
			side_panel = side_panel.push(row![
				// Visibility toggle
				checkbox(
					*self
						.shown_parts
						.get(name)
						.expect("We're iterating over this")
				)
				.on_toggle(|value| Message::TogglePartVisibility(name.clone(), value)),
				// Label and jump-to button
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
