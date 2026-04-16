# SPDX-FileCopyrightText: 2026 Twilit Jack <twilit-jack@gmail.com>
# SPDX-License-Identifier: GPL-3.0-or-later
class_name State
extends Node

enum EditorMode {
	NORMAL,
	INSERT,
	SELECT,
	COMMAND,
	SPACE,
	GOTO,
	VIEW,
	MATCH,
}

var editor_mode: EditorMode
var selections: Array[Selection]


func _ready() -> void:
	pass


func set_editor_mode(mode: EditorMode) -> void:
	editor_mode = mode
	%Statusbar.update_mode_indicator(mode)


class Selection:
	var anchor: int
	var cursor: int
