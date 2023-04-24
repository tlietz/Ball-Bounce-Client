# Ball Bounce 

Try to keep the ball bouncing.

Move the white paddle using the `A` and `D` keys. 
Move the purple paddle using the `<-` and `->` keys.

You can play the game here: https://tlietz.github.io/Ball-Bounce-Client/index.html

![bounce out gif](media/ball_bounce_first.gif)


Ball Bounce is a game created in Rust using the [macroquad](https://github.com/not-fl3/macroquad) game engine.
It is compiled to wasm so that it can be played in a browser. For now, it has only local multiplayer with 2 players and keyboard controls.


## Code Architecture

The game is created with an ECS (entity component system) architecture.

The `Player` and `Ball` are entities, which are represented by `struct`s.

`Velocity`, `Position`, and `Control` are components, which are also represented by `struct`s. Entities can have any number of components depending on what characteristics they need. For example, the `Ball` entity does not have a `Control` component, but the `Player` entities do.

`System`s are functions that control everything.
All of the game logic complexity is contained within the `system`s. This makes it easy to know where in the code features should be implemented. 

This also explains why ECS was chosen instead of OOP. With OOP, it would be more difficult to determine where features should be implemented because games have many features that require interaction between mutliple objects. 

For example, let's see how we would implement a `bounce()` function that bounces a ball off our paddle. 

In OOP, we would have to decide whether to add `bounce()` to the `Ball` or `Player` object. *Does a ball bounce off the paddle, or does the paddle bounce the ball?* This may be easy to manage with a simple game, but as it gets more complex, OOP would become exponentially harder to manage.

With ECS, a `system` takes in the `game_state` and determines when to call `bounce()`. 
The `system` is external to both the `Ball` and the `Player`, so we no longer have to worry about where to put methods like we would with OOP.

## Future Work

- Make a `GameState` enum to replace the currently used `BallState`. This will allow adding states like `Pause`, and `Menu`. 
- Create a `game_state_system` that handles changing the `GameState` enum.
For example, pressing the `p` button for pause, and pressing `q` to quit. 
