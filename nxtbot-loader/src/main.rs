#![allow(dead_code, unused)]

use std::{fs::create_dir_all, process::Command};
use dll_injector::inject_dll_load_library;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();
    log::info!("Starting NXT Bot loader by Kyle Escobar.");

    // Check required directories exist.
    check_dirs();

    // Unpack assets from loader EXE.
    //unpack();

    // Launch the OSRS client process.
    let pid = launch_osrs();

    // Inject NXT bot DLL into OSRS client process.
    inject_dll(pid);

    // Finished 
    log::info!("Loader has completed. Exiting process.");
}

fn check_dirs() {
    log::info!("Checking required directory structure.");

    let dirs = vec![
        "bin/",
        "configs/",
        "scripts/",
        "plugins/",
        "logs/",
        "cache/"
    ];
    dirs.into_iter().for_each(|dir| {
        let home_dir = dirs::home_dir();

        let mut path = home_dir.unwrap().join(".nxtbot/");
        path = path.join(dir);
        if !path.exists() {
            log::info!("Creating missing directory: {:?}.", path.to_str().unwrap());
            create_dir_all(path).unwrap();
        }
    });

    log::info!("Verified all directories.");
}

fn unpack() {
    log::info!("Unpacking assets from loader.");
}

fn launch_osrs() -> u32 {
    log::info!("Launching Old School RuneScape client process.");
    let proc = Command::new("nxtbot-loader/bin/osclient.exe").spawn().unwrap();
    return proc.id();
}

fn inject_dll(pid: u32) {
    log::info!("Injecting NXT bot into process ID: {:?}", pid);
    inject_dll_load_library(pid, "target/debug/nxtbot.dll").unwrap();
    log::info!("NXT bot loader successfully injected into process.");
}

