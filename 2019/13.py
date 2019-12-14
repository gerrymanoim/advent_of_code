"""
The arcade cabinet runs Intcode software like the game the Elves sent (your
puzzle input). It has a primitive screen capable of drawing square tiles on a
grid. The software draws tiles to the screen with output instructions: every
three output instructions specify the x position (distance from the left), y
position (distance from the top), and tile id. The tile id is interpreted as
follows:

- 0 is an empty tile. No game object appears in this tile.
- 1 is a wall tile. Walls are indestructible barriers.
- 2 is a block tile. Blocks can be broken by the ball.
- 3 is a horizontal paddle tile. The paddle is indestructible.
- 4 is a ball tile. The ball moves diagonally and bounces off objects.

For example, a ßsequence of output values like 1,2,3,6,5,4 would draw a
horizontal paddle tile (1 tile from the left and 2 tiles from the top) and a
ball tile (6 tiles from the left and 5 tiles from the top).
"""
from intcode import Computer, DefaultList, read_intcode_program
from pathlib import Path

from typing import List


class ArcadeCabinet:

    number_to_tile = {0: " ", 1: "#", 2: "%", 3: "=", 4: "@"}

    def __init__(
        self, game: List[str], free_play: bool = True, debug_mode: bool = False
    ):
        self.game = game
        self.load_game(free_play, debug_mode)
        self.screen = DefaultList(lambda: DefaultList(lambda: 0))
        self.score = 0
        self.ball_loc = (0, 0)
        self.paddle_loc = (0, 0)

    def __call__(self):
        while not self.computer.is_halted:
            self.computer.inputs = [str(self.move_paddle())]
            self.step()

    def load_game(self, free_play: bool, debug_mode: bool):
        if free_play:
            self.game[0] = "2"
        self.computer = Computer(self.game, debug_mode=debug_mode)

    def step(self):
        x_pos = self.computer()
        y_pos = self.computer()
        tile = self.computer()
        if (x_pos, y_pos) == (-1, 0):
            self.score = tile
        else:
            self.screen[y_pos][x_pos] = tile

            if tile == 3:
                self.paddle_loc = (x_pos, y_pos)
            if tile == 4:
                self.ball_loc = (x_pos, y_pos)

    def move_paddle(self):
        """
        Do we want the paddle tilted towards the ball?
        """
        if self.ball_loc[0] < self.paddle_loc[0]:
            return -1
        elif self.ball_loc[0] > self.paddle_loc[0]:
            return 1
        else:
            return 0

    def __str__(self):
        return "\n".join(
            [
                " ".join([self.number_to_tile[tile] for tile in row])
                for row in self.screen
            ]
        )


program = read_intcode_program(Path("input13.txt"))

arcade_cabinet = ArcadeCabinet(program, free_play=False)
arcade_cabinet()

print(
    len([tile for row in arcade_cabinet.screen for tile in row if tile == 2])
)

"""
The game didn't run because you didn't put in any quarters. Unfortunately, you
did not bring any quarters. Memory address 0 represents the number of quarters
that have been inserted; set it to 2 to play for free.

The arcade cabinet has a joystick that can move left and right. The software
reads the position of the joystick with input instructions:

- If the joystick is in the neutral position, provide 0.
- If the joystick is tilted to the left, provide -1.
- If the joystick is tilted to the right, provide 1.

The arcade cabinet also has a segment display capable of showing a single
number that represents the player's current score. When three output
instructions specify X=-1, Y=0, the third output instruction is not a tile; the
value instead specifies the new score to show in the segment display. For
example, a sequence of output values like -1,0,12345 would show 12345 as the
player's current score.

Beat the game by breaking all the blocks. What is your score after the last
block is broken?
"""

# We play by keeping our joystick pointed towards the ball

arcade_cabinet = ArcadeCabinet(program, free_play=True, debug_mode=True)
arcade_cabinet()
print(arcade_cabinet.score)
