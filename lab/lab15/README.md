# Lab15: The game of GO

For this lab I used rust

# Things I accomplished in this lab
* representing the game state
* represent the stones on the board
* understand and count the groups
* Count amount of white and black groups
* updating the board (game moves)
* detect some illegal game moves
* detect captures of single stones and bigger groups
* socket based communication between two clients
* server/client mode
* local mode

The server/client mode is working, but there are a lot of stuff it doesn't do correctly yet.

# Run the program
This executes as one program, just use cargo run!
after this you will get a prompt to choose which mode you want to enter.
remember that if you want to test the client mode, it needs a server to connect to, and another client to play against.
The local gamemode lets you play against yourself :)

# Commands
```json
newgame <size> - start new game with size
move <x> <y> - place stone at x, y
groups - print number of groups
board - print current board
quit - exit the game
help - print this message"
```
