use std::collections::HashSet;
use sdl2::{
    event::Event,
    pixels::Color,
    keyboard::{Scancode, Keycode}
};
use bt_kb::init::Ctx;
use bt_kb::KbReport;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let Ctx {
        port,
        sdl_context,
        video_subsystem,
        mut event_pump,
        mut canvas
    } = Ctx::new()?;

    let mut reporter = KbReport::new(port);

    canvas.set_draw_color(Color::RGB(100, 100, 100));
    canvas.clear();
    canvas.present();


    let mut ctrl_pressed = false;
    let mut alt_pressed = false;

    let mut pressed_keys: HashSet<Scancode> = HashSet::new();

    'running: loop {
        for event in event_pump.wait_iter() {
            canvas.clear();
            match event {
                Event::Quit {..} => break 'running,
                Event::KeyDown {
                    scancode: Some(scancode),
                    keycode: Some(keycode),
                    ..
                } => {
                    match keycode {
                        Keycode::LCtrl | Keycode::RCtrl => ctrl_pressed = true,
                        Keycode::LAlt | Keycode::RAlt => alt_pressed = true,
                        Keycode::Q if ctrl_pressed && alt_pressed => {
                            //for key in pressed_keys.iter() {
                            //    send_key_release(&mut *port, *key)?;
                            //}
                            reporter.release_all()?;
                            break 'running
                        }
                        _ => ()
                    }
                    if let None = pressed_keys.get(&scancode) {
                        //send_key_press(&mut *port, scancode)?;
                        reporter.press(scancode)?;
                        pressed_keys.insert(scancode);
                    }
                    //let slice = &[kbgrab::newkeys::scancode_to_key(scancode)];
                    //let s = std::str::from_utf8(slice).unwrap_or("0");
                    //println!("key down: {}", s);

                }
                Event::KeyUp {
                    scancode: Some(scancode),
                    keycode: Some(keycode),
                    ..
                } => {
                    match keycode {
                        Keycode::LCtrl | Keycode::RCtrl => ctrl_pressed = false,
                        Keycode::LAlt | Keycode::RAlt => alt_pressed = false,
                        _ => ()
                    }
                    //port.write_all(&[0, kbgrab::newkeys::scancode_to_key(scancode)])?;
                    pressed_keys.remove(&scancode);
                    //send_key_release(&mut *port, scancode)?;
                    reporter.release(scancode)?;

                }
                _ => ()
            }
            canvas.present();
        }
    }

    Ok(())
}

//fn send_key_press(tx: &mpsc::Sender<[u8; 2]>, scancode: Scancode) -> Result<(), mpsc::SendError<[u8; 2]>> {
//    tx.send([1, kbgrab::newkeys::scancode_to_key(scancode)])
//}
//fn send_key_release(tx: &mpsc::Sender<[u8; 2]>, scancode: Scancode) -> Result<(), mpsc::SendError<[u8; 2]>> {
//    tx.send([0, kbgrab::newkeys::scancode_to_key(scancode)])
//}
