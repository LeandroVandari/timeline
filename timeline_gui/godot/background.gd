extends Node2D

@onready var viewport: Viewport = get_viewport()
var offset: float = 0.
const NUM_LINES: int = 10

func _draw() -> void:
	print_verbose("Redrawing background lines...")
	var size = viewport.get_visible_rect().size
	var line_dist = size.x / NUM_LINES
	offset = fposmod(offset, line_dist)
	for i in range(NUM_LINES):
		draw_line(
			Vector2(-size.x / 2 + offset + line_dist * i, -size.y / 2),
			Vector2(-size.x / 2 + offset + line_dist * i, size.y / 2),
			Color.DARK_GRAY
		)


func _input(event: InputEvent) -> void:
	if event is InputEventMouseMotion and Input.is_action_pressed("timeline_drag"):
		offset += event.relative.x


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	if Input.is_action_pressed("timeline_drag"):
		queue_redraw()
