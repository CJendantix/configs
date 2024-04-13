execute if entity @a[nbt={Inventory:[{id:"minecraft:iron_door"}]}] run say IRON DOOR FOUND
execute if score $timer timer matches 5 run scoreboard players reset $timer timer
scoreboard players add $timer timer 1