# SPDX-FileCopyrightText: 2026 Twilit Jack <twilit-jack@gmail.com>
# SPDX-License-Identifier: GPL-3.0-or-later
extends Node2D

@export var zoom_sensitivity := 0.1


func _input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		_handle_zoom(event)
	elif event is InputEventMouseMotion:
		_handle_pan(event)


func _handle_zoom(event: InputEventMouseButton) -> void:
	if event.button_index == MOUSE_BUTTON_WHEEL_UP:
		$Camera2D.zoom *= (1.0 + zoom_sensitivity)
	elif event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
		$Camera2D.zoom /= (1.0 + zoom_sensitivity)


func _handle_pan(event: InputEventMouseMotion) -> void:
	if not (Input.is_mouse_button_pressed(MOUSE_BUTTON_MIDDLE)
		or Input.is_mouse_button_pressed(MOUSE_BUTTON_LEFT) ):
		return

	global_position -= event.relative / $Camera2D.zoom
