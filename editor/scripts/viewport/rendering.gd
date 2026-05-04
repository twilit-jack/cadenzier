extends Node2D


func add_part(part: Dictionary) -> void:
	var part_node := Node2D.new()

	self.add_child(part_node)
