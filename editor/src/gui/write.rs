// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::gui::GlobalState;

use iced::{
	Element, Length,
	widget::{Column, button, column, row, rule, scrollable, text},
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
		let side_panel = Column::from_vec(
			[
				// Side panel "Layouts" title/header
				text("Layouts").into(),
				rule::horizontal(2).into(),
			]
			.into_iter()
			// Convert all layouts in `state.project` into side panel toggles
			.chain(
				global
					.project
					.layouts
					.keys()
					.map(|name| button(name.as_str()).into()), // TODO: Add functionality on button
			)
			.collect::<Vec<_>>(),
		);

		let viewport = scrollable(
			text("Viewport placeholder")
				.width(Length::Fill)
				.height(Length::Fill),
		);

		let status_bar = row![];

		column![row![viewport, side_panel], status_bar].into()
	}
}
