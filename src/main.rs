use piston_window::*;
use piston_window::types::Color;

mod drawing;

use drawing::to_gui_coord_u32;


const BG_COLOR: Color = [0.204, 0.286, 0.369, 1.0];

fn main() {
    let (width, height) = (20, 20);

    // Window Settings
    let mut win_settings = WindowSettings::new("Snake Rust", [to_gui_coord_u32(width), to_gui_coord_u32(height)]).exit_on_esc(true);

    win_settings.set_vsync(true);

    // Create window
    let mut window: PistonWindow = win_settings.build().unwrap();

    // Create Snake


    // Event Loop
    while let Some(event) = window.next() {

        // Keyboard events
        if let Some(Button::Keyboard(key)) = event.press_args() {

        }

        // Draw
        window.draw_2d(&event, |c, g, _| {
            clear(BG_COLOR, g);

        });

        // Update screen   
        event.update(|arg| {
            
        });
    }
}
