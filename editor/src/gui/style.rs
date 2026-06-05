// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod icons;

use iced::widget::container;
use iced::{Border, Theme};

pub fn title_bar_active(theme: &Theme) -> container::Style {
	let palette = theme.extended_palette();

	container::Style {
		text_color: Some(palette.background.strong.text),
		background: Some(palette.background.strong.color.into()),
		..Default::default()
	}
}

pub fn title_bar_focused(theme: &Theme) -> container::Style {
	let palette = theme.extended_palette();

	container::Style {
		text_color: Some(palette.primary.strong.text),
		background: Some(palette.primary.strong.color.into()),
		..Default::default()
	}
}

pub fn pane_active(theme: &Theme) -> container::Style {
	let palette = theme.extended_palette();

	container::Style {
		background: Some(palette.background.weak.color.into()),
		border: Border {
			width: 2.0,
			color: palette.background.strong.color,
			..Border::default()
		},
		..Default::default()
	}
}

pub fn pane_focused(theme: &Theme) -> container::Style {
	let palette = theme.extended_palette();

	container::Style {
		background: Some(palette.background.weak.color.into()),
		border: Border {
			width: 2.0,
			color: palette.primary.strong.color,
			..Border::default()
		},
		..Default::default()
	}
}
