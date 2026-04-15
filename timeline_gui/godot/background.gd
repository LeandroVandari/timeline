extends Node2D
@onready var viewport: Viewport = get_viewport()
@onready var timeline: Timeline = $".."
var offset: float = 0.
var num_lines: int = 0

func _draw() -> void:
	var YEAR_FONT = ThemeDB.fallback_font
	print_verbose("Redrawing background lines...")
	var size = viewport.get_visible_rect().size
	var line_dist = size.x / num_lines
	offset = fposmod(offset, line_dist)
	var marker_iter = LineMarkerIterator.create("1970-01-01T00:00:00+00:00[+00:00]", 0)

	for i in range(num_lines):
		var x_pos = -size.x / 2 + offset + line_dist * i;
		draw_line(
			Vector2(x_pos, -size.y / 2),
			Vector2(x_pos, size.y / 2),
			Color.DARK_GRAY
		)

		var year = marker_iter.next()
		var year_text_size = YEAR_FONT.get_string_size(year)
		draw_string(YEAR_FONT, Vector2(x_pos - year_text_size.x/2, MainLine.vertical_offset + 20.), year, HORIZONTAL_ALIGNMENT_CENTER)


func _ready() -> void:
	num_lines = Timeline.years_width()

	# initialize the offset so that the starting year isn't exactly on the screen edge
	var size = viewport.get_visible_rect().size
	var line_dist = size.x / num_lines
	offset = line_dist/2

func _input(event: InputEvent) -> void:
	if event is InputEventMouseMotion and Input.is_action_pressed("timeline_drag"):
		offset += event.relative.x



# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	if Input.is_action_pressed("timeline_drag"):
		queue_redraw()
