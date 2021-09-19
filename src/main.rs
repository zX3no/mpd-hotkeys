#![windows_subsystem = "windows"]

use hotkey::{keys, modifiers, Listener};
use mpd::Client;

static IP: &str = "192.168.0.100:6600";

fn pause() {
    if let Ok(mut conn) = Client::connect(IP) {
        conn.toggle_pause().ok();
    }
}

fn prev() {
    if let Ok(mut conn) = Client::connect(IP) {
        conn.prev().ok();
    }
}

fn next() {
    if let Ok(mut conn) = Client::connect(IP) {
        conn.next().unwrap_or(());
    }
}

fn vol_up() {
    if let Ok(mut conn) = Client::connect(IP) {
        if let Ok(status) = conn.status() {
            let vol = status.volume;
            if vol + 5 <= 100 {
                conn.volume(vol + 5).unwrap();
            } else {
                conn.volume(100).unwrap();
            }
        }
    }
}

fn vol_down() {
    if let Ok(mut conn) = Client::connect(IP) {
        if let Ok(status) = conn.status() {
            let vol = status.volume;
            if vol - 5 <= 100 {
                conn.volume(vol - 5).unwrap();
            } else {
                conn.volume(0).unwrap();
            }
        }
    }
}

fn main() {
    let mut hk = Listener::new();
    let alt_shift = hotkey::modifiers::ALT | hotkey::modifiers::SHIFT;
    hk.register_hotkey(alt_shift, 'Q' as u32, prev).unwrap();
    hk.register_hotkey(alt_shift, 'W' as u32, next).unwrap();
    hk.register_hotkey(alt_shift, '1' as u32, vol_down).unwrap();
    hk.register_hotkey(alt_shift, '2' as u32, vol_up).unwrap();
    hk.register_hotkey(modifiers::SHIFT, keys::ESCAPE, pause)
        .unwrap();
    //115 is f4
    hk.register_hotkey(alt_shift | modifiers::CONTROL, 115_u32, || {
        std::process::exit(0x100)
    })
    .unwrap();
    hk.listen();
}
