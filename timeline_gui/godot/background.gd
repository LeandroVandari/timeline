extends Node2D
@onready var viewport: Viewport = get_viewport()
@onready var timeline: Timeline = $".."

@export var line_dist: float = 100.
var show_months: bool

var year_start: Year = Year.get_current()
var offset: float = 0.


func _draw() -> void:
	var YEAR_FONT: Font = ThemeDB.fallback_font
	var MONTH_FONT: Font = ThemeDB.fallback_font
	print_verbose("Redrawing background lines...")

	var size: Vector2 = viewport.get_visible_rect().size
	var num_lines: float = ceil(size.x / line_dist) + 2

	# right edge of screen
	if offset <= 0:
		year_start = year_start.get_next()
	# left edge
	elif offset >= line_dist:
		year_start = year_start.get_previous()
	offset = fposmod(offset, line_dist)

	var year_iter: YearIterator = YearIterator.create(year_start)
	for i: int in range(num_lines):
		var year_x_pos: float = (-line_dist * num_lines) / 2 + line_dist * i + offset
		draw_line(
			Vector2(year_x_pos, -size.y / 2), Vector2(year_x_pos, size.y / 2), Color.DARK_GRAY
		)

		var year: Year = year_iter.next_year()
		var year_text: String = year.label()
		var year_text_size: Vector2 = YEAR_FONT.get_string_size(year_text)
		draw_string(
			YEAR_FONT,
			Vector2(year_x_pos - year_text_size.x / 2, MainLine.vertical_offset + 20.),
			year_text,
			HORIZONTAL_ALIGNMENT_CENTER,
			-1
		)

		if show_months:
			var month_dist: float = line_dist / year.months_amount()
			# no enumerate function :(
			var j: int = 0
			for month: int in year.months():
				var month_text: String = str(month)
				var month_text_size: Vector2 = MONTH_FONT.get_string_size(month_text)
				draw_string(
					MONTH_FONT,
					Vector2(
						year_x_pos + month_dist * j - month_text_size.x / 2,
						MainLine.vertical_offset + 15.
					),
					month_text,
					HORIZONTAL_ALIGNMENT_CENTER,
					-1,
					12
				)
				j += 1


func _ready() -> void:
	# initialize the offset so that the starting year isn't exactly on the screen edge
	offset = line_dist / 2


func _input(event: InputEvent) -> void:
	if event is InputEventMouseMotion and Input.is_action_pressed("timeline_drag"):
		offset += (event as InputEventMouseMotion).relative.x


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	if Input.is_action_pressed("timeline_drag"):
		queue_redraw()
