use once_cell::sync::OnceCell;

use super::Global;

macro_rules! expect {
    ($val:expr, $msg:expr) => {
        if cfg!(feature = "no-msgs") {
            $val.unwrap()
        } else {
            $val.expect($msg)
        }
    };
}

macro_rules! panic_msg {
    ($($t:tt)*) => {
        if cfg!(feature = "no-msgs") {
            unimplemented!()
        } else {
            panic!($($t)*)
        }
    };
}

mod app;
pub mod overlay;
pub use app::OpenGLApp;

mod input;
mod painter;
mod shader;
pub mod utils;

pub struct Overlay {}

impl Global for Overlay {
    fn cell() -> &'static once_cell::sync::OnceCell<Self> {
        static INSTANCE: OnceCell<Overlay> = OnceCell::new();
        &INSTANCE
    }

    fn create() -> Self {
        Self {
            
        }
    }
}