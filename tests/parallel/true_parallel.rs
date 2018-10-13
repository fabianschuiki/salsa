use crate::setup::{Input, Knobs, ParDatabase, ParDatabaseImpl, WithValue};
use salsa::{Database, ParallelDatabase};

/// Test where two threads are executing sum. We show that they can
/// both be executing sum in parallel by having thread1 wait for
/// thread2 to send a signal before it leaves (similarly, thread2
/// waits for thread1 to send a signal before it enters).
#[test]
fn true_parallel_different_keys() {
    let db = ParDatabaseImpl::default();

    db.query(Input).set('a', 100);
    db.query(Input).set('b', 010);
    db.query(Input).set('c', 001);

    // Thread 1 will signal stage 1 when it enters and wait for stage 2.
    let thread1 = std::thread::spawn({
        let db = db.fork();
        move || {
            let v = db.knobs().sum_signal_on_entry.with_value(1, || {
                db.knobs().sum_await_on_exit.with_value(2, || db.sum("a"))
            });
            v
        }
    });

    // Thread 2 will await stage 1 when it enters and signal stage 2
    // when it leaves.
    let thread2 = std::thread::spawn({
        let db = db.fork();
        move || {
            let v = db.knobs().sum_await_on_entry.with_value(1, || {
                db.knobs().sum_signal_on_exit.with_value(2, || db.sum("b"))
            });
            v
        }
    });

    assert_eq!(thread1.join().unwrap(), 100);
    assert_eq!(thread2.join().unwrap(), 010);
}
