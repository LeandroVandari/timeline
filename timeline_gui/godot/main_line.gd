extends Line2D

@onready var viewport: Viewport = get_viewport()
var offset: float = 0


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	viewport.connect("size_changed", _on_viewport_resized)
	place_self()


func _input(event: InputEvent) -> void:
	if event is InputEventMouseMotion and Input.is_action_pressed("timeline_drag"):
		self.offset += event.relative.y
		self.position.y += event.relative.y


func place_self() -> void:
	var size = viewport.get_visible_rect().size
	self.add_point(Vector2(-size.x / 2, self.offset))
	self.add_point(Vector2(size.x / 2, self.offset))


func _on_viewport_resized() -> void:
	self.clear_points()
	place_self()
