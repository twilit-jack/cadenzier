# SPDX-FileCopyrightText: 2026 Twilit Jack <twilit-jack@gmail.com>
# SPDX-License-Identifier: GPL-3.0-or-later

extends Node
## Contains user config and provides a way to access it through [method get_value].

var config: Dictionary[String, ConfigCategory] = {
	"viewport": ConfigCategory.new("Viewport", {
		"zoom_sensitivity": SliderItem.new(
			"Zoom Sensitivity",
			"Makes panning the viewport have an acceleration effect, "
				+ "and zooming have a slight ease in and out.",
				0.1,
				0.002,
				0.5,
				0.002,
		),
	}),

	"input": ConfigCategory.new("Input", {
		"note_language": OptionItem.new(
			"Note Language",
			"Sets the note language used when interacting with the score (e.g. "
				+ "inserting notes).\n"
				+ "\n"
				+ "Common options are English (`c d fs bf`), Nederlands (`c d "
				+ "fis bes`), and Italiano (`do re fad sib`).\n"
				+ "\n"
				+ "NOTE: This only sets the language used in Cadenza.\n"
				+ "LilyPond export always uses Nederlands except if explicitly"
				+ "set.",

			"Nederlands",
			[
				"Nederlands",
				"Català",
				"Deutsch",
				"English",
				"Español",
				"Français",
				"Italiano",
				"Norsk",
				"Português",
				"Suomi",
				"Svenska",
				"Vlaams",
			],
		),
	}),
}


## Returns the value of the config, or an [constant ERR_DOES_NOT_EXIST].
func get_value(category_name: String, item_name: String) -> Variant:
	var category: ConfigCategory = config.get(category_name)
	if category == null:
		return ERR_DOES_NOT_EXIST

	var item: ConfigItem = category.get(item_name)
	if item == null:
		return ERR_DOES_NOT_EXIST

	return item.value


class ConfigCategory:
	var name: String
	var items: Dictionary[String, ConfigItem]


	func _init(p_name: String, p_items: Dictionary[String, ConfigItem]) -> void:
		name = p_name
		items = p_items


@abstract class ConfigItem:
	var name: String
	var description: String
	var value: Variant


	func _init(p_name: String, p_description: String, default_value: Variant) -> void:
		name = p_name
		description = p_description
		value = default_value


class ToggleItem:
	extends ConfigItem


	func _init(p_name: String, p_description: String, default_value: bool) -> void:
		super(p_name, p_description, default_value)


class SliderItem:
	extends ConfigItem

	var min_val: float
	var max_val: float
	var step_val: float

	func _init(
			p_name: String, p_description: String, default_value: float,
			p_min_val: float = 0.0, p_max_val: float = 1.0, p_step_val: float = 0.1,
	) -> void:
		super(p_name, p_description, default_value)
		min_val = p_min_val
		max_val = p_max_val
		step_val = p_step_val


class OptionItem:
	extends ConfigItem

	var options: Array[String]

	func _init(
			p_name: String, p_description: String, default_value: String,
			p_options: Array[String],
	) -> void:
		super(p_name, p_description, default_value)
		options = p_options
