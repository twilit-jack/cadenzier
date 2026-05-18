// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::gui::GlobalState;

use iced::{
	Element,
	keyboard::{Key, Modifiers},
	widget::{scrollable, text},
};

#[derive(Debug, Default)]
pub struct Setup {}

#[derive(Debug, Clone)]
pub enum Message {}

#[derive(Debug, Clone)]
pub enum Action {
	None,
}

impl Setup {
	pub fn update(&mut self, _global: &mut GlobalState, _message: Message) -> Action {
		Action::None
	}

	pub fn view(&self, _global: &GlobalState) -> Element<'_, Message> {
		scrollable(text("Setup screen placeholder")).into()
	}

	pub fn keyboard(key: Key, modifiers: Modifiers) -> Option<Message> {
		Some(match (key, modifiers) {
			_ => return None,
		})
	}
}
