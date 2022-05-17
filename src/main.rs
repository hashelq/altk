use device_query::{DeviceQuery, DeviceState, Keycode};
use libxdo::XDo;

fn main() {
    let device_state = DeviceState::new();
    let mut buffer = 0;
    let mut lastkeys = device_state.get_keys();
    let xdo = XDo::new(None).unwrap();

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        for key in &keys {
            if lastkeys.contains(&key) {
                continue;
            }
            println!("Summary: {}", buffer);
            println!("Key pressed: {:?}", key);
            use Keycode::*;
            match key {
                // Flush
                NumpadAdd => {
                    let character = match char::from_u32(buffer) {
                        Some(v) => v,
                        None => {eprintln!("Invalid code: {}", buffer); buffer = 0; continue}
                    };
                    println!("Writing {}", character);
                    buffer = 0;
                    match xdo.enter_text(&character.to_string(), 100) {
                        Ok(v) => v,
                        Err(e) => { eprintln!("Cannot enter: {:?}", e); continue }
                    };
                },
                
                // Numpad number
                // TODO: Macro
                Numpad0 => buffer = buffer * 10,
                Numpad1 => buffer = buffer * 10 + 1,
                Numpad2 => buffer = buffer * 10 + 2,
                Numpad3 => buffer = buffer * 10 + 3,
                Numpad4 => buffer = buffer * 10 + 4,
                Numpad5 => buffer = buffer * 10 + 5,
                Numpad6 => buffer = buffer * 10 + 6,
                Numpad7 => buffer = buffer * 10 + 7,
                Numpad8 => buffer = buffer * 10 + 8,
                Numpad9 => buffer = buffer * 10 + 9,
                
                // Any other key like QGMLWY, ignore
                _ => {}
            }
        }
        lastkeys = keys;
    }
}
