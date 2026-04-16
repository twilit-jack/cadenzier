# SPDX-FileCopyrightText: 2026 Twilit Jack <twilit-jack@gmail.com>
# SPDX-License-Identifier: GPL-3.0-or-later
extends Node

const EditorMode = State.EditorMode


# TODO: Turn input command- and config-driven
func _input(event: InputEvent) -> void:
	if event is not InputEventKey:
		return
	if not event.pressed:
		return

	match %State.editor_mode:
		EditorMode.NORMAL:
			_handle_normal_mode_input(event)
		EditorMode.INSERT:
			_handle_insert_mode_input(event)
		EditorMode.SELECT:
			_handle_select_mode_input(event)
		EditorMode.COMMAND:
			_handle_command_mode_input(event)
		EditorMode.SPACE:
			_handle_space_mode_input(event)
		EditorMode.GOTO:
			_handle_goto_mode_input(event)
		EditorMode.VIEW:
			_handle_view_mode_input(event)
		EditorMode.MATCH:
			_handle_match_mode_input(event)


func _handle_normal_mode_input(event: InputEvent) -> void:
	match event.keycode:
		KEY_W:
			print("W key pressed")
		KEY_A:
			print("A key pressed")
		_:
			return
	get_viewport().set_input_as_handled()


func _handle_insert_mode_input(event: InputEvent) -> void:
	match event.keycode:
		KEY_ESCAPE:
			%State.set_editor_mode(EditorMode.NORMAL)
		_:
			return
	get_viewport().set_input_as_handled()


func _handle_select_mode_input(event: InputEvent) -> void:
	match event.keycode:
		KEY_ESCAPE:
			%State.set_editor_mode(EditorMode.NORMAL)
		_:
			return
	get_viewport().set_input_as_handled()


func _handle_command_mode_input(event: InputEvent) -> void:
	match event.keycode:
		KEY_ESCAPE:
			%State.set_editor_mode(EditorMode.NORMAL)
		_:
			return
	get_viewport().set_input_as_handled()


func _handle_space_mode_input(event: InputEvent) -> void:
	match event.keycode:
		KEY_ESCAPE:
			%State.set_editor_mode(EditorMode.NORMAL)
		_:
			return
	get_viewport().set_input_as_handled()


func _handle_goto_mode_input(event: InputEvent) -> void:
	match event.keycode:
		KEY_ESCAPE:
			%State.set_editor_mode(EditorMode.NORMAL)
		_:
			return
	get_viewport().set_input_as_handled()


func _handle_view_mode_input(event: InputEvent) -> void:
	match event.keycode:
		KEY_ESCAPE:
			%State.set_editor_mode(EditorMode.NORMAL)
		_:
			return
	get_viewport().set_input_as_handled()


func _handle_match_mode_input(event: InputEvent) -> void:
	match event.keycode:
		KEY_ESCAPE:
			%State.set_editor_mode(EditorMode.NORMAL)
		_:
			return
	get_viewport().set_input_as_handled()
