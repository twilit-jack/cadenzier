// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::PaneContent;
use crate::gui::panes::pane::{help::Help, render::Render, setup::Setup, write::Write};

use iced::{
	Element, Length,
	alignment::Horizontal,
	widget::{button, center, column, text},
};

#[derive(Debug, Clone, Default)]
pub struct Blank;

#[derive(Debug, Clone)]
pub enum Message {
	Transform(PaneContent),
}

impl Blank {
	pub fn view(&self) -> Element<'static, Message> {
		let button = |str, pane_content| {
			button(text(str).align_x(Horizontal::Center).width(Length::Fill))
				.on_press(Message::Transform(pane_content))
				.width(Length::Fill)
		};
		center(
			column![
				text("New pane"),
				button("Render", PaneContent::Render(Render::default())),
				button("Setup", PaneContent::Setup(Setup::default())),
				button("Write", PaneContent::Write(Write::default())),
				button("Help", PaneContent::Help(Help::default())),
			]
			.width(128)
			.align_x(Horizontal::Center)
			.spacing(4),
		)
		.into()
	}
}
