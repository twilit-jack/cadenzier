// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::config::Config;

use iced::{
	Element,
	widget::{button, center, column, row, rule::horizontal as h_rule, scrollable, svg, text},
};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Render {
	pub selected_layout: Option<cdnz::LayoutName>,
	pub svgs: HashMap<cdnz::LayoutName, Vec<svg::Handle>>,
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
		let viewport = if let Some(selected_layout) = &self.selected_layout {
			let svg_handles = self
				.svgs
				.get(selected_layout)
				.expect("`selected_layout` should be an existing layout in the project");
			let row = row(svg_handles.iter().cloned().map(svg).map(Element::from));
			scrollable(row)
		} else {
			scrollable(center("No layout selected."))
		};

		// Side panel: List of buttons for switching between different layouts (i.e. instrument
		// parts, conductor's score, etc.).
		let mut list_items =
			Vec::<Element<Message>>::from([text("Layouts").into(), h_rule(2).into()]);
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
