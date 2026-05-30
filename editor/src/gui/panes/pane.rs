// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod blank;
mod help;
mod render;
mod setup;
mod write;

use blank::Blank;
use help::Help;
use render::Render;
use setup::Setup;
use write::Write;

use iced::{Element, widget::container};

use crate::config::Config;

#[derive(Clone, Default)]
pub struct Pane {
	pub content: PaneContent,
}

#[derive(Debug, Clone)]
pub enum PaneContent {
	Blank(Blank),
	Render(Render),
	Setup(Setup),
	Write(Write),
	Help(Help),
}

impl Default for PaneContent {
	fn default() -> Self {
		PaneContent::Blank(Blank::default())
	}
}

#[derive(Debug, Clone)]
pub enum Message {
	Blank(blank::Message),
	Render(render::Message),
	Setup(setup::Message),
	Write(write::Message),
	Help(help::Message),
}

impl Pane {
	pub fn update(&mut self, message: Message, project: &mut cdnz::Project) {
		match &mut self.content {
			PaneContent::Blank(_) => match message {
				Message::Blank(message) => match message {
					blank::Message::Transform(content) => self.content = content,
				},
				_ => (),
			},
			PaneContent::Render(render) => match message {
				Message::Render(message) => render.update(message, project),
				_ => (),
			},
			PaneContent::Setup(setup) => match message {
				Message::Setup(message) => setup.update(message, project),
				_ => (),
			},
			PaneContent::Write(write) => match message {
				Message::Write(message) => write.update(message, project),
				_ => (),
			},
			PaneContent::Help(help) => match message {
				Message::Help(message) => help.update(message, project),
				_ => (),
			},
		}
	}

	pub fn view<'a>(
		&'a self,
		config: &'a Config,
		project: &'a cdnz::Project,
	) -> Element<'a, Message> {
		container(match &self.content {
			PaneContent::Blank(blank) => blank.view().map(Message::Blank),
			PaneContent::Render(render) => render.view(config, project).map(Message::Render),
			PaneContent::Setup(setup) => setup.view(config, project).map(Message::Setup),
			PaneContent::Write(write) => write.view(config, project).map(Message::Write),
			PaneContent::Help(help) => help.view(config, project).map(Message::Help),
		})
		.padding(5)
		.into()
	}
}
