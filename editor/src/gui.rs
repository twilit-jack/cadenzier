// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod keyboard;
mod panes;
mod style;

use self::panes::Panes;
use crate::config::Config;

use iced::{Element, Settings, Subscription, Task};
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
enum Message {
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
				self.panes.update(message, &mut self.project);
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
		self.keyboard() // Defined in mod `keyboard`
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
