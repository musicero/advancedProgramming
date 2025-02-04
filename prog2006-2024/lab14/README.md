# Lab 14: Waterland

* Deadline: **no deadline, portfolio only**
* Difficulty: intermediate
* Language: Haskell or Rust
* ChatGPT: **decide for yourself**
* Co-pilot: **decide for yourself**
* Focus on concise nature, scalability and simplicity of the solution.


# Waterland

In a not-so-distant future, humanity's relentless pursuit of progress and industrialization has led to catastrophic consequences for the planet. Global warming has caused sea levels to rise dramatically, swallowing entire continents and leaving only scattered islands and vast expanses of open water. In this new world, known as **Waterland**, the remaining population has adapted to life on the water, forming floating cities and settlements atop massive artificial platforms.

Apart water cities, there were also *floating islands*.
The islands that dotted the vast oceans were not stable landmasses but rather shifting masses of debris, vegetation, and soil carried by the water currents.
Over time, these islands would drift and collide with one another, creating unpredictable patterns and rearranging the geography of *Waterland*.

The inhabitants of the floating cities soon realized the potential of these drifting islands. They could serve as valuable sources of resources, offering fertile soil for agriculture, precious metals and minerals, and refuge from the harsh realities of life on the open water. However, accessing and harnessing the resources of these islands proved to be a daunting task.

The challenge lay in navigating the treacherous waters between the floating cities and the drifting islands. The vast expanses of open water were teeming with dangers, from unpredictable storms and rough seas to marauding bands of pirates and rival factions vying for control. Furthermore, the shifting nature of the islands made it difficult to predict their movements, requiring skilled navigators and advanced technology to safely reach them.

As tensions between the floating cities reached a boiling point and resources grew increasingly scarce, the race to harness the resources of the drifting islands intensified.
Now, brave adventurers, skilled sailors, and resourceful engineers set out on perilous expeditions across the open waters hoping to secure a better future for themselves and their communities amidst the chaos and uncertainty of this watery world.


# The task

You have been given a map of floating islands at the start of your journey.
The map is provided in `data.txt` file. 

The map includes empty space (`.`) and islands (`#`). 
For example:

```
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
```

Your goal is to calculate the sum of the lengths of the shortest path between every pair of islands.

However, there's a catch: the water keep raising, and the Waterland expands, and the islands drift apart. In the time it will take you to reach the area, the map needs to be updated for the drift and expansion effects. 
The space between the islands expands in a specific, pecular way, such that any rows or columns that contain no islands should all actually be twice as big.

In the above example, three columns and two rows contain no islands:

```
   v  v  v
 ...#......
 .......#..
 #.........
>..........<
 ......#...
 .#........
 .........#
>..........<
 .......#..
 #...#.....
   ^  ^  ^
```

These rows and columns need to be twice as big; the result of water  expansion and island drift therefore looks like this:

```
....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
```

Equipped with this expanded map, the shortest path between every pair of islands can be found. It can help to assign every island a unique number:

```
....1........
.........2...
3............
.............
.............
........4....
.5...........
............6
.............
.............
.........7...
8....9.......
```

In these 9 islands, there are 36 unique pairs. 
Only count each pair once. Of course, the order within the pair doesn't matter. 
For each pair, find any shortest path between the two islands using only steps that move: `up`, `down`, `left`, or `right` exactly one `.` or `#` at a time. Note, that the shortest path between two islands is allowed, and may need to pass through another island.

For example, here is one of the shortest paths between islands 5 and 9:

```
....1........
.........2...
3............
.............
.............
........4....
.5...........
.##.........6
..##.........
...##........
....##...7...
8....9.......
```

This path has length 9 because it takes a minimum of nine steps to get from island 5 to island 9 (the eight locations marked # plus the step onto island 9 itself). 

Here are some other example shortest path lengths:
* Between island 1 and island 7: 15
* Between island 3 and island 6: 17
* Between island 8 and island 9: 5

In this example, after expanding the map, the sum of the shortest path between all 36 pairs of islands is 374.

Expand the map, then find the length of the shortest path between every pair of islands.

**What is the sum of these shortest paths?**



# Additional data file

If you solve the task before the end of semester, contact Mariusz with your solution in your workspace to obtain feedback and additional data file to test.


