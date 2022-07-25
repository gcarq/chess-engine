# chess-engine

This is a *WIP* chess engine written in Rust and [bevy](https://bevyengine.org/) 0.7.
In the current state the engine can handle basic piece movements and rendering.

The assets in use are copied from [lichess](https://github.com/lichess-org/lila/tree/master/public/images), authors and license can be found [here](https://github.com/lichess-org/lila/blob/master/COPYING.md#exceptions-free).

## Run

Use `cargo run` to launch the app, when built in debug mode a debug inspector will be shown as well.

## Board interaction

Click *left* mouse button to select a piece and click again on a target square where the piece should go.

## TODOs

* Implement piece capturing and missing rules (castling, en passant)
* Add check handling and win conditions
* Add system to import positions with FEN and PGN and replay them
* Implement proper UI
