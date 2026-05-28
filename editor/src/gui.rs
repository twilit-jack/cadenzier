// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod panes;
mod style;

use self::panes::Panes;
use crate::{
	config::{
		keyboard::{Command, Context, Keybind},
		Config,
	},
	gui::panes::pane::PaneContent,
};

use iced::{keyboard, Element, Settings, Subscription, Task};
use std::borrow::Cow;

pub(super) fn run() -> iced::Result {
	iced::application(Editor::default, Editor::update, Editor::view)
		.subscription(Editor::subscription)
		.settings(Editor::settings())
		.run()
}

struct Editor {
	panes: Panes,

	config: Config,
	project: cdnz::Project,
}

#[derive(Debug, Clone)]
pub enum Message {
	/// Prints debug text to show that the message was recieved.
	DebugPrint,

	Pane(panes::Message),
}

impl Default for Editor {
	fn default() -> Self {
		Self {
			panes: Panes::default(),

			config: Config::load_from_disk().unwrap_or_default(),
			project: cdnz::Project::default(),
		}
	}
}

impl Editor {
	fn update(&mut self, message: Message) -> Task<Message> {
		match message {
			Message::DebugPrint => {
				eprintln!("Debug print message recieved!");
				Task::none()
			}
			Message::Pane(message) => {
				self.panes.update(message);
				Task::none()
			}
		}
	}

	fn view(&self) -> Element<'_, Message> {
		self.panes
			.view(&self.config, &self.project)
			.map(Message::Pane)
	}

	fn subscription(&self) -> Subscription<Message> {
		let Some(focused_pane_id) = self.panes.focus else {
			return Subscription::none();
		};
		let Some(focused_pane) = self.panes.panes.get(focused_pane_id) else {
			return Subscription::none();
		};
		let context = match focused_pane.content {
			PaneContent::Blank(_) => Context::Blank,
			PaneContent::Render(_) => Context::Render,
			PaneContent::Setup(_) => Context::Setup,
			PaneContent::Write(_) => Context::Write,
			PaneContent::Help(_) => Context::Help,
		};

		let Some(keybinds) = self.config.keybinds.get(&context) else {
			return Subscription::none();
		};

		let keybinds = keybinds.clone();
		keyboard::listen()
			.with(keybinds)
			.filter_map(|(keybinds, event)| {
				let keybind = Keybind::from_event(event)?;
				Some(keybinds.get(&keybind)?.clone())
			})
			.filter_map(|command| {
				#[allow(unused_imports)] // Will be expanded in the near future
				use crate::config::keyboard::{
					BlankCmd, GlobalCmd, HelpCmd, RenderCmd, SetupCmd, ViewCmd, WriteCmd,
				};
				Some(match command {
					Command::Global(command) => match command {
						GlobalCmd::DebugPrint => Message::DebugPrint,
					},
					Command::View(command) => match command {
						//ViewCmd:: => Message::,
					},
					Command::Blank(command) => match command {
						//BlankCmd:: => Message::,
					},
					Command::Render(command) => match command {
						//RenderCmd:: => Message::,
					},
					Command::Setup(command) => match command {
						//SetupCmd:: => Message::,
					},
					Command::Write(command) => match command {
						//WriteCmd:: => Message::,
					},
					Command::Help(command) => match command {
						//HelpCmd:: => Message::,
					},
				})
			})
	}

	fn settings() -> Settings {
		Settings {
			fonts: vec![
				Cow::Borrowed(include_bytes!("../fonts/fa_7_regular_400.otf")),
				Cow::Borrowed(include_bytes!("../fonts/fa_7_solid_900.otf")),
				Cow::Borrowed(include_bytes!("../fonts/fa_7_brands_400.otf")),
			],
			..Settings::default()
		}
	}
}
