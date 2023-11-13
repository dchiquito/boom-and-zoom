extends Node2D

@onready var gameboard = $GodotGameBoard
@onready var pieces = $Pieces
var game_piece_scene = preload("res://GamePiece.tscn")

# Called when the node enters the scene tree for the first time.
func _ready():
	gameboard.logit()
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
		gameboard.advance()
		update_pieces()
