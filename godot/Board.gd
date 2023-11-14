extends Node2D

@onready var gameboard = $GodotGameBoard
@onready var pieces = $Pieces
var game_piece_scene = preload("res://GamePiece.tscn")
const NOT_SELECTED: int = -1
var selected_piece: int = NOT_SELECTED

# Called when the node enters the scene tree for the first time.
func _ready():
	for i in range(0, 8):
		var piece = game_piece_scene.instantiate()
		pieces.add_child(piece)
	update_pieces()

func update_pieces():
	var children = pieces.get_children()
	for i in range(0, 8):
		var piece = children[i]
		piece.is_white = gameboard.is_piece_white(i)
		piece.height = gameboard.get_piece_height(i)
		piece.board_coords = gameboard.get_piece_position(i)
		piece.render()

func _input(event):
	if event is InputEventMouseButton and event.pressed:
		var board_coords = Vector2(event.position.x / 100, 8 - (event.position.y / 100))
		print(board_coords)
		if selected_piece == NOT_SELECTED:
			selected_piece = gameboard.get_piece_at(board_coords)
		else:
			gameboard.move_or_boom(selected_piece, board_coords)
			selected_piece = NOT_SELECTED
			update_pieces()
