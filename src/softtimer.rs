// ************************************************************************************************
// DESCRIPTION
// ************************************************************************************************

//! # softtimer.rs
//!
//! Module Description
//! Implement a Software count down timer
//!
//! A Softimer is a counter with the following requirements
//!
//! * Each timer as a counter that counts down until zero.
//!   Zero means it expired and goes into a signaled state.
//! * The countdown happens by the update() function which
//! * A counter is registerd to a cyclic updater which triggers
//!   the update() function (hardware timer or other thread)
//! * A timer can be started with a start value and then counts
//!   down on update()
//! * A timer can be restarted to use the last start value
//! * A timer can be stopped to ignore updates.
//! * A timer has an auto_reset feature to restart if zero
//! * and get() or get_signal_state() is called
//!

// ************************************************************************************************
// USES
// ************************************************************************************************

use crate::{Signal, SignalState};
use core::borrow::Borrow;
use core::cell::{Ref, RefCell};
use core::sync::atomic::{AtomicUsize, Ordering};

// ************************************************************************************************
// TRAITS
// ************************************************************************************************

// ************************************************************************************************
// TYPES AND STRUCTURES
// ************************************************************************************************

/// SoftTimerData states
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    Disabled,
    Stopped,
    Running,
}

/// Posible SoftTimerErr values from this module.
#[derive(Debug, PartialEq)]
pub enum SoftTimerErr {
    Disabled,
    LimitExceeded,
    NoSuchTimer,
    InvalidParameter,
}

type Counter = usize;
type SoftTimerHandle = usize;

/// SoftTimerData instance
#[derive(Debug)]

pub struct SoftTimerData {
    state: State,
    counter: AtomicUsize,
    threshold: Counter,
    auto_restart: bool,
}

// ************************************************************************************************
// CONSTANTS
// ************************************************************************************************

const MAX_SOFT_COUNTER: usize = 16usize;

// ************************************************************************************************
// LOCAL VARIABLES
// ************************************************************************************************

pub struct SofTimers {
    timer: RefCell<[Option<RefCell<SoftTimerData>>; MAX_SOFT_COUNTER]>,
}

// ************************************************************************************************
// IMPLEMENTATIONS
// ************************************************************************************************

impl Signal for SoftTimerData {
    /// Checks if the timer timed out or not. If the timer timed out, the signal
    /// will be true, otherwise false.
    ///
    fn get_signal_state(&self) -> SignalState {
        let counter = self.counter.load(Ordering::Relaxed);

        if (State::Running == self.state) && (0 == counter) {
            if self.auto_restart {
                self.counter.store(self.threshold, Ordering::Relaxed);
            }
            SignalState::Signaled
        } else {
            SignalState::NotSignaled
        }
    }
}

impl SofTimers {
    const TIMER_INIT_NONE: Option<RefCell<SoftTimerData>> = None;

    pub fn new() -> Self {
        SofTimers {
            timer: RefCell::new([Self::TIMER_INIT_NONE; MAX_SOFT_COUNTER]),
        }
    }

    /// Create a new SofTimer
    ///
    pub fn create(&self) -> Result<SoftTimerHandle, SoftTimerErr> {
        let mut timers = self.timer.borrow_mut();
        match timers.iter().position(|x| x.is_none()) {
            Some(id) => {
                timers[id] = Some(RefCell::new(SoftTimerData {
                    state: State::Disabled,
                    counter: AtomicUsize::new(0),
                    threshold: 0,
                    auto_restart: false,
                }));

                Ok(id)
            }
            None => Err(SoftTimerErr::LimitExceeded),
        }
    }

    pub fn delete(&self, handle: SoftTimerHandle) -> Result<(), SoftTimerErr> {
        if handle < MAX_SOFT_COUNTER {
            let mut timers = self.timer.borrow_mut();
            if let Some(_t) = timers[handle].borrow() {
                timers[handle] = None;

                return Ok(());
            }
        }

        Err(SoftTimerErr::NoSuchTimer)
    }

