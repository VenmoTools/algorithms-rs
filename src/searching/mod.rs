// Project algorithms-rs
// Create by VenmoSnake 2020/7/13 11:41
//

use crate::utils::time_it;

mod liner;
mod binary;


#[test]
fn test_liner_search() {
    let data_set = vec![21, 22, 31, 4, 55, 67];
    assert_eq!(liner::search(&data_set, &4), Some(3));
    assert_eq!(liner::search(&data_set, &666), None);

    let data_set = [21, 22, 31, 4, 55, 67];
    assert_eq!(liner::search(&data_set, &4), Some(3));
    assert_eq!(liner::search(&data_set, &666), None);

    let data_set = "abcdef";
    assert_eq!(liner::search(&data_set, &b'c'), Some(2));
    assert_eq!(liner::search(&data_set, &b'h'), None);

    let data_set = "abcdef".to_string();
    assert_eq!(liner::search(&data_set, &b'c'), Some(2));
    assert_eq!(liner::search(&data_set, &b'h'), None);
}

#[test]
fn bench_liner_search() {
    use random::Source;

    let mut gen = random::default().seed([0, 100]);
    let data_set: Vec<i32> = gen.iter().take(30).collect();
    time_it(|| {
        liner::search(&data_set, &4);
    }, 1000);
}
