scoreboard players add @s stone_timer 1
execute as @s if score @s stone_timer matches 5 run function testing:recipe_triggered/stone_step2
execute as @s unless score @s stone_timer matches 5 run function testing:recipe_triggered/stone