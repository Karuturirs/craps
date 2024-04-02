mod lib;

fn main() {
    let mut game = lib::CrapsGame::new();
    game.play();
}