    /// Starts a timer. If the given threshold timer is timed out, the timer
    /// will signal. If the timer is already running, it will be restarted
    /// with the new given threshold
    ///
    pub fn start(
        &self,
        handle: SoftTimerHandle,
        threshold: Counter,
        auto_restart: bool,
    ) -> Result<(), SoftTimerErr> {
        if handle < MAX_SOFT_COUNTER {
            let timers: Ref<'_, [Option<RefCell<SoftTimerData>>; 16]> = self.timer.borrow();

            if let Some(t) = &timers[handle] {
                let mut data = t.borrow_mut();
                data.threshold = threshold;
                data.counter.store(threshold, Ordering::Relaxed);
                data.auto_restart = auto_restart;
                data.state = State::Running;

                return Ok(());
            }
        } else {
            return Err(SoftTimerErr::InvalidParameter);
        }

        Err(SoftTimerErr::NoSuchTimer)
    }

    /// Restarts a timer
    ///
    pub fn restart(&self, handle: SoftTimerHandle) -> Result<(), SoftTimerErr> {
        if handle < MAX_SOFT_COUNTER {
            let timers: Ref<'_, [Option<RefCell<SoftTimerData>>; 16]> = self.timer.borrow();

            if let Some(t) = &timers[handle] {
                let mut data = t.borrow_mut();
                data.counter.store(data.threshold, Ordering::Relaxed);
                data.state = State::Running;

                return Ok(());
            }
        } else {
            return Err(SoftTimerErr::InvalidParameter);
        }

        Err(SoftTimerErr::NoSuchTimer)
    }

    /// Stops a timer. Note, in stop state the timer will not signal.
    ///
    pub fn stop(&self, handle: SoftTimerHandle) -> Result<(), SoftTimerErr> {
        if handle < MAX_SOFT_COUNTER {
            let timers: Ref<'_, [Option<RefCell<SoftTimerData>>; 16]> = self.timer.borrow();

            if let Some(t) = &timers[handle] {
                let mut data = t.borrow_mut();
                data.state = State::Stopped;

                return Ok(());
            }
        } else {
            return Err(SoftTimerErr::InvalidParameter);
        }

        Err(SoftTimerErr::NoSuchTimer)
    }

    /// Disables a timer.
    ///
    pub fn disable(&self, handle: SoftTimerHandle) -> Result<(), SoftTimerErr> {
        if handle < MAX_SOFT_COUNTER {
            let timers: Ref<'_, [Option<RefCell<SoftTimerData>>; 16]> = self.timer.borrow();

            if let Some(t) = &timers[handle] {
                let mut data = t.borrow_mut();
                data.state = State::Disabled;

                return Ok(());
            }
        } else {
            return Err(SoftTimerErr::InvalidParameter);
        }

        Err(SoftTimerErr::NoSuchTimer)
    }

    /// Update all running timer
    ///
    pub fn update(&self) {
        for (_idx, entry) in self.timer.borrow().iter().enumerate() {
            if let Some(t) = entry {
                let data = t.borrow_mut();

                if State::Running == data.state {
                    let counter = data.counter.load(Ordering::Relaxed);
                    if 0 < counter {
                        data.counter.fetch_sub(1, Ordering::Relaxed);
                    }
                }
            }
        }
    }

    /// Get timer data
    ///
    pub fn get(&self, handle: SoftTimerHandle) -> Result<SoftTimerData, SoftTimerErr> {
        if handle < MAX_SOFT_COUNTER {
            let timers: Ref<'_, [Option<RefCell<SoftTimerData>>; 16]> = self.timer.borrow();

            if let Some(t) = &timers[handle] {
                let data = t.borrow();

                return Ok(SoftTimerData {
                    state: data.state,
                    counter: AtomicUsize::new(data.counter.load(Ordering::Relaxed)),
                    auto_restart: data.auto_restart,
                    threshold: data.threshold,
                });
            } else {
                return Err(SoftTimerErr::InvalidParameter);
            }
        }
        Err(SoftTimerErr::NoSuchTimer)
    }
}

