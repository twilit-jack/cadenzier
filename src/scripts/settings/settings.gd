extends TabContainer

#@export var structure: Array[Dictionary] = [
#	{
#		"name": "Viewport",
#		"items": [
#			{
#				"type": ItemType.TOGGLE,
#				"name": "Floaty Viewport",
#				"description": "Makes panning the viewport have an acceleration effect, "
#						+ "and zooming have a slight ease in and out.",
#			},
#		],
#	},
#]

@export var structure: Array[ConfigTab] = [
	ConfigTab.new("Viewport", [
		ToggleItem.new(
			"Floaty Viewport",
			"Makes panning the viewport have an acceleration effect, "
				+ "and zooming have a slight ease in and out.",
		),
	]),

	ConfigTab.new("Input", [
		OptionItem.new(
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
	]),
]


func _ready() -> void:
	for tab in structure:
		var cont := GridContainer.new()
		var tab_name: String = tab.name
		cont.name = tab_name
		cont.columns = 2

		for item in tab.items:
			var nodes: Array[Control]
			match item.type:
				ItemType.TOGGLE:
					nodes = _create_toggle(item)
			cont.add_child(nodes[0])
			cont.add_child(nodes[1])

		self.add_child(cont)


func _create_toggle(item: Dictionary) -> Array[Control]:
	var label := Label.new()
	label.text = item.name

	var check_box := CheckBox.new()

	return [label, check_box]


class ConfigTab:
	var name: String
	var items: Array[ConfigItem]

	func _init(name: String, items: Array[ConfigItem] = []) -> void:
		self.name = name
		self.items = items


class ConfigItem:
	var name: String
	var description: String

	func _init(name: String, description: String = "") -> void:
		self.name = name
		self.description = description


class ToggleItem:
	extends ConfigItem

	func _init(name: String, description: String = "") -> void:
		super(name, description)


class SliderItem:
	extends ConfigItem

	var min: Float
	var max: Float
	var step: Float

	func _init(
			name: String, description: String = "",
			min: Float = 0.0, max: Float = 1.0, step: Float = 0.1,
	) -> void:
		super(name, description)
		self.min = min
		self.max = max
		self.step = step


class OptionItem:
	extends ConfigItem

	var options: Array[String]

	func _init(
			name: String, description: String = "",
			options: Array[String],
	) -> void:
		super(name, description)
		self.options = options
