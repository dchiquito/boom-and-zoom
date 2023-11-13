extends Node2D

func _ready():
	for x in range(0, 8):
		for y in range(0, 8):
			var tile = ColorRect.new()
			tile.size = Vector2(100,100)
			tile.position = Vector2(x*100, y*100)
			if (x+y) % 2 == 0:
				tile.color = Color(0.9, 0.9, 0.9)
			else:
				tile.color = Color(0.2, 0.2, 0.2)
			add_child(tile)