// ************************************************************************************************
// TESTS
// ************************************************************************************************

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn softimer_register() {
        let timers = SofTimers::new();

        let st = timers.create().unwrap();
        assert_eq!(timers.delete(st), Ok(()));
    }

    #[test]
    fn softimer_init() {
        let timers = SofTimers::new();
        let st_h = timers.create().unwrap();
        let data: SoftTimerData = timers.get(st_h).unwrap();

        assert_eq!(data.state, State::Disabled);
        assert!(!data.auto_restart);
        assert_eq!(data.counter.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn softimer_modify() {
        let timers = SofTimers::new();
        let h = timers.create().unwrap();
        let data: SoftTimerData = timers.get(h).unwrap();

        assert_eq!(data.state, State::Disabled);

        assert_eq!(timers.start(h, 1234, true), Ok(()));
        let data: SoftTimerData = timers.get(h).unwrap();

        assert_eq!(data.state, State::Running);
        assert_eq!(data.auto_restart, true);
        assert_eq!(data.counter.load(Ordering::Relaxed), 1234);

        assert_eq!(timers.stop(h), Ok(()));

        let data: SoftTimerData = timers.get(h).unwrap();
        assert_eq!(data.state, State::Stopped);
        assert_eq!(data.auto_restart, true);
        assert_eq!(data.counter.load(Ordering::Relaxed), 1234);

        assert_eq!(timers.delete(h), Ok(()));
    }
    #[test]
    fn softtimer_disable() {
        let timers = SofTimers::new();
        let h = timers.create().unwrap();
        assert_eq!(timers.disable(h), Ok(()));
        let data: SoftTimerData = timers.get(h).unwrap();
        assert_eq!(data.state, State::Disabled);
    }

    #[test]
    fn softtimer_update() {
        let timers = SofTimers::new();
        let h1 = timers.create().unwrap();
        let h2 = timers.create().unwrap();

        assert_eq!(timers.start(h1, 3, true), Ok(()));
        assert_eq!(timers.start(h2, 1, true), Ok(()));

        let data: SoftTimerData = timers.get(h1).unwrap();
        assert_eq!(data.counter.load(Ordering::Relaxed), 3);
        let data: SoftTimerData = timers.get(h2).unwrap();
        assert_eq!(data.counter.load(Ordering::Relaxed), 1);

        timers.update();
        let data: SoftTimerData = timers.get(h1).unwrap();
        assert_eq!(data.counter.load(Ordering::Relaxed), 2);
        let data: SoftTimerData = timers.get(h2).unwrap();
        assert_eq!(data.counter.load(Ordering::Relaxed), 0);

        timers.update();
        let data: SoftTimerData = timers.get(h1).unwrap();
        assert_eq!(data.counter.load(Ordering::Relaxed), 1);
        let data: SoftTimerData = timers.get(h2).unwrap();
        assert_eq!(data.counter.load(Ordering::Relaxed), 0);

        timers.update();
        let data: SoftTimerData = timers.get(h1).unwrap();
        assert_eq!(data.counter.load(Ordering::Relaxed), 0);
        let data: SoftTimerData = timers.get(h2).unwrap();
        assert_eq!(data.counter.load(Ordering::Relaxed), 0);

        assert_eq!(timers.restart(h1), Ok(()));
        assert_eq!(timers.restart(h2), Ok(()));

        let data: SoftTimerData = timers.get(h1).unwrap();
        assert_eq!(data.counter.load(Ordering::Relaxed), 3);
        let data: SoftTimerData = timers.get(h2).unwrap();
        assert_eq!(data.counter.load(Ordering::Relaxed), 1);
    }
}
