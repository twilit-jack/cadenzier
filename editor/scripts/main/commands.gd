extends Node

@onready var commands: Dictionary[String, Callable] = {
	#region ===== INTERNAL COMMANDS =====
	"set_mode": func(mode: State.Mode) -> void:
		%State.mode = mode,
	"display_error": func(error: String) -> void:
		%ErrorDialog.dialog_text = error
		%ErrorDialog.show(),
	#endregion

	#region ===== MENU COMMANDS =====
	"open_settings": %Settings.show,
	"open_project": %ProjectOpenFileDialog.show,
	"save_project": write.bind(%State.project_path),
	"save_project_as": func() -> void:
		write.bind(%State.project_path),
	#endregion
}


func _ready() -> void:
	%ProjectOpenFileDialog.file_selected.connect(load_project)


func write(path: String) -> void:
	var bytes: Variant = CDNZ.serialize(%State.project)
	# If it's `Error`, it would be an implementation bug
	assert(bytes is PackedByteArray, "Error: {0}".format([error_string(bytes)]))

	var file := FileAccess.open(path, FileAccess.WRITE)
	if not file:
		commands["display_error"].call(
				"Open failed: " + str(FileAccess.get_open_error()))
	file.store_buffer(bytes)
	file.close()


func load_project(path: String) -> void:
	var result: Variant = CDNZ.load_from_file(path)
	if result is not Dictionary:
		commands["display_error"].call(
				"Open failed: " + str(result))
	%State.project = result
