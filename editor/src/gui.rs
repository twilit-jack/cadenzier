// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod help;
mod render;
mod setup;
mod write;

use help::Help;
use render::Render;
use setup::Setup;
use write::Write;

use crate::config::Config;

use iced::{
	Element, Subscription, Task,
	keyboard::{self, Event},
};

pub(super) fn run() -> iced::Result {
	iced::application(Editor::default, Editor::update, Editor::view)
		.subscription(Editor::subscription)
		.run()
}

struct Editor {
	screen: Screen,
	global: GlobalState,
}

struct GlobalState {
	config: Config,
	project: cdnz::Project,
}

impl Default for Editor {
	fn default() -> Self {
		Self {
			screen: Screen::Setup(Setup::default()),
			global: GlobalState {
				config: Config::load_from_disk().unwrap_or_default(),
				project: cdnz::Project::default(),
			},
		}
	}
}

enum Screen {
	Render(Render),
	Setup(Setup),
	Write(Write),
	Help(Help),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScreenId {
	Render,
	Setup,
	Write,
	Help,
}

impl Screen {
	pub fn get_id(&self) -> ScreenId {
		match self {
			Screen::Render(_) => ScreenId::Render,
			Screen::Setup(_) => ScreenId::Setup,
			Screen::Write(_) => ScreenId::Write,
			Screen::Help(_) => ScreenId::Help,
		}
	}
}

#[derive(Debug, Clone)]
enum Message {
	Render(render::Message),
	Setup(setup::Message),
	Write(write::Message),
	Help(help::Message),
}

impl Editor {
	fn update(&mut self, message: Message) -> Task<Message> {
		match message {
			Message::Render(message) => {
				let Screen::Render(render) = &mut self.screen else {
					return Task::none();
				};
				let action = render.update(&mut self.global, message);
				match action {
					render::Action::None => Task::none(),
				}
			}
			Message::Setup(message) => {
				let Screen::Setup(setup) = &mut self.screen else {
					return Task::none();
				};
				let action = setup.update(&mut self.global, message);
				match action {
					setup::Action::None => Task::none(),
				}
			}
			Message::Write(message) => {
				let Screen::Write(write) = &mut self.screen else {
					return Task::none();
				};
				let action = write.update(&mut self.global, message);
				match action {
					write::Action::None => Task::none(),
				}
			}
			Message::Help(message) => {
				let Screen::Help(help) = &mut self.screen else {
					return Task::none();
				};
				let action = help.update(&mut self.global, message);
				match action {
					help::Action::None => Task::none(),
				}
			}
		}
	}

	fn view(&self) -> Element<'_, Message> {
		match &self.screen {
			Screen::Render(render) => render.view(&self.global).map(Message::Render),
			Screen::Setup(setup) => setup.view(&self.global).map(Message::Setup),
			Screen::Write(write) => write.view(&self.global).map(Message::Write),
			Screen::Help(help) => help.view(&self.global).map(Message::Help),
		}
		//let mode_button = |label: &'static str, screen: Screen| {
		//	button(label)
		//		.on_press(Message::ScreenSwitch(screen))
		//		.style(move |theme: &Theme, _| {
		//			let palette = theme.extended_palette();
		//			let active = self.screen;
		//			button::Style {
		//				background: Some(
		//					if active {
		//						palette.primary.base.color
		//					} else {
		//						palette.background.base.color
		//					}
		//					.into(),
		//				),
		//				text_color: palette.background.base.text,
		//				..button::Style::default()
		//			}
		//		})
		//};
		//let mode_switch_buttons = row![
		//	mode_button("Setup", Screen::Setup),
		//	mode_button("Write", Screen::Write),
		//	mode_button("Help", Screen::Help),
		//];
	}

	fn subscription(&self) -> Subscription<Message> {
		let screen_id = self.screen.get_id();
		let map_event = move |event| match event {
			Event::KeyPressed { key, modifiers, .. } => match screen_id {
				ScreenId::Render => Render::keyboard(key, modifiers).map(Message::Render),
				ScreenId::Setup => Setup::keyboard(key, modifiers).map(Message::Setup),
				ScreenId::Write => Write::keyboard(key, modifiers).map(Message::Write),
				ScreenId::Help => Help::keyboard(key, modifiers).map(Message::Help),
			},
			_ => None,
		};
		keyboard::listen().filter_map(map_event)
	}
}
