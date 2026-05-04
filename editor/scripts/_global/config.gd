# SPDX-FileCopyrightText: 2026 Twilit Jack <twilit-jack@gmail.com>
# SPDX-License-Identifier: GPL-3.0-or-later

extends Node
## Contains user config and provides a way to access it through [method get_value].

static var save_path := OS.get_cache_dir() + "settings.conf"

var config: Dictionary[String, ConfigCategory] = {
	"viewport": ConfigCategory.new("Viewport", {
		"zoom_sensitivity": FloatItem.new(
			"Zoom Sensitivity",
			"Makes panning the viewport have an acceleration effect, "
				+ "and zooming have a slight ease in and out.",
			0.1,
			0.001,
			1.0,
			0.001,
		),
		"zoom_to_cursor": BoolItem.new(
			"Zoom To Cursor",
			"If enabled, zooming moves the view towards the cursor.",
			true,
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


#region --- Getter functions ---
func get_bool(category_name: String, item_name: String) -> bool:
	var item: ConfigItem = _get_item(category_name, item_name)
	assert(item is BoolItem,
		"Requested item '{0}' in category '{0}' is not a BoolItem."
				.format([item_name, category_name]))
	return (item as BoolItem).value


func get_float(category_name: String, item_name: String) -> float:
	var item: ConfigItem = _get_item(category_name, item_name)
	assert(item is FloatItem,
		"Requested item '{0}' in category '{1}' is not a FloatItem."
				.format([item_name, category_name]))
	return (item as FloatItem).value


func get_option(category_name: String, item_name: String) -> String:
	var item: ConfigItem = _get_item(category_name, item_name)
	assert(item is OptionItem,
		"Requested item '{0}' in category '{1}' is not an OptionItem."
				.format([item_name, category_name]))
	return (item as OptionItem).value


func _get_item(category_name: String, item_name: String) -> ConfigItem:
	var category: ConfigCategory = config.get(category_name)
	assert(category, "Category '{0}' does not exist."
			.format([category_name]))

	var item: ConfigItem = category.items.get(item_name)
	assert(item, "Item '{0}' in category '{1}' does not exist."
			.format([item_name, category_name]))

	return item
#endregion


#region --- Save & Load ---
func save_to_disk() -> void:
	var cf := ConfigFile.new()
	
	for cat_id in config:
		var category := config[cat_id]
		for item_id in category.items:
			var item := category.items[item_id]
			@warning_ignore("unsafe_property_access") # `value` is present on all subtypes
			cf.set_value(cat_id, item_id, item.value)
	
	cf.save(save_path)


func load_from_disk() -> void:
	var cf := ConfigFile.new()
	var err := cf.load(save_path)
	
	# If file doesn't exist, we just keep the defaults defined in your dict
	if err != OK:
		return

	for cat_id in cf.get_sections():
		if not config.has(cat_id):
			return
		for item_id in cf.get_section_keys(cat_id):
			if not config[cat_id].items.has(item_id):
				return
			@warning_ignore("unsafe_property_access") # `value` is present on all subtypes
			config[cat_id].items[item_id].value = cf.get_value(cat_id, item_id)
#endregion


class ConfigCategory:
	var name: String
	var items: Dictionary[String, ConfigItem]

	func _init(p_name: String, p_items: Dictionary[String, ConfigItem]) -> void:
		name = p_name
		items = p_items


@abstract class ConfigItem:
	var name: String
	var description: String

	func _init(p_name: String, p_description: String) -> void:
		name = p_name
		description = p_description


class BoolItem extends ConfigItem:
	var value: bool

	func _init(p_name: String, p_description: String, default_value: bool) -> void:
		super(p_name, p_description)
		value = default_value


class IntItem extends ConfigItem:
	var value: int

	var min_val: int
	var max_val: int
	var step_val: int

	func _init(
			p_name: String, p_description: String, default_value: int,
			p_min_val: int = 0, p_max_val: int = 10, p_step_val: int = 1,
	) -> void:
		super(p_name, p_description)
		value = default_value
		min_val = p_min_val
		max_val = p_max_val
		step_val = p_step_val


class FloatItem extends ConfigItem:
	var value: float

	var min_val: float
	var max_val: float
	var step_val: float

	func _init(
			p_name: String, p_description: String, default_value: float,
			p_min_val: float = 0.0, p_max_val: float = 1.0, p_step_val: float = 0.1,
	) -> void:
		super(p_name, p_description)
		value = default_value
		min_val = p_min_val
		max_val = p_max_val
		step_val = p_step_val


class OptionItem extends ConfigItem:
	var value: String

	var options: Array[String]

	func _init(
			p_name: String, p_description: String, default_value: String,
			p_options: Array[String],
	) -> void:
		super(p_name, p_description)
		value = default_value
		options = p_options
