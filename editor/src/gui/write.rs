// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::gui::GlobalState;

use iced::{
	Element, Length,
	widget::{button, column, row, rule, scrollable, text},
};

#[derive(Debug, Default)]
pub struct Write {}

#[derive(Debug, Clone)]
pub enum Message {}

#[derive(Debug, Clone)]
pub enum Action {
	None,
}

impl Write {
	pub fn update(&mut self, _global: &mut GlobalState, _message: Message) -> Action {
		Action::None
	}

	pub fn view<'a>(&'a self, global: &'a GlobalState) -> Element<'a, Message> {
		let mut side_panel = column![text("Layouts"), rule::horizontal(2)].spacing(10);

		for name in global.project.layouts.keys() {
			side_panel = side_panel.push(button(name.as_str())); // TODO: Add .on_press(...)
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
