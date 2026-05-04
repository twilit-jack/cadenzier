# SPDX-FileCopyrightText: 2026 Twilit Jack <twilit-jack@gmail.com>
# SPDX-License-Identifier: GPL-3.0-or-later

extends TabContainer


func _ready() -> void:
	for category in Config.config.values():
		var node_pairs: Array[Array]
		for item in category.items.values():
			var item_nodes: Array[Control]
			if item is Config.BoolItem:
				item_nodes = _build_toggle_item(item)
			elif item is Config.FloatItem:
				item_nodes = _build_slider_item(item)
			elif item is Config.OptionItem:
				item_nodes = _build_option_item(item)
			node_pairs.append(item_nodes)

		# HBoxContainer that splits settings into left side and right side
		var h_box := HBoxContainer.new()
		h_box.size_flags_horizontal = Control.SIZE_EXPAND_FILL
		h_box.add_theme_constant_override("separation", 64)

		# Left side containing config interface nodes
		var l_cont := GridContainer.new()
		l_cont.name = category.name
		l_cont.columns = 2
		l_cont.size_flags_horizontal = Control.SIZE_EXPAND_FILL
		l_cont.add_theme_constant_override("h_separation", 32)

		# Right side containing config interface nodes
		var r_cont := GridContainer.new()
		r_cont.name = category.name
		r_cont.columns = 2
		r_cont.size_flags_horizontal = Control.SIZE_EXPAND_FILL
		r_cont.add_theme_constant_override("h_separation", 32)

		# Bucket node_pairs intermittently into `l_cont` and `r_cont`
		for i in node_pairs.size():
			if i % 2 == 0:
				l_cont.add_child(node_pairs[i][0])
				l_cont.add_child(node_pairs[i][1])
			else:
				r_cont.add_child(node_pairs[i][0])
				r_cont.add_child(node_pairs[i][1])

		h_box.add_child(l_cont)
		h_box.add_child(r_cont)

		var margin_cont := MarginContainer.new()
		margin_cont.name = category.name
		margin_cont.add_child(h_box)
		margin_cont.add_theme_constant_override("margin_left", 16)
		margin_cont.add_theme_constant_override("margin_right", 16)
		margin_cont.add_theme_constant_override("margin_top", 12)
		margin_cont.add_theme_constant_override("margin_bottom", 12)

		self.add_child(margin_cont)


func _build_toggle_item(item: Config.BoolItem) -> Array[Control]:
	var label := Label.new()
	label.text = item.name
	label.tooltip_text = item.description

	var spacer := Control.new()
	spacer.size_flags_horizontal = Control.SIZE_EXPAND_FILL

	var check_box := CheckBox.new()
	check_box.toggled.connect(func(new_value: bool) -> void:
		item.value = new_value
	)

	var check_box_cont := HBoxContainer.new()
	check_box_cont.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	check_box_cont.add_child(spacer)
	check_box_cont.add_child(check_box)

	return [label, check_box_cont]


func _build_slider_item(item: Config.FloatItem) -> Array[Control]:
	var label := Label.new()
	label.text = item.name
	label.tooltip_text = item.description

	var indicator := Label.new()
	indicator.text = str(item.value)
	indicator.custom_minimum_size.x = 48
	indicator.horizontal_alignment = HORIZONTAL_ALIGNMENT_RIGHT

	var slider := HSlider.new()
	slider.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	slider.size_flags_vertical = Control.SIZE_EXPAND_FILL
	slider.value_changed.connect(func(new_value: float) -> void:
		item.value = new_value
		indicator.text = str(new_value)
	)

	var slider_cont := HBoxContainer.new()
	slider_cont.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	slider_cont.add_child(indicator)
	slider_cont.add_child(slider)

	return [label, slider_cont]


func _build_option_item(item: Config.OptionItem) -> Array[Control]:
	var label := Label.new()
	label.text = item.name
	label.tooltip_text = item.description

	var option_button := OptionButton.new()
	option_button.size_flags_horizontal = Control.SIZE_EXPAND_FILL

	for option_item in item.options:
		option_button.add_item(option_item)

	option_button.item_selected.connect(func(idx: int) -> void:
		item.value = item.options[idx]
	)

	return [label, option_button]
