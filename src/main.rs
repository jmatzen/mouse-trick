extern crate num;

#[macro_use]
extern crate num_derive;

use windows:: {
    core::*,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::UI::Input::KeyboardAndMouse::*,
};

use std::{thread,time};


#[derive(FromPrimitive,Debug)]
enum Action {
    MOVE_FORWARD = 0,
    MOVE_BACKWARD = 1
}

#[derive(ToPrimitive,Debug)]
enum ScanCode {
    W = 17,
    S = 31
}

fn delay_s(s: u64) -> time::Duration {
    time::Duration::from_secs(s)
}

fn main() {
    loop {
        let action = num::FromPrimitive::from_u32(rand::random::<u32>() % 2);
        println!("{:?}", action);
        match action  {
            Some(Action::MOVE_BACKWARD) => move_backward(),
            Some(Action::MOVE_FORWARD) => move_forward(),
            None => {}
        }
    }
}

fn do_scan_code(sc: ScanCode) {
    let keydown = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 { ki: KEYBDINPUT {
           wScan: num::ToPrimitive::to_u16(&sc).unwrap(),
           dwFlags: KEYEVENTF_SCANCODE,
           ..Default::default()
        }}
    };

    let mut keyup = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 { ki: KEYBDINPUT {
            wScan: num::ToPrimitive::to_u16(&sc).unwrap(),
            dwFlags: KEYEVENTF_KEYUP | KEYEVENTF_SCANCODE,
            ..Default::default()
        }}
    };
    let delay = rand::random::<u64>() % 4 + 2;
    unsafe {
        SendInput( &[keydown], std::mem::size_of::<INPUT>() as i32);
        wait(delay);
        SendInput( &[keyup], std::mem::size_of::<INPUT>() as i32);
    }

}

fn move_backward() {
    do_scan_code(ScanCode::S);
}

fn move_forward() {
    do_scan_code(ScanCode::W);
}

fn wait(delay: u64) {
    thread::sleep(delay_s(delay));
}