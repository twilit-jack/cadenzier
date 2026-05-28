// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::PaneContent;
use crate::gui::panes::pane::{help::Help, render::Render, setup::Setup, write::Write};

use iced::{
	Element,
	widget::{button, column},
};

#[derive(Debug, Clone, Default)]
pub struct Blank;

#[derive(Debug, Clone)]
pub enum Message {
	Transform(PaneContent),
}

impl Blank {
	pub fn view(&self) -> Element<'static, Message> {
		column![
			button("Render").on_press(Message::Transform(PaneContent::Render(Render::default()))),
			button("Setup").on_press(Message::Transform(PaneContent::Setup(Setup::default()))),
			button("Write").on_press(Message::Transform(PaneContent::Write(Write::default()))),
			button("Help").on_press(Message::Transform(PaneContent::Help(Help::default()))),
		]
		.into()
	}
}
