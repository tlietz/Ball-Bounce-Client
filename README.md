# Ball Bounce 

![bounce out gif](media/ball_bounce_first.gif)

Play the game. Hosted by Github pages: https://tlietz.github.io/Ball-Bounce-Client/

Ball Bounce is a game created in Rust using the [macroquad](https://github.com/not-fl3/macroquad) game engine.
It is compiled to wasm so that it can be played in a browser. For now, it is has local multiplayer with 2 players and keyboard controls.

Move the white paddle using the `A` and `D` keys. 

Move the purple paddle using the left and right arrow keys.

## Code Architecture

It has an ECS-like (entity component system) architecture.

## Future Work

Make a `GameState` enum to replace the currently used `BallState`. This will allow adding states like `Pause`, and `Menu`. 

Then, create a `game_state_system` that handles changing the `GameState` enum.
For example, pressing the `p` button for pause, and pressing `q` to quit. 