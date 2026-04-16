# SPDX-FileCopyrightText: 2026 Twilit Jack <twilit-jack@gmail.com>
# SPDX-License-Identifier: GPL-3.0-or-later
class_name Statusbar
extends HBoxContainer

const EditorMode = State.EditorMode

@onready var mode_indicator: Label = $ModeIndicator
@onready var selection_count: Label = $SelectionCount
@onready var line_number: Label = $LineNumber


func _ready() -> void:
	assert(mode_indicator, "Mode indicator not found")
	assert(selection_count, "Selection count not found")
	assert(line_number, "Line number not found")


func update_mode_indicator(mode: EditorMode):
	match mode:
		EditorMode.NORMAL, EditorMode.SPACE, EditorMode.GOTO, EditorMode.VIEW, EditorMode.MATCH:
			mode_indicator.text = "NOR"
		EditorMode.INSERT:
			mode_indicator.text = "INS"
		EditorMode.SELECT:
			mode_indicator.text = "SEL"
		EditorMode.COMMAND:
			mode_indicator.text = "CMD"


func update_selection_count():
	pass


func update_line_number():
	pass
