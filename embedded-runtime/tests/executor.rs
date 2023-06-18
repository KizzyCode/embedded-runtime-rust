use embedded_runtime::run;
use std::{
    future::Future,
    pin::Pin,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering::SeqCst},
        Arc, Mutex,
    },
    task::{Context, Poll, Waker},
    thread,
    time::{Duration, Instant},
};

/// Implements the runtime functions
pub mod runtime {
    use std::{
        sync::atomic::{AtomicBool, Ordering::SeqCst},
        thread,
        time::Duration,
    };

    /// A global, shared signal
    static SIGNAL: AtomicBool = AtomicBool::new(false);

    /// Blocks until an event occurs (may wake spuriously)
    #[no_mangle]
    #[allow(non_snake_case)]
    pub fn _runtime_waitforevent_TBFzxdKN() {
        SIGNAL.store(true, SeqCst);
    }

    /// Raises an event
    #[no_mangle]
    #[allow(non_snake_case)]
    pub fn _runtime_sendevent_3YSaPmB7() {
        // The spinloop interval
        const SPINLOOP_INTERVAL: Duration = Duration::from_millis(33);

        // Spinlock until the signal is changed to true
        while SIGNAL.compare_exchange(true, false, SeqCst, SeqCst).is_err() {
            thread::sleep(SPINLOOP_INTERVAL);
        }
    }
}

/// A simple timer future
struct TimerFuture {
    /// The registered waker
    waker: Arc<Mutex<Option<Waker>>>,
    /// Whether the timer is done or not
    done: Arc<AtomicBool>,
    /// The amount of times this future was polled
    polled: &'static AtomicUsize,
}
impl TimerFuture {
    /// The timer delay of 7 seconds
    const DELAY_7S: Duration = Duration::from_secs(7);
    /// The timer delay of 3 seconds
    const DELAY_3S: Duration = Duration::from_secs(3);

    /// Creates a timer future that is pending for `duration`
    pub fn delay(duration: Duration, polled: &'static AtomicUsize) -> Self {
        // Create instance
        let this = Self { waker: Default::default(), done: Default::default(), polled };
        let (waker, done) = (Arc::clone(&this.waker), Arc::clone(&this.done));

        // Start background task
        thread::spawn(move || {
            // Sleep and mark as done
            thread::sleep(duration);
            done.store(true, SeqCst);

            // Wake executor
            let waker = waker.lock().expect("failed to lock waker slot");
            if let Some(waker) = waker.as_ref() {
                waker.wake_by_ref();
            }
        });
        this
    }
}
impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        self.polled.fetch_add(1, SeqCst);
        match self.done.load(SeqCst) {
            true => Poll::Ready(()),
            false => Poll::Pending,
        }
    }
}

/// A countdown future that resolves to pending until the poll-countdown becomes zero
struct CountdownFuture {
    /// The current countdown value
    countdown: usize,
    /// The amount of times this future was polled
    polled: &'static AtomicUsize,
}
impl CountdownFuture {
    /// Creates a new countdown future
    pub const fn new(countdown: usize, polled: &'static AtomicUsize) -> Self {
        Self { countdown, polled }
    }
}
impl Future for CountdownFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.polled.fetch_add(1, SeqCst);

        // Decrement the value if we are still pending
        if self.countdown > 0 {
            *self = Self::new(self.countdown - 1, self.polled);
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }

        // Return ready
        Poll::Ready(())
    }
}

/// Tests a timer executor
fn test_timer() {
    // Create the observer flag and build the executor
    static POLLED: AtomicUsize = AtomicUsize::new(0);

    // Start the executor
    let start = Instant::now();
    run!(TimerFuture::delay(TimerFuture::DELAY_3S, &POLLED)).expect("failed to execute timer");

    // Assert that 7s have passed and that the future has been polled
    assert!(Instant::now() - start >= TimerFuture::DELAY_3S, "executor finished too early");
    assert!(POLLED.load(SeqCst) >= 2, "future has not been polled often enough");
}

/// Tests multiple timer executiors
fn test_mutliple_timers() {
    // Create the observer flag and build the executor
    static POLLED: AtomicUsize = AtomicUsize::new(0);

    // Start the executor
    let start = Instant::now();
    run! {
        TimerFuture::delay(TimerFuture::DELAY_3S, &POLLED),
        TimerFuture::delay(TimerFuture::DELAY_7S, &POLLED),
        async {
            TimerFuture::delay(TimerFuture::DELAY_3S, &POLLED).await;
            TimerFuture::delay(TimerFuture::DELAY_7S, &POLLED).await;
        }
    }
    .expect("failed to execute timer");

    // Assert that 7s have passed and that the future has been polled
    assert!(Instant::now() - start >= TimerFuture::DELAY_7S, "executor finished too early");
    assert!(POLLED.load(SeqCst) >= 8, "future has not been polled often enough");
}

/// Tests repeated polling of a future that wakes the executor immediately
fn test_multipoll_immediately() {
    // Create the observer flag and build the executor
    static POLLED: AtomicUsize = AtomicUsize::new(0);

    // Perform the countdown
    run!(CountdownFuture::new(7, &POLLED)).expect("failed to execute countdown");
    assert!(POLLED.load(SeqCst) >= 8, "future has not been polled often enough");
}

/// Executes all tests serially
///
/// # Important
/// Serial execution is necessary since our cheap runtime cannot wake multiple simultaneous executors
#[test]
fn all() {
    test_timer();
    test_mutliple_timers();
    test_multipoll_immediately();
}
