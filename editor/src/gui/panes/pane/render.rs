// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::config::Config;

use cdnz::LayoutName;
use iced::{
	Element,
	widget::{button, column, row, scrollable, svg},
};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Render {
	pub selected_layout: LayoutName,
	pub svgs: HashMap<LayoutName, Vec<svg::Handle>>,
}

#[derive(Debug, Clone)]
pub enum Message {}

impl Render {
	pub fn update(&mut self, _message: Message) {}

	pub fn view<'a>(
		&'a self,
		_config: &Config,
		project: &'a cdnz::Project,
	) -> Element<'a, Message> {
		// Viewport: Scrollable view of LilyPond-generated score pages in a row.
		let handles = self
			.svgs
			.get(&self.selected_layout)
			.expect("`selected_layout` should be an existing layout in the project");
		let row = row(handles.iter().cloned().map(svg).map(Element::from));
		let viewport = scrollable(row);

		// Side panel: List of buttons for switching between different layouts (i.e. instrument
		// parts, conductor's score, etc.).
		let mut list_items = Vec::<Element<Message>>::new();
		for (layout_name, _) in &project.layouts {
			let item = button(layout_name.as_str());
			list_items.push(item.into());
		}
		let side_panel = column(list_items.into_iter());

		// Statusbar: HUD of relevant information.
		let statusbar = row![];

		column![row![viewport, side_panel], statusbar].into()
	}
}
