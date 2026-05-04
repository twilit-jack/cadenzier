# SPDX-FileCopyrightText: 2026 Twilit Jack <twilit-jack@gmail.com>
# SPDX-License-Identifier: GPL-3.0-or-later

extends Node2D

const PAGE_SEPARAION_PADDING := 8.0

@onready var pages_node := $Pages


func load_svgs(file_paths: Array[String]) -> void:
	for child in pages_node.get_children():
		pages_node.remove_child(child)
		child.queue_free()

	var width_sum := 0.0
	for path in file_paths:
		var image := Image.load_from_file(path)
		if not image:
			push_error("Failed to load SVG from path: '{0}'".format([path]))
			continue

		var texture := ImageTexture.create_from_image(image)

		var sprite := Sprite2D.new()
		sprite.global_position.x = width_sum
		sprite.texture = texture
		$Pages.add_child(sprite)

		width_sum += texture.get_size().x + PAGE_SEPARAION_PADDING
