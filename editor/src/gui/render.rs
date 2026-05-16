// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

// use std::collections::HashMap;

use crate::gui::GlobalState;

// use cdnz::LayoutName;
use iced::{
	Element,
	widget::{scrollable, text},
};

#[derive(Debug, Default)]
pub struct Render {}

#[derive(Debug, Clone)]
pub enum Message {}

#[derive(Debug, Clone)]
pub enum Action {
	None,
}

impl Render {
	pub fn update(&mut self, _global: &mut GlobalState, _message: Message) -> Action {
		Action::None
	}

	pub fn view(&self, _global: &GlobalState) -> Element<'_, Message> {
		scrollable(text("Render screen placeholder")).into()
	}
}
