# Sudoku Cruncher
This is a 2x2/3x3 sudoku solver using a backtracking algorithm I wrote in rust for fun and to get a better handle on the language.

# Theory

"Backtracking" is basically a fancy word for brute forcing. The puzzle is initially set up and the program looks for the first square to fill. It fills this square with a 1. It then validates the state of the board. If the board is in an invalid state, it erases the last cell and increments that cell to 2. It repeats this until the cell is in a valid state, at which point it performs this on the next digit.

If the digit goes all the way to the highest possible value (9 in a 3x3 grid) and none of the values are valid, it erases that square and previous squares until it finds a digit it can increment and carries on from that position. If the puzzle unwinds all the way back to the original square and that square can't be incremented, it declares the puzzle as unsolvable.

# Implementation

To implement this project, I designed two pieces: a factory and a board object. The factory keeps track of the setup process and the moves which construct the game board. I wanted to design the application so that each time the solution is iterated, a new game board is generated. 

The benefit of this is that we could easily reuse the board object for other purposes. It contains only the game logic for sudoku; it contains no long-term state storage or any of the generator logic besides a constructor. If you wanted to make this the engine of an actual game, that wouldn't be very hard to accomplish; just rip out the the factory and write a new one hooked into a UI. The downside is that regenerating the board every iteration instead of mutating existing data structures takes more processing time. Never the less, it provides some interesting benefits.

Another "interesting" choice I made was doing all of this with a 1D array just because I could. I can't say if it was the right decision as it caused a few headaches, but I made it work.

# Usage
`sudoku_cruncher <board size> <filename.sku>`

- The board size can be 2 or 3 to represent 2x2 and 3x3 puzzles. I haven't programmed in hexadecimal support or oddly shaped boards (though it probably could be tweaked to do so.)
- The filename argument contains the board. Examples are provided in the repository, but it's pretty lenient.
