extends Node2D


# Called when the node enters the scene tree for the first time.
func _ready():
	var player = find_child("Player")
	print(player)
	var player2 = Player.new()
	add_child(player2)

func _input(event):
	if event is InputEventMouseButton and event.pressed:
		print("clicky", event.position)

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
