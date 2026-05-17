// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod config;
mod gui;

fn main() -> iced::Result {
	// gui is separated here for adding a cli down the line.
	gui::run()
}
