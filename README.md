# Quackhacks 2025 Submission
Our project sumbitted for the Quackhacks 2025 hackathon, built in roughly 26 hours.

![game example](/game.gif?raw=true "game example")

## Team Members
- Lynx: Lead Developer
- Enzo: Game Design and level editing

## Project Overview
The program runs a 3d doom-like engine with support for concave spaces. The program can input arbitrary text files of wall segments and build a playable map.
This was built from scratch using simple 2d polygon drawing and a custon 3d engine.

## Map Input
You can pass in a text file as a command line argument to load a custom map. Example map files are included in the repository. A map file consists of wall segments defined by their start and end coordinates in 2D space. Each line in the file represents a wall segment in the format x1 y1 x2 y2 where (x1, y1) are the coordinates of the start point and (x2, y2) are the coordinates of the end point of the wall segment. This has support for floating point coordinates.

### Camera
The First line in the map file defines the starting position of the camera in the format start_x start_y where (start_x, start_y) are the coordinates of the camera's starting position.

