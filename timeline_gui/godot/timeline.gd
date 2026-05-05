extends Timeline


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	self.replace_by(Timeline.load_from_file("/Users/administrador/programming/rust/timeline/timeline_cli/test.json"))
