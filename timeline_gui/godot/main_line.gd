extends Line2D
class_name MainLine

@onready var viewport: Viewport = get_viewport()
static var vertical_offset: float = 0


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	viewport.connect("size_changed", _on_viewport_resized)
	place_self()


func _input(event: InputEvent) -> void:
	if event is InputEventMouseMotion and Input.is_action_pressed("timeline_drag"):
		self.vertical_offset += event.relative.y
		self.position.y += event.relative.y


func place_self() -> void:
	var size = viewport.get_visible_rect().size
	self.add_point(Vector2(-size.x / 2, self.vertical_offset))
	self.add_point(Vector2(size.x / 2, self.vertical_offset))


func _on_viewport_resized() -> void:
	self.clear_points()
	place_self()
