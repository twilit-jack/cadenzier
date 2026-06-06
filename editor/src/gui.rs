// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod keyboard;
mod style;
mod tab;

use self::tab::Tab;
use crate::{
	config::Config,
	gui::style::icons::{Icon, icon},
};

use iced::{
	Element, Settings, Subscription, Task,
	widget::{Row, button, center, column, text},
};
use std::borrow::Cow;

pub(super) fn run() -> iced::Result {
	iced::application(Editor::default, Editor::update, Editor::view)
		.subscription(Editor::subscription)
		.settings(Editor::settings())
		.run()
}

struct Editor {
	tabs: Vec<Tab>,
	selected_tab: usize,

	config: Config,
	project: cdnz::Project,
}

#[derive(Debug, Clone)]
enum Message {
	/// Prints debug text to show that the message was recieved.
	DebugPrint,

	NewTab,
	DeleteTab(usize),
	SwitchToTab(usize),
	Tab(tab::Message),
}

impl Default for Editor {
	fn default() -> Self {
		Self {
			tabs: vec![Tab::default()],
			selected_tab: 0,

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
			Message::NewTab => {
				self.tabs.push(Tab::default());
				self.selected_tab = self.tabs.len() - 1;
				Task::none()
			}
			Message::DeleteTab(tab_idx) => {
				self.tabs.remove(tab_idx);
				if self.selected_tab >= tab_idx {
					if self.tabs.len() != 0 {
						self.selected_tab -= 1;
					} else {
						self.tabs.push(Tab::default());
						self.selected_tab = 0;
					}
				}
				Task::none()
			}
			Message::SwitchToTab(tab_idx) => {
				self.selected_tab = tab_idx;
				Task::none()
			}
			Message::Tab(message) => {
				self.tabs
					.get_mut(self.selected_tab)
					.expect("selected tab should exist")
					.update(message, &mut self.project);
				Task::none()
			}
		}
	}

	fn view(&self) -> Element<'_, Message> {
		let tab_bar = {
			let mut row = Row::with_capacity(self.tabs.len());
			for (i, tab) in self.tabs.iter().enumerate() {
				if self.selected_tab != i {
					row = row.push(button(tab.label.as_str()).on_press(Message::SwitchToTab(i)));
				} else {
					row = row.push(button(tab.label.as_str()));
				}
			}
			row.push(button(icon(Icon::Plus)).on_press(Message::NewTab))
		};

		let content: Element<Message> = {
			if let Some(tab) = self.tabs.get(self.selected_tab) {
				tab.view(&self.config, &self.project)
					.map(Message::Tab)
					.into()
			} else {
				center(text("No tab selected. This might be a bug.")).into()
			}
		};

		column![tab_bar, content].into()
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
