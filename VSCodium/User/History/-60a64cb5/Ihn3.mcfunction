say hi
scoreboard players add @s stone_timer 1
execute if score @s stone_timer matches 5 run say a
execute unless score @s stone_timer matches 5 run function testing:recipe_triggered/stone