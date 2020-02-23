extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use draw::to_coord_u32;
use game::Game;

// 背景颜色
const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (20, 20);

    // 创建窗口
    let mut window: PistonWindow =
        WindowSettings::new("snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(width, height);

    // 事件循环监听按键
    while let Some(event) = window.next() {

        // 事件更新重绘窗口
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        // 重绘窗口
        window.draw_2d(&event, |c, g, _device| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        // 定时更新窗口信息
        event.update(|arg| {
            game.update(arg.dt);
        });

    }
}
