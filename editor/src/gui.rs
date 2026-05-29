// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod panes;
mod style;

use self::panes::Panes;
use crate::{
	config::{
		Config,
		keyboard::{Command, Context, Keybind},
	},
	gui::panes::pane::PaneContent,
};

use iced::{
	Element, Settings, Subscription, Task, keyboard,
	widget::pane_grid::{Axis, Direction},
};
use std::{borrow::Cow, collections::BTreeMap};

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

	Panes(panes::Message),
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
			Message::Panes(message) => {
				self.panes.update(message);
				Task::none()
			}
		}
	}

	fn view(&self) -> Element<'_, Message> {
		self.panes
			.view(&self.config, &self.project)
			.map(Message::Panes)
	}

	fn subscription(&self) -> Subscription<Message> {
		let focused_pane_id = self.panes.focus;

		// Identify active contexts
		let mut active_contexts = vec![Context::Global, Context::View];

		if let Some(id) = focused_pane_id {
			if let Some(pane) = self.panes.panes.get(id) {
				active_contexts.push(match pane.content {
					PaneContent::Blank(_) => Context::Blank,
					PaneContent::Render(_) => Context::Render,
					PaneContent::Setup(_) => Context::Setup,
					PaneContent::Write(_) => Context::Write,
					PaneContent::Help(_) => Context::Help,
				});
			}
		}

		// Aggregate keybinds from all active contexts
		let mut combined_binds = BTreeMap::new();
		for ctx in active_contexts {
			if let Some(map) = self.config.keybinds.get(&ctx) {
				combined_binds.extend(map.clone());
			}
		}

		if combined_binds.is_empty() {
			return Subscription::none();
		}

		keyboard::listen()
			.with(combined_binds)
			.filter_map(|(binds, event)| {
				let keybind = Keybind::from_event(event)?;
				binds.get(&keybind).cloned()
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
						ViewCmd::FocusLeft => {
							Message::Panes(panes::Message::FocusAdjacent(Direction::Left))
						}
						ViewCmd::FocusRight => {
							Message::Panes(panes::Message::FocusAdjacent(Direction::Right))
						}
						ViewCmd::FocusDown => {
							Message::Panes(panes::Message::FocusAdjacent(Direction::Down))
						}
						ViewCmd::FocusUp => {
							Message::Panes(panes::Message::FocusAdjacent(Direction::Up))
						}
						ViewCmd::SplitFocusedVertical => {
							Message::Panes(panes::Message::SplitFocused(Axis::Vertical))
						}
						ViewCmd::SplitFocusedHorizontal => {
							Message::Panes(panes::Message::SplitFocused(Axis::Horizontal))
						}
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
