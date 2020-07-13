// Project algorithms-rs
// Create by VenmoSnake 2020/7/13 11:41
//

/// return element index if find in `data_set` otherwise return `None`
///```no_run
/// let data_set = vec![21, 22, 31, 4, 55, 67];
/// assert_eq!(liner::search(&data_set, &4), Some(3));
/// assert_eq!(liner::search(&data_set, &666), None);
///
/// let data_set = [21, 22, 31, 4, 55, 67];
/// assert_eq!(liner::search(&data_set, &4), Some(3));
/// assert_eq!(liner::search(&data_set, &666), None);
///
/// let data_set = "abcdef";
/// assert_eq!(liner::search(&data_set, &b'c'), Some(2));
/// assert_eq!(liner::search(&data_set, &b'h'), None);
///
/// let data_set = "abcdef".to_string();
/// assert_eq!(liner::search(&data_set, &b'c'), Some(2));
/// assert_eq!(liner::search(&data_set, &b'h'), None);
/// ```
pub fn search<'a, T, E>(data_set: &'a T, ele: &'a E) -> Option<usize>
    where E: Eq + PartialEq,
          T: AsRef<[E]>
{
    let data_set = data_set.as_ref();
    for index in 0..data_set.len() {
        if &data_set[index] == ele {
            return Some(index);
        }
    }
    None
}