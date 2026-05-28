// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::config::Config;

use iced::{
	Element,
	widget::{scrollable, text},
};

#[derive(Debug, Clone, Default)]
pub struct Help {}

#[derive(Debug, Clone)]
pub enum Message {}

impl Help {
	pub fn update(&mut self, _message: Message) {}

	pub fn view<'a>(
		&'a self,
		_config: &'a Config,
		_project: &'a cdnz::Project,
	) -> Element<'a, Message> {
		scrollable(text("Help screen placeholder")).into()
	}
}
