# SPDX-FileCopyrightText: 2026 Twilit Jack <twilit-jack@gmail.com>
# SPDX-License-Identifier: GPL-3.0-or-later

extends Camera2D


func _input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		_handle_zoom(event as InputEventMouseButton)
	elif event is InputEventMouseMotion:
		_handle_pan(event as InputEventMouseMotion)


func _handle_zoom(event: InputEventMouseButton) -> void:
	var zoom_factor: float = 0.0

	if event.button_index == MOUSE_BUTTON_WHEEL_UP:
		zoom_factor = 1.0 + Config.get_float("viewport", "zoom_sensitivity")
	elif event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
		zoom_factor = 1.0 / (1.0 + Config.get_float("viewport", "zoom_sensitivity"))
	else:
		return

	if Config.get_bool("viewport", "zoom_to_cursor"):
		var mouse_pos := get_global_mouse_position()

		zoom *= zoom_factor
		var new_mouse_pos := get_global_mouse_position()
		force_update_scroll() # These are needed to avoid Godot caching bug

		global_position += (mouse_pos - new_mouse_pos)
		force_update_scroll()
	else:
		zoom *= zoom_factor

	get_tree().root.set_input_as_handled()


func _handle_pan(event: InputEventMouseMotion) -> void:
	if not (Input.is_mouse_button_pressed(MOUSE_BUTTON_MIDDLE)
			or Input.is_mouse_button_pressed(MOUSE_BUTTON_LEFT)):
		return

	global_position -= event.relative / zoom

	get_tree().root.set_input_as_handled()
