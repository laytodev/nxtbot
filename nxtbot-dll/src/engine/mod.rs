pub mod globals;

use std::{thread, time::Duration, ops::DerefMut};
use lazy_static::__Deref;
use winapi::um::winuser::UnhookWindowsHookEx;

use crate::{engine::globals::LoginState, sdk::{SDK, overlay}};

pub fn start() {
    log::info!("Starting engine.");

    // Start nxtbot overlay.
    overlay::overlay::start();

    test();
}

fn test() {
    update_login_page();
}

fn update_login_page() {
    thread::spawn(move || {
        loop {
            unsafe {
            }
        }
    });
}