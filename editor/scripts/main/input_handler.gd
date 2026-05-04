# SPDX-FileCopyrightText: 2026 Twilit Jack <twilit-jack@gmail.com>
# SPDX-License-Identifier: GPL-3.0-or-later

extends Node

@onready var keybinds: Dictionary[State.Mode, Dictionary] = {
	State.Mode.NORMAL: {
		{ "key": Key.KEY_I }: %Commands.commands
				["set_mode"].bind(State.Mode.INSERT),

		{ "key": Key.KEY_A }: %Commands.commands
				["set_mode"].bind(State.Mode.INSERT),
		{ "key": Key.KEY_G }: %Commands.commands
				["set_mode"].bind(State.Mode.GOTO),
		{ "key": Key.KEY_SEMICOLON, "shift": true }: %Commands.commands
				["set_mode"].bind(State.Mode.COMMAND),

		{ "key": Key.KEY_V }: %Commands.commands
				["set_mode"].bind(State.Mode.SELECT),

		{ "key": Key.KEY_SPACE }: %Commands.commands
				["set_mode"].bind(State.Mode.SPACE),
	},

	State.Mode.INSERT: {
		{ "key": Key.KEY_ESCAPE }: %Commands.commands
				["set_mode"].bind(State.Mode.NORMAL),
	},

	State.Mode.SELECT: {
		{ "key": Key.KEY_ESCAPE }: %Commands.commands
				["set_mode"].bind(State.Mode.NORMAL),
	},

	State.Mode.COMMAND: {
		{ "key": Key.KEY_ESCAPE }: %Commands.commands
				["set_mode"].bind(State.Mode.NORMAL),
	},

	State.Mode.SPACE: {
		{ "key": Key.KEY_ESCAPE }: %Commands.commands
				["set_mode"].bind(State.Mode.NORMAL),
	},

	State.Mode.GOTO: {
		{ "key": Key.KEY_ESCAPE }: %Commands.commands
				["set_mode"].bind(State.Mode.NORMAL),
	},

	State.Mode.VIEW: {
		{ "key": Key.KEY_ESCAPE }: %Commands.commands
				["set_mode"].bind(State.Mode.NORMAL),
	},

	State.Mode.MATCH: {
		{ "key": Key.KEY_ESCAPE }: %Commands.commands
				["set_mode"].bind(State.Mode.NORMAL),
	},
}


func _unhandled_input(event: InputEvent) -> void:
	if event is not InputEventKey or not event.is_pressed():
		return

	var keybind := { "key": event.keycode }
	if event.shift_pressed:
		keybind["shift"] = event.shift_pressed
	if event.ctrl_pressed:
		keybind["ctrl"] = event.ctrl_pressed
	if event.alt_pressed:
		keybind["alt"] = event.alt_pressed

	var action: Variant = keybinds[%State.mode].get(keybind)
	if action is not Callable:
		return
	action.call()
