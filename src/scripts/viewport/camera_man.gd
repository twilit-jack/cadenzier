# SPDX-FileCopyrightText: 2026 Twilit Jack <twilit-jack@gmail.com>
# SPDX-License-Identifier: GPL-3.0-or-later
extends Node2D

@export var zoom_sensitivity := 0.01


func _input(event: InputEvent) -> void:
	match event:
		InputEventMouseButton:
			_handle_zoom(event)
		InputEventMouseMotion:
			_handle_pan(event)


func _handle_zoom(event: InputEventMouseButton) -> void:
	match event.button_index:
		MOUSE_BUTTON_WHEEL_UP:
			$Camera2D.zoom -= Vector2(zoom_sensitivity, zoom_sensitivity)
		MOUSE_BUTTON_WHEEL_DOWN:
			$Camera2D.zoom += Vector2(zoom_sensitivity, zoom_sensitivity)


func _handle_pan(event: InputEventMouseMotion) -> void:
	if not (
		Input.is_mouse_button_pressed(MOUSE_BUTTON_MIDDLE)
		or Input.is_mouse_button_pressed(MOUSE_BUTTON_LEFT)
	):
		return

	global_position += event.relative
	print("Panning: ", event.relative)
