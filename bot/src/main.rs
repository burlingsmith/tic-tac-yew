
// 0 - Calibrate
// 1 - Gather image info
// 2 - Process info into n nodes
// 3 - Move
// 4 - Get feedback
// 5 - Pass
// 6 - If game is over, score the bots and hit the reset button

type PixelPos = (u32, u32);

#[derive(Debug)]
struct PosKey {
    tiles: [PixelPos; 9],
    player_indicator: PixelPos,
    reset: PixelPos,
}

fn main() {
    println!("Hello, world!");
}
