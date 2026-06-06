// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::BTreeMap;

use super::{Editor, Message, tab};
use crate::{
	config::keyboard::{Command, Context, Keybind},
	gui::tab::pane::PaneContent,
};

use iced::{
	Subscription, keyboard,
	widget::pane_grid::{Axis, Direction},
};

impl Editor {
	pub fn keyboard(&self) -> Subscription<Message> {
		let tab = self
			.tabs
			.get(self.selected_tab)
			.expect("selected tab should exist");
		let focused_pane_id = tab.focus;

		// Identify active contexts
		let mut active_contexts = vec![Context::Global, Context::View];

		if let Some(id) = focused_pane_id {
			if let Some(pane) = tab.panes.get(id) {
				active_contexts.push(match pane.content {
					PaneContent::Blank(_) => Context::Blank,
					PaneContent::Render(_) => Context::Render,
					PaneContent::Setup(_) => Context::Setup,
					PaneContent::Write(_) => Context::Write,
					PaneContent::Help(_) => Context::Help,
				});
			}
		}

		// Aggregate keybinds from all active contexts
		let mut combined_binds = BTreeMap::new();
		for ctx in active_contexts {
			if let Some(map) = self.config.keybinds.get(&ctx) {
				combined_binds.extend(map.clone());
			}
		}

		if combined_binds.is_empty() {
			return Subscription::none();
		}

		keyboard::listen()
			.with(combined_binds)
			.filter_map(|(binds, event)| {
				let keybind = Keybind::from_event(event)?;
				binds.get(&keybind).cloned()
			})
			.filter_map(|command| {
				#[allow(unused_imports)] // Will be expanded in the near future
				use crate::config::keyboard::{
					BlankCmd, GlobalCmd, HelpCmd, RenderCmd, SetupCmd, ViewCmd, WriteCmd,
				};
				Some(match command {
					Command::Global(command) => match command {
						GlobalCmd::DebugPrint => Message::DebugPrint,
					},
					Command::View(command) => match command {
						ViewCmd::FocusLeft => {
							Message::Tab(tab::Message::FocusAdjacent(Direction::Left))
						}
						ViewCmd::FocusRight => {
							Message::Tab(tab::Message::FocusAdjacent(Direction::Right))
						}
						ViewCmd::FocusDown => {
							Message::Tab(tab::Message::FocusAdjacent(Direction::Down))
						}
						ViewCmd::FocusUp => {
							Message::Tab(tab::Message::FocusAdjacent(Direction::Up))
						}
						ViewCmd::SplitFocusedVertical => {
							Message::Tab(tab::Message::SplitFocused(Axis::Vertical))
						}
						ViewCmd::SplitFocusedHorizontal => {
							Message::Tab(tab::Message::SplitFocused(Axis::Horizontal))
						}
					},
					Command::Blank(command) => match command {
						//BlankCmd:: => Message::,
					},
					Command::Render(command) => match command {
						//RenderCmd:: => Message::,
					},
					Command::Setup(command) => match command {
						//SetupCmd:: => Message::,
					},
					Command::Write(command) => match command {
						//WriteCmd:: => Message::,
					},
					Command::Help(command) => match command {
						//HelpCmd:: => Message::,
					},
				})
			})
	}
}
