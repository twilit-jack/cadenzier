// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::config::Config;

use iced::{
	Element,
	widget::{scrollable, text},
};

#[derive(Debug, Clone, Default)]
pub struct Setup {}

#[derive(Debug, Clone)]
pub enum Message {}

impl Setup {
	pub fn update(&mut self, _message: Message) {}

	pub fn view(&self, _config: &Config, _project: &cdnz::Project) -> Element<'_, Message> {
		scrollable(text("Setup screen placeholder")).into()
	}
}
