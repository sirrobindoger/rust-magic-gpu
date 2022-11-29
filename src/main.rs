use rmagic::run;

use pollster;

fn main() {
    pollster::block_on(run());
}
