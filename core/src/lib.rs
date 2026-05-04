// SPDX-FileCopyrightText: 2026 Twilit Jack <twilit-jack@gmail.com>
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod cdnz_godot;

use godot::prelude::*;

pub struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
