extends Node2D

@onready var label = $Label

var is_white: bool
var height: int
var board_coords: Vector2

func render():
	if height == 0:
		visible = false
	else:
		visible = true
	label.text = str(height)
	if is_white:
		label.modulate = Color(1,1,1)
	else:
		label.modulate = Color(0,0,0)
	label.position = Vector2(board_coords.x * 100, (7-board_coords.y) * 100)
