extends Label


func _ready() -> void:
	%State.mode_changed.connect(update)


func update(mode: State.Mode) -> void:
	match mode:
		State.Mode.NORMAL, State.Mode.SPACE, State.Mode.GOTO, State.Mode.VIEW, State.Mode.MATCH:
			text = "NOR"
			add_theme_color_override(&"font_color", Color.WHITE)
		State.Mode.INSERT:
			text = "INS"
			add_theme_color_override(&"font_color", Color.LIGHT_GREEN)
		State.Mode.SELECT:
			text = "SEL"
			add_theme_color_override(&"font_color", Color.SKY_BLUE)
		State.Mode.COMMAND:
			text = "CMD"
			add_theme_color_override(&"font_color", Color.GOLD)
