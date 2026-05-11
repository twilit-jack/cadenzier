// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

use iced::{
	Element, Length,
	widget::{button, column, container, row, text},
};

#[derive(Debug, Default)]
struct State {
	editor_mode: EditorMode,
	write_mode_state: WriteModeState,

	project: cdnz::Project,
	selected_layout: cdnz::LayoutName,
}

#[derive(Debug, Clone, Copy, Default)]
enum EditorMode {
	#[default]
	Setup,
	Write,
	Help,
}

#[derive(Debug, Default)]
struct WriteModeState {
	pub side_panel_tab: SidePanelTab,
}

#[derive(Debug, Clone, Copy, Default)]
enum SidePanelTab {
	#[default]
	Layouts,
	Parts,
}

#[derive(Debug, Clone, Copy)]
enum Message {
	ModeSwitch(EditorMode),
	SidePanelTabSwitch(SidePanelTab),
}

pub(super) fn run() -> iced::Result {
	iced::application(State::default, update, view).run()
}

fn update(state: &mut State, message: Message) {
	match message {
		Message::ModeSwitch(mode) => state.editor_mode = mode,
		Message::SidePanelTabSwitch(tab) => state.write_mode_state.side_panel_tab = tab,
	}
}

fn view(state: &State) -> Element<'_, Message> {
	let mode_switch_buttons = row![
		// TODO: Make these have a selected appearence
		button("Setup").on_press(Message::ModeSwitch(EditorMode::Setup)),
		button("Write").on_press(Message::ModeSwitch(EditorMode::Write)),
		button("Help").on_press(Message::ModeSwitch(EditorMode::Help)),
	];

	let content = container(match state.editor_mode {
		EditorMode::Setup => text("Setup view placeholder").into(),
		EditorMode::Write => view_write_mode(state),
		EditorMode::Help => text("Documentation viewer placeholder").into(),
	})
	.padding(8);

	column![mode_switch_buttons, content].into()
}

// TODO: Make function `view_setup_mode`

fn view_write_mode(state: &State) -> Element<'_, Message> {
	column![
		row![view_viewport(state), view_side_panel(state)],
		view_status_bar(state)
	]
	.into()
}

fn view_side_panel(state: &State) -> Element<'_, Message> {
	let tab_switch_buttons = row![
		// TODO: Make these have a selected appearence
		button("Layouts").on_press(Message::SidePanelTabSwitch(SidePanelTab::Layouts)),
		button("Parts").on_press(Message::SidePanelTabSwitch(SidePanelTab::Parts)),
	];

	let content_list = match state.write_mode_state.side_panel_tab {
		SidePanelTab::Layouts => {
			// TODO: Make this use the actual layouts for the current project
			column![button("Full Score"), button("Flute"), button("Trumpet")]
		}
		SidePanelTab::Parts => {
			// TODO: Make this use the actual parts for the current project
			column![button("Flute"), button("Trumpet")]
		}
	};

	column![tab_switch_buttons, content_list].into()
}

fn view_status_bar(state: &State) -> Element<'_, Message> {
	// TODO: Make status bar
	row![].into()
}

fn view_viewport(state: &State) -> Element<'_, Message> {
	// TODO: Make viewport
	container(text("Viewport placeholder"))
		.width(Length::Fill)
		.height(Length::Fill)
		.into()
}

// TODO: Make function `view_doc_viewer`
