

fn main() {
    if let Some(i) = std::env::args().nth(1) {
        if &i == "panic" {
            panic!("check my backtrace");
        } else if &i == "-v" {
            println!("Version output");
            return;
        }
    }

    let sdl = sdl2::init().unwrap();
    let _event_pump = sdl.event_pump().unwrap();
    let video = sdl.video().unwrap();

    let window = video.window("Vault 13", 640, 480)
        .position_centered()
        .allow_highdpi()
        .build()
        .unwrap();

    let mouse = sdl.mouse();
    mouse.set_relative_mouse_mode(true);

    let _canvas = window
        .into_canvas()
        .build()
        .unwrap();
}
