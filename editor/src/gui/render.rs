// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::HashMap;

use crate::gui::GlobalState;

use cdnz::LayoutName;
use iced::{
	Element,
	widget::{button, column, row, scrollable, svg},
};

#[derive(Debug, Default)]
pub struct Render {
	pub selected_layout: LayoutName,
	pub svgs: HashMap<LayoutName, Vec<svg::Handle>>,
}

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

	pub fn view<'a>(&'a self, global: &'a GlobalState) -> Element<'a, Message> {
		// Viewport: Scrollable view of LilyPond-generated score pages in a row.
		let handles = self
			.svgs
			.get(&self.selected_layout)
			.expect("`selected_layout` should be real layout");
		let row = row(handles.iter().cloned().map(svg).map(Element::from));
		let viewport = scrollable(row);

		// Side panel: List of buttons for switching between different layouts (i.e. instrument
		// parts, conductor's score, etc.).
		let mut list_items = Vec::<Element<Message>>::new();
		for (layout_name, _) in &global.project.layouts {
			let item = button(layout_name.as_str());
			list_items.push(item.into());
		}
		let side_panel = column(list_items.into_iter());

		// Statusbar: HUD of relevant information.
		let statusbar = row![];

		column![row![viewport, side_panel], statusbar].into()
	}
}
