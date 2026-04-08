extends Node2D

@onready var viewport: Viewport = get_viewport()
var offset: Vector2 = Vector2(0., 0.)
const NUM_LINES: int = 10


# Called when the node enters the scene tree for the first time.
func _draw() -> void:
	print("Redrawing grid...")
	var size = viewport.get_visible_rect().size
	var line_dist = size / NUM_LINES
	offset = offset.posmodv(line_dist)
	for i in range(NUM_LINES):
		draw_line(
			Vector2(-size.x / 2, -size.y / 2 + offset.y + line_dist.y * i),
			Vector2(size.x / 2, -size.y / 2 + offset.y + line_dist.y * i),
			Color.WHITE
		)
		draw_line(
			Vector2(-size.x / 2 + offset.x + line_dist.x * i, -size.y / 2),
			Vector2(-size.x / 2 + offset.x + line_dist.x * i, size.y / 2),
			Color.WHITE
		)


func _input(event: InputEvent) -> void:
	if event is InputEventMouseMotion and Input.is_action_pressed("timeline_drag"):
		offset += event.relative


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	if Input.is_action_pressed("timeline_drag"):
		queue_redraw()
