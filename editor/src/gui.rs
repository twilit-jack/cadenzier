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

use iced::{
	keyboard::{self, Event},
	Element, Subscription, Task,
};

pub(super) fn run() -> iced::Result {
	iced::application(State::default, update, view)
		.subscription(subscription)
		.run()
}

struct State {
	screen: Screen,
	global: GlobalState,
}

struct GlobalState {
	project: cdnz::Project,
}

impl Default for State {
	fn default() -> Self {
		Self {
			screen: Screen::Setup(Setup::default()),
			global: GlobalState {
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

fn update(state: &mut State, message: Message) -> Task<Message> {
	match message {
		Message::Render(message) => {
			let Screen::Render(render) = &mut state.screen else {
				return Task::none();
			};
			let action = render.update(&mut state.global, message);
			match action {
				render::Action::None => Task::none(),
			}
		}
		Message::Setup(message) => {
			let Screen::Setup(setup) = &mut state.screen else {
				return Task::none();
			};
			let action = setup.update(&mut state.global, message);
			match action {
				setup::Action::None => Task::none(),
			}
		}
		Message::Write(message) => {
			let Screen::Write(write) = &mut state.screen else {
				return Task::none();
			};
			let action = write.update(&mut state.global, message);
			match action {
				write::Action::None => Task::none(),
			}
		}
		Message::Help(message) => {
			let Screen::Help(help) = &mut state.screen else {
				return Task::none();
			};
			let action = help.update(&mut state.global, message);
			match action {
				help::Action::None => Task::none(),
			}
		}
	}
}

fn view(state: &State) -> Element<'_, Message> {
	match &state.screen {
		Screen::Render(render) => render.view(&state.global).map(Message::Render),
		Screen::Setup(setup) => setup.view(&state.global).map(Message::Setup),
		Screen::Write(write) => write.view(&state.global).map(Message::Write),
		Screen::Help(help) => help.view(&state.global).map(Message::Help),
	}
	//let mode_button = |label: &'static str, screen: Screen| {
	//	button(label)
	//		.on_press(Message::ScreenSwitch(screen))
	//		.style(move |theme: &Theme, _| {
	//			let palette = theme.extended_palette();
	//			let active = state.screen;
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

fn subscription(state: &State) -> Subscription<Message> {
	let screen_id = state.screen.get_id();
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
