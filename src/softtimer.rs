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
use core::sync::atomic::{AtomicUsize, Ordering};

// ************************************************************************************************
// TRAITS
// ************************************************************************************************

// ************************************************************************************************
// TYPES AND STRUCTURES
// ************************************************************************************************

/// SoftTimer states
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    NotRegistered,
    Stopped,
    Running,
}

/// Posible SoftTimerErr values from this module.
#[derive(Debug, PartialEq)]
pub enum SoftTimerErr {
    NotRegistered,
    LimitExceeded,
    NoSuchTimer,
    InvalidParameter,
}

type Counter = usize;

/// SoftTimer instance data
pub struct SoftTimer {
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

pub struct TimerRegistry<'a> {
    timer: [Option<&'a SoftTimer>; MAX_SOFT_COUNTER],
}

// ************************************************************************************************
// IMPLEMENTATIONS
// ************************************************************************************************

impl<'a> SoftTimer {
    /// Create a new SofTimer
    ///
    pub const fn new() -> Self {
        SoftTimer {
            state: State::NotRegistered,
            counter: AtomicUsize::new(0),
            threshold: 0,
            auto_restart: false,
        }
    }

    /// Starts a timer. If the given threshold timer is timed out, the timer
    /// will signal. If the timer is already running, it will be restarted
    /// with the new given threshold
    ///
    pub fn start(&mut self, threshold: Counter, auto_restart: bool) -> Result<(), SoftTimerErr> {
        match self.state {
            State::NotRegistered => Err(SoftTimerErr::NotRegistered),
            _ => {
                self.threshold = threshold;
                self.counter.store(threshold, Ordering::Relaxed);
                self.auto_restart = auto_restart;
                self.state = State::Running;

                Ok(())
            }
        }
    }

    /// Restarts a timer
    ///
    pub fn restart(&mut self) -> Result<(), SoftTimerErr> {
        if let State::NotRegistered = self.state {
            Err(SoftTimerErr::NotRegistered)
        } else {
            self.counter.store(self.threshold, Ordering::Relaxed);
            self.state = State::Running;

            Ok(())
        }
    }

    /// Stops a timer. Note, in stop state the timer will not signal.
    ///
    pub fn stop(&mut self) {
        self.state = State::Stopped;
    }

    /// Return the current tick count of the timer. Note, that the tick count
    /// starts at threshold number and will be decreased. Therefore a 10 means
    /// that the timer has 10 ticks left for timeout.
    ///
    pub fn get(&self) -> Result<Counter, SoftTimerErr> {
        let val = self.counter.load(Ordering::Relaxed);

        match self.state {
            State::NotRegistered => Err(SoftTimerErr::NotRegistered),
            State::Stopped => Ok(val),
            State::Running => {
                if (0 == val) && self.auto_restart {
                    self.counter.store(self.threshold, Ordering::Relaxed);
                }
                Ok(val)
            }
        }
    }

    pub fn get_state(&self) -> State {
        self.state
    }

    /// Update the timer
    ///
    /// This function is expected from a cyclic task or interrupt. It
    /// Decrements a running timer until the count drops down to zero.
    ///
    pub fn update(&self) {
        if State::Running == self.state {
            let counter = self.counter.load(Ordering::Relaxed);
            if 0 < counter {
                self.counter.fetch_sub(1, Ordering::Relaxed);
            }
        }
    }

    pub fn register(
        &'a mut self,
        registry: &mut TimerRegistry<'a>,
    ) -> Result<(&'a mut SoftTimer), SoftTimerErr> {
        match registry.timer.iter().position(|x| x.is_none()) {
            Some(id) => {
                self.state = State::Stopped;
                registry.timer[id] = Some(self);

                Ok((self))
            }
            None => Err(SoftTimerErr::LimitExceeded),
        }
    }

    pub fn un_register(&mut self, registry: &mut TimerRegistry) -> Result<(), SoftTimerErr> {
        for (index, item) in registry.timer.iter().enumerate() {
            if let Some(_entry) = item {
                self.state = State::NotRegistered;
                registry.timer[index] = None;

                return Ok(());
            }
        }

        Err(SoftTimerErr::NoSuchTimer)
    }
}

impl<'a> Signal for SoftTimer {
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

impl<'a> Default for TimerRegistry<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> TimerRegistry<'a> {
    const TIMER_INIT_NONE: Option<&'a SoftTimer> = None;

    pub fn new() -> Self {
        TimerRegistry {
            timer: [Self::TIMER_INIT_NONE; MAX_SOFT_COUNTER],
        }
    }
}

// ************************************************************************************************
// TESTS
// ************************************************************************************************

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn softimer_init() {
        let st = SoftTimer::new();

        assert_eq!(st.state, State::NotRegistered);
        assert!(!st.auto_restart);
        assert_eq!(st.counter.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn softimer_register() {
        let mut registry = TimerRegistry::new();
        let mut st = SoftTimer::new();

        {
            st = st.register(&mut registry).unwrap();
        }
        //assert_eq!(st.state, State::Stopped);

        st.un_register(&mut registry).unwrap()
    }
}
