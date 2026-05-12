// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

use iced::{
	Element, Length, Renderer, Theme,
	widget::{Column, button, column, container, row, text},
};

pub(super) fn run() -> iced::Result {
	iced::application(State::default, update, view).run()
}

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
struct WriteModeState {}

#[derive(Debug, Clone)]
enum Message {
	ModeSwitch(EditorMode),
	LayoutSelect(cdnz::LayoutName),
}

fn update(state: &mut State, message: Message) {
	match message {
		Message::ModeSwitch(mode) => state.editor_mode = mode,
		Message::LayoutSelect(name) => state.selected_layout = name,
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
	let mut buttons: Vec<Element<'_, Message, Theme, Renderer>> = Vec::new();
	for (name, _layout) in &state.project.layouts {
		buttons.push(
			button(name.as_str())
				.on_press(Message::LayoutSelect(name.clone()))
				.into(),
		);
	}
	let side_panel = Column::from_vec(buttons);

	column![
		row![view_viewport(state), side_panel],
		view_status_bar(state)
	]
	.into()
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
