pub mod error;
pub mod hooks;
pub mod macros;

use std::future::IntoFuture;

use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use tokio::{runtime::Runtime, task::JoinHandle};

lazy_static! {
    pub static ref SDK: Sdk = Sdk::setup();
}

pub trait Global: Send + Sync + Sized {
    fn cell() -> &'static OnceCell<Self>;
    fn create() -> Self;
    fn get_or_create() -> &'static Self {
        Self::cell().get_or_init(|| Self::create())
    }
}

pub struct Sdk {
    scheduler: Runtime
}

impl Sdk {
    pub fn setup() -> Self {
        log::info!("Setting up SDK.");

        unsafe { log::info!("LoginState: {:x}", FnDoLoggedOutCycle.address()); }

        let scheduler = Runtime::new().unwrap();

        Self {
            scheduler
        }
    }

    pub fn spawn<T>(&self, future: T) -> JoinHandle<<T as IntoFuture>::Output>
    where
    T: IntoFuture,
    T::IntoFuture: Send + Sync + 'static,
    T::Output: Send + Sync + 'static,
    {
        self.scheduler.spawn(future.into_future())
    }


    pub fn do_cycle(&self) {
        self.scheduler.block_on(async move {
            loop {
                tokio::task::yield_now().await;
            }
        });
    }
}