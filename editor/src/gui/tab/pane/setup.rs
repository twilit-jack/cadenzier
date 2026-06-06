// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
	config::Config,
	gui::style::icons::{Icon, icon},
};

use iced::{
	Element,
	alignment::Horizontal,
	widget::{Column, button, center, column, row, rule::horizontal as h_rule, scrollable, text},
};
use std::collections::BTreeMap;

const SIDE_PANEL_WIDTH: u32 = 160;

#[derive(Debug, Clone, Default)]
pub struct Setup {
	mode: Mode,
	selected_part: Option<cdnz::PartName>,
	selected_layout: Option<cdnz::LayoutName>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Mode {
	#[default]
	Parts,
	Layouts,
}

#[derive(Debug, Clone)]
pub enum Message {
	SwitchMode(Mode),
	SelectPart(cdnz::PartName),
	SelectLayout(cdnz::LayoutName),
	AddPart,
	AddLayout,
}

impl Setup {
	pub fn update(&mut self, message: Message, project: &mut cdnz::Project) {
		match message {
			Message::SwitchMode(mode) => self.mode = mode,
			Message::SelectPart(part) => self.selected_part = Some(part),
			Message::SelectLayout(layout) => self.selected_layout = Some(layout),
			Message::AddPart => {
				let name = generate_unique_name("New part", &project.parts);
				project.parts.insert(name.clone(), cdnz::Part::default());
				self.selected_part = Some(name);
			}
			Message::AddLayout => {
				let name = generate_unique_name("New layout", &project.layouts);
				project
					.layouts
					.insert(name.clone(), cdnz::Layout::default());
				self.selected_layout = Some(name);
			}
		}
	}

	pub fn view<'a>(
		&'a self,
		_config: &'a Config,
		project: &'a cdnz::Project,
	) -> Element<'a, Message> {
		let setup_menu = { center(text("Setup menu placeholder")) };

		let side_panel = {
			let mode_switch_buttons = row![
				mode_button("Parts", Mode::Parts, &self.mode),
				mode_button("Layouts", Mode::Layouts, &self.mode),
			]
			.width(SIDE_PANEL_WIDTH)
			.spacing(4);

			let label = text(match self.mode {
				Mode::Parts => "Parts",
				Mode::Layouts => "Layouts",
			});

			let content = match self.mode {
				Mode::Parts => render_list(
					&project.parts,
					self.selected_part.as_deref(),
					Message::SelectPart,
					Message::AddPart,
				),
				Mode::Layouts => render_list(
					&project.layouts,
					self.selected_layout.as_deref(),
					Message::SelectLayout,
					Message::AddLayout,
				),
			};

			column![mode_switch_buttons, label, h_rule(2), content].width(SIDE_PANEL_WIDTH)
		};

		row![setup_menu, side_panel].into()
	}
}

fn generate_unique_name<T>(prefix: &str, map: &BTreeMap<String, T>) -> String {
	(0..)
		.map(|i| match i {
			0 => prefix.to_string(),
			_ => format!("{prefix} {i}"),
		})
		.find(|name| !map.contains_key(name))
		.unwrap()
}

fn mode_button<'a>(label: &'static str, target: Mode, current: &'a Mode) -> Element<'a, Message> {
	let btn = button(label);
	if &target == current {
		btn.into()
	} else {
		btn.on_press(Message::SwitchMode(target)).into()
	}
}

fn render_list<'a, T, F>(
	map: &'a BTreeMap<String, T>,
	selected: Option<&str>,
	on_select: F,
	on_add: Message,
) -> Element<'a, Message>
where
	F: 'a + Fn(String) -> Message,
{
	let mut col = Column::with_capacity(map.len())
		.width(SIDE_PANEL_WIDTH)
		.align_x(Horizontal::Center)
		.spacing(4);

	for name in map.keys() {
		let mut btn = button(name.as_str()).width(SIDE_PANEL_WIDTH);
		if Some(name.as_str()) != selected {
			btn = btn.on_press(on_select(name.clone()));
		}
		col = col.push(btn);
	}

	scrollable(col.push(button(icon(Icon::Plus)).on_press(on_add))).into()
}
