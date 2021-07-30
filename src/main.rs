#![windows_subsystem = "windows"]

use hotkey::{keys, modifiers, Listener};
use mpd::Client;

static IP: &str = "192.168.0.100:6600";

fn pause() {
    let mut conn = Client::connect(IP).unwrap();
    conn.toggle_pause().unwrap();
}

fn prev() {
    let mut conn = Client::connect(IP).unwrap();
    conn.prev().unwrap();
}

fn next() {
    let mut conn = Client::connect(IP).unwrap();
    conn.next().unwrap();
}

fn vol_up() {
    let mut conn = Client::connect(IP).unwrap();
    let vol = conn.status().unwrap().volume;

    if vol + 5 <= 100 {
        conn.volume(vol + 5).unwrap();
    } else {
        conn.volume(100).unwrap();
    }
}

fn vol_down() {
    let mut conn = Client::connect(IP).unwrap();
    let vol = conn.status().unwrap().volume;

    if vol - 5 >= 0 {
        conn.volume(vol - 5).unwrap();
    } else {
        conn.volume(0).unwrap();
    }
}
fn main() {
    let mut hk = Listener::new();
    let alt_shift = hotkey::modifiers::ALT | hotkey::modifiers::SHIFT;
    hk.register_hotkey(alt_shift, 'Q' as u32, prev).unwrap();
    hk.register_hotkey(alt_shift, 'W' as u32, next).unwrap();
    hk.register_hotkey(alt_shift, '1' as u32, vol_down).unwrap();
    hk.register_hotkey(alt_shift, '2' as u32, vol_up).unwrap();
    hk.register_hotkey(modifiers::SHIFT, keys::CAPS_LOCK, pause)
        .unwrap();
    //115 is f4
    hk.register_hotkey(alt_shift | modifiers::CONTROL, 115_u32, || {
        std::process::exit(0x100)
    })
    .unwrap();
    hk.listen();
}
