// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me>
// SPDX-License-Identifier: AGPL-3.0-or-later

use iced::{
	Font,
	widget::{Text, text},
};

pub const FONT_REGULAR: Font = Font::with_name("Font Awesome 7 Free Regular");
pub const FONT_SOLID: Font = Font::with_name("Font Awesome 7 Free Solid");
pub const FONT_BRANDS: Font = Font::with_name("Font Awesome 7 Brands");

pub enum Icon {
	Close,
	Maximize,
	Compress,
}

impl Icon {
	fn get_unicode(&self) -> &'static str {
		match self {
			Icon::Close => "\u{58}",
			Icon::Maximize => "\u{f065}",
			Icon::Compress => "\u{f066}",
		}
	}
}

#[allow(dead_code)] // Required by `styled_icon`, which I might need in the future
pub enum IconStyle {
	Regular,
	Solid,
	Brands,
}

impl IconStyle {
	fn get_font(&self) -> Font {
		match self {
			IconStyle::Regular => FONT_REGULAR,
			IconStyle::Solid => FONT_SOLID,
			IconStyle::Brands => FONT_BRANDS,
		}
	}
}

pub fn icon<'a, Theme, Renderer>(icon: Icon) -> Text<'a, Theme, Renderer>
where
	Theme: text::Catalog + 'a,
	Renderer: iced_core::text::Renderer,
	<Renderer as iced_core::text::Renderer>::Font: From<Font>,
{
	text(icon.get_unicode()).font(FONT_REGULAR).size(20).into()
}

#[allow(dead_code)] // I might need this in the future
pub fn styled_icon<'a, Theme, Renderer>(icon: Icon, style: IconStyle) -> Text<'a, Theme, Renderer>
where
	Theme: text::Catalog + 'a,
	Renderer: iced_core::text::Renderer,
	<Renderer as iced_core::text::Renderer>::Font: From<Font>,
{
	text(icon.get_unicode())
		.font(style.get_font())
		.size(20)
		.into()
}
