extends Node2D

@onready var gameboard = $GodotGameBoard
@onready var pieces = $Pieces
@onready var decorations = $Decorations
var game_piece_scene = preload("res://GamePiece.tscn")
var legal_move_indicator_scene = preload("res://LegalMoveIndicator.tscn")
var piece_highlight_scene = preload("res://PieceHighlight.tscn")
const NOT_SELECTED: int = -1
var selected_piece: int = NOT_SELECTED
var highlight_coords = null
var legal_moves = []

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

func update_decorations():
	for child in decorations.get_children():
		child.queue_free()
	if selected_piece != NOT_SELECTED:
		var highlight = piece_highlight_scene.instantiate()
		decorations.add_child(highlight)
		highlight.position = Vector2(100*highlight_coords.x, 100*(7-highlight_coords.y))
		for move in legal_moves:
			var indicator = legal_move_indicator_scene.instantiate()
			indicator.position = Vector2(100 * move.x, 100 * (7-move.y))
			decorations.add_child(indicator)

func _input(event):
	if event is InputEventMouseButton and event.pressed:
		var board_coords = Vector2(floor(event.position.x / 100), 8 - floor(event.position.y / 100))
		if selected_piece == NOT_SELECTED:
			selected_piece = gameboard.get_piece_at(board_coords)
			# TODO for now only white pieces are selectable
			if selected_piece != NOT_SELECTED and gameboard.is_piece_white(selected_piece):
				highlight_coords = board_coords
				legal_moves = gameboard.legal_moves(selected_piece)
			else:
				selected_piece = NOT_SELECTED
		else:
			gameboard.move_or_boom(selected_piece, board_coords)
			selected_piece = NOT_SELECTED
			update_pieces()
		update_decorations()
