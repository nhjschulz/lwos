#![no_std]
pub mod scheduler;
pub mod softtimer;
pub mod task;

pub use scheduler::*;
pub use softtimer::*;
pub use task::*;

pub enum SignalState {
    NotSignaled,
    Signaled,
}
trait Signal {
    fn get_signal_state(&self) -> SignalState;
}
