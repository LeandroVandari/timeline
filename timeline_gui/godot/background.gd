extends Node2D
@onready var viewport: Viewport = get_viewport()
@onready var timeline: Timeline = $".."
const EXTRA_MARKER_ROOM: float = 1.1

var offset: float = 0.
var num_lines: int = 0


func _draw() -> void:
	var YEAR_FONT: Font = ThemeDB.fallback_font
	print_verbose("Redrawing background lines...")
	var size: Vector2 = viewport.get_visible_rect().size
	var line_dist: float = (size.x * EXTRA_MARKER_ROOM) / num_lines
	offset = fposmod(offset, line_dist)
	var marker_iter: LineMarkerIterator = LineMarkerIterator.create_from_now( 0)

	for i: int in range(num_lines):
		var x_pos: float = (-size.x * EXTRA_MARKER_ROOM) / 2 + offset + line_dist * i
		draw_line(Vector2(x_pos, -size.y / 2), Vector2(x_pos, size.y / 2), Color.DARK_GRAY)

		var year: LineMarker = marker_iter.next_marker()

		var year_text_size: Vector2 = YEAR_FONT.get_string_size(year.marker_str)
		draw_string(
			YEAR_FONT,
			Vector2(x_pos - year_text_size.x / 2, MainLine.vertical_offset + 20.),
			year.marker_str,
			HORIZONTAL_ALIGNMENT_CENTER
		)


func _ready() -> void:
	num_lines = Timeline.years_width()

	# initialize the offset so that the starting year isn't exactly on the screen edge
	var size: Vector2 = viewport.get_visible_rect().size
	var line_dist: float = size.x / num_lines
	offset = line_dist / 2


func _input(event: InputEvent) -> void:
	if  event is InputEventMouseMotion and Input.is_action_pressed("timeline_drag"):

		offset += (event as InputEventMouseMotion).relative.x


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	if Input.is_action_pressed("timeline_drag"):
		queue_redraw()
