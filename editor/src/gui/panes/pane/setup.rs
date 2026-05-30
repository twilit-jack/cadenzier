// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::config::Config;

use iced::{
	Element,
	widget::{button, column, row, rule::horizontal as h_rule, scrollable, text},
};

#[derive(Debug, Clone, Default)]
pub struct Setup {
	mode: Mode,
	selected_part: Option<cdnz::PartName>,
}

#[derive(Debug, Clone, Default)]
pub enum Mode {
	#[default]
	Parts,
	Layouts,
}

#[derive(Debug, Clone)]
pub enum Message {
	ModeToggled(Mode),
}

impl Setup {
	pub fn update(&mut self, message: Message, project: &mut cdnz::Project) {
		match message {
			Message::ModeToggled(mode) => self.mode = mode,
		}
	}

	pub fn view<'a>(
		&'a self,
		_config: &'a Config,
		project: &'a cdnz::Project,
	) -> Element<'a, Message> {
		// Side panel
		let mut list_items = Vec::<Element<Message>>::from([
			row![
				button("Parts").on_press(Message::ModeToggled(Mode::Parts)),
				button("Layouts").on_press(Message::ModeToggled(Mode::Layouts))
			]
			.into(),
			match self.mode {
				Mode::Parts => text("Parts").into(),
				Mode::Layouts => text("Layouts").into(),
			},
			h_rule(2).into(),
		]);
		match self.mode {
			Mode::Parts => {
				for (part_name, _) in &project.parts {
					let item = button(part_name.as_str());
					list_items.push(item.into());
				}
			}
			Mode::Layouts => {
				for (layout_name, _) in &project.layouts {
					let item = button(layout_name.as_str());
					list_items.push(item.into());
				}
			}
		}
		let side_panel = column(list_items);

		scrollable(row![side_panel]).into()
	}
}
