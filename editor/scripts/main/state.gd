# SPDX-FileCopyrightText: 2026 Twilit Jack <twilit-jack@gmail.com>
# SPDX-License-Identifier: GPL-3.0-or-later

class_name State
extends Node

enum Mode {
	NORMAL,
	INSERT,
	SELECT,
	COMMAND,
	SPACE,
	GOTO,
	VIEW,
	MATCH,
}

signal mode_changed(mode: Mode)

var mode: Mode = Mode.NORMAL:
	set(value):
		if mode != value:
			mode = value
			mode_changed.emit(mode)

var selections: Array[Selection]

var project: Dictionary
var project_path: String


class Selection:
	var anchor: int
	var cursor: int
