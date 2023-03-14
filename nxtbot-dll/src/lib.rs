#![feature(naked_functions)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(concat_idents)]
#![feature(inherent_associated_types)]
#![feature(tuple_trait)]

pub mod sdk;
pub mod engine;

use std::{thread, panic, io::{stdin, Stdin, Read}};

use log::LevelFilter;
use sdk::SDK;
use simple_logger::SimpleLogger;
use winapi::{shared::minwindef::{HINSTANCE, DWORD, LPVOID, BOOL, TRUE}, um::{consoleapi::AllocConsole, wincon::FreeConsole, libloaderapi::FreeLibraryAndExitThread}};

#[no_mangle]
pub extern "system" fn DllMain(
    module: HINSTANCE,
    reason: DWORD,
    _: LPVOID
) -> BOOL {
    if reason == 1 { thread::spawn(move || {
        unsafe { on_load(); }
    }); } else if reason == 0 {
        unsafe { on_unload(module); }
    };
    TRUE
}

unsafe fn on_load() {
    let res = panic::catch_unwind(|| {
        AllocConsole();
        SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

        // Initialize the SDK and start the internal scheduler cycle.
        lazy_static::initialize(&SDK);
        engine::start();
        SDK.do_cycle();
    });

    if let Err(e) = res { log::error!("Error: {:?}", e) };
}

unsafe fn on_unload(module: HINSTANCE) {
    FreeConsole();
    FreeLibraryAndExitThread(module, 0);
}