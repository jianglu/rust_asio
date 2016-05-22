use std::io;
use time::{Duration, Tm, SteadyTime, now};
use {IoObject, IoService, Strand};
use backbone::{Expiry, ToExpiry, TimerActor};
use timer::WaitTimer;
use ops::*;
use ops::async::*;

pub struct SystemTimer {
    io: IoService,
    actor: TimerActor,
}

impl IoObject for SystemTimer {
    fn io_service(&self) -> IoService {
        self.io.clone()
    }
}

impl AsTimerActor for SystemTimer {
    fn as_timer_actor(&self) -> &TimerActor {
        &self.actor
    }
}

impl WaitTimer for SystemTimer {
    type TimePoint = Tm;
    type Duration = Duration;

    fn new(io: &IoService) -> Self {
        SystemTimer {
            io: io.clone(),
            actor: TimerActor::new(),
        }
    }

    fn wait_at(&self, time: &Self::TimePoint) -> io::Result<()> {
        sleep_for((*time - now()).to_std())
    }

    fn async_wait_at<A, F, T>(a: A, time: &Self::TimePoint, callback: F, obj: &Strand<T>)
        where A: Fn(&T) -> &Self + Send + 'static,
              F: FnOnce(&Strand<T>, io::Result<()>) + Send + 'static,
              T: 'static {
        async_timer(a, time.to_expiry(), callback, obj)
    }

    fn wait_for(&self, time: &Self::Duration) -> io::Result<()> {
        sleep_for(time.to_std())
    }

    fn async_wait_for<A, F, T>(a: A, time: &Self::Duration, callback: F, obj: &Strand<T>)
        where A: Fn(&T) -> &Self + Send + 'static,
              F: FnOnce(&Strand<T>, io::Result<()>) + Send + 'static,
              T: 'static {
        async_timer(a, (now() + *time).to_expiry(), callback, obj)
    }

    fn cancel<A, T>(a: A, obj: &Strand<T>)
        where A: Fn(&T) -> &Self + Send + 'static {
        cancel_timer(a, obj);
    }
}

impl Drop for SystemTimer {
    fn drop(&mut self) {
        let _ = self.actor.unset_timer(&self.io);
    }
}