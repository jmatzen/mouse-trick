use windows:: {
    // core::*,
    // Win32::Foundation::*,
    // Win32::UI::WindowsAndMessaging::*,
    Win32::UI::Input::KeyboardAndMouse::*,
};

use std::{thread,time};

fn delay_s(s: u64) -> time::Duration {
    time::Duration::from_secs(s)
}

fn main() {
    loop {
        let rnd = rand::random::<u32>() % 8;
        match rnd {
            0 => do_scan_code('W'),
            1 => do_scan_code('W'),
            2 => do_scan_code('W'),
            3 => do_scan_code('W'),
            4 => do_scan_code(' '),
            5 => do_scan_code('S'),
            6 => do_scan_code('A'),
            7 => do_scan_code('D'),
            _ => (),
        }
    }
}

fn do_scan_code(ch: char) {
    let scancode;
    unsafe {
        scancode = MapVirtualKeyA(ch as u32, MAPVK_VK_TO_VSC);
    }
    
    let delay = rand::random::<u64>() % 4 + 2;
    
    if ch == 'W' || ch == ' ' {
        send_scan_code(SCANCODE_LSHIFT, Default::default());
        send_scan_code(scancode, Default::default());
        wait(delay);
        send_scan_code(scancode, KEYEVENTF_KEYUP);
        send_scan_code(SCANCODE_LSHIFT, KEYEVENTF_KEYUP);
    } else {
        send_scan_code(scancode, Default::default());
        wait(delay);
        send_scan_code(scancode, KEYEVENTF_KEYUP);
    }

}

fn send_scan_code(sc: u32, flags: KEYBD_EVENT_FLAGS) {
    
    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 { ki: KEYBDINPUT {
            wScan:  sc as u16,
            dwFlags: flags | KEYEVENTF_SCANCODE,
            ..Default::default()
        }}
    };

    unsafe {
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

fn wait(delay: u64) {
    thread::sleep(delay_s(delay));
}