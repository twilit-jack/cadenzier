// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod pane;

use self::pane::Pane;
use crate::{
	config::Config,
	gui::style::{
		self,
		icons::{Icon, icon},
	},
};

use iced::{
	Element, Fill, Padding,
	widget::{
		button, container,
		pane_grid::{self, PaneGrid},
		responsive, row,
	},
};

pub struct Panes {
	pub panes: pane_grid::State<Pane>,
	pub focus: Option<pane_grid::Pane>,
}

impl Default for Panes {
	fn default() -> Self {
		let (panes, pane) = pane_grid::State::new(Pane::default());

		Panes {
			panes,
			focus: Some(pane),
		}
	}
}

#[derive(Debug, Clone)]
pub enum Message {
	Pane(pane_grid::Pane, pane::Message),

	Split(pane_grid::Axis, pane_grid::Pane),
	SplitFocused(pane_grid::Axis),
	FocusAdjacent(pane_grid::Direction),
	Clicked(pane_grid::Pane),
	Dragged(pane_grid::DragEvent),
	Resized(pane_grid::ResizeEvent),
	Maximize(pane_grid::Pane),
	Restore,
	Close(pane_grid::Pane),
	CloseFocused,
}

impl Panes {
	pub fn update(&mut self, message: Message, project: &mut cdnz::Project) {
		match message {
			Message::Pane(pane, message) => {
				if let Some(pane) = self.panes.get_mut(pane) {
					pane.update(message, project);
				};
			}

			Message::Split(axis, pane) => {
				if let Some((pane, _)) = self.panes.split(axis, pane, Pane::default()) {
					self.focus = Some(pane);
				}
			}
			Message::SplitFocused(axis) => {
				if let Some(pane) = self.focus {
					if let Some((pane, _)) = self.panes.split(axis, pane, Pane::default()) {
						self.focus = Some(pane);
					}
				}
			}
			Message::FocusAdjacent(direction) => {
				if let Some(pane) = self.focus
					&& let Some(adjacent) = self.panes.adjacent(pane, direction)
				{
					self.focus = Some(adjacent);
				}
			}
			Message::Clicked(pane) => {
				self.focus = Some(pane);
			}
			Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
				self.panes.resize(split, ratio);
			}
			Message::Dragged(pane_grid::DragEvent::Dropped { pane, target }) => {
				self.panes.drop(pane, target);
			}
			Message::Dragged(_) => {}
			Message::Maximize(pane) => self.panes.maximize(pane),
			Message::Restore => {
				self.panes.restore();
			}
			Message::Close(pane) => {
				if let Some((_, sibling)) = self.panes.close(pane) {
					self.focus = Some(sibling);
				}
			}
			Message::CloseFocused => {
				if let Some(pane) = self.focus
					&& let Some((_, sibling)) = self.panes.close(pane)
				{
					self.focus = Some(sibling);
				}
			}
		}
	}

	pub fn view<'a>(
		&'a self,
		config: &'a Config,
		project: &'a cdnz::Project,
	) -> Element<'a, Message> {
		let total_panes = self.panes.len();

		let pane_grid = PaneGrid::new(&self.panes, |id, pane, is_maximized| {
			let is_focused = self.focus == Some(id);

			let title = row![].spacing(5);

			let title_bar = pane_grid::TitleBar::new(title)
				.controls(pane_grid::Controls::dynamic(
					view_controls(id, total_panes, is_maximized),
					button(icon(Icon::Close).size(14))
						.style(button::danger)
						.on_press_maybe(if total_panes > 1 {
							Some(Message::Close(id))
						} else {
							None
						}),
				))
				.padding(Padding {
					top: 2.0,
					right: 8.0,
					bottom: 2.0,
					left: 8.0,
				})
				.style(if is_focused {
					style::title_bar_focused
				} else {
					style::title_bar_active
				});

			pane_grid::Content::new(responsive(move |_size| {
				pane.view(config, project)
					.map(move |message| Message::Pane(id, message))
			}))
			.title_bar(title_bar)
			.style(if is_focused {
				style::pane_focused
			} else {
				style::pane_active
			})
		})
		.width(Fill)
		.height(Fill)
		.spacing(10)
		.on_click(Message::Clicked)
		.on_drag(Message::Dragged)
		.on_resize(10, Message::Resized);

		container(pane_grid).padding(10).into()
	}
}

fn view_controls<'a>(
	pane: pane_grid::Pane,
	total_panes: usize,
	is_maximized: bool,
) -> Element<'a, Message> {
	let maximize = if total_panes > 1 {
		let (content, message) = if is_maximized {
			(Icon::Compress, Message::Restore)
		} else {
			(Icon::Maximize, Message::Maximize(pane))
		};

		Some(
			button(container(icon(content).size(14)))
				.style(button::secondary)
				.padding(3)
				.on_press(message),
		)
	} else {
		None
	};

	let close = button(icon(Icon::Close).size(14))
		.style(button::danger)
		.padding(3)
		.on_press_maybe(if total_panes > 1 {
			Some(Message::Close(pane))
		} else {
			None
		});

	row![maximize, close].spacing(5).into()
}
