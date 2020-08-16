// Project algorithms-rs
// Create by VenmoSnake 2020/7/13 11:41
//
use std::time::SystemTime;

mod graph;

/// calculate function execute time
/// ```no_run
/// use random::Source;
/// use algorithms_rs::utils::time_it;
///
/// let mut gen = random::default().seed([0, 100]);
/// let data_set: Vec<i32> = gen.iter().take(30).collect();
/// time_it(|| {
///     liner::search(&data_set, &4); // data_set is moved
/// }, 1000);
/// ```
pub fn time_it<T>(func: T, times: u32)
    where T: FnOnce() + Copy
{
    let start = SystemTime::now();
    for _i in 0..times {
        func();
    }
    let end = SystemTime::now();
    println!("function execute time: `{:?}`", end.duration_since(start).unwrap())
}
