# Multiplayer Maze-War game
Project developed as a part of school curriculum project
### Project description

Our own version of the game [maze-wars](<https://en.wikipedia.org/wiki/Maze_(1973_video_game)>)

Maze-Wars is a simplistic multiplayer first person shooting game.

The game presents a simple user interface:
* interactive main menu and level editor page
* map with player location and direction
* 3D view of the maze and opponents

### Tools
- Rust
- ggez (GUI)
- UDP protocol for communication
- ray-casting for 3D view 


### How to run and use the project

Simply navigate to the project folder and run `cargo run`.
You will be presented with the **main menu**, where you have the options to:

1. **Join a game**
   - Enter your username and IP address(can be without a port) of the server
2. **Create a game**
   - Host and join a game on your local IP address
   - Enter your username and select a map
3. **Create a map**
   - Make a map which you can use for your own game
   - Enter a name for your map and use or hold the right click to draw walls

### Team
- Zane
- Vic
- Gatis
- Kristofer

### Acknowledgments
* Graphics of opponents and general guidelines for GUI design from [here](https://github.com/Blueteak/MazeWar)

