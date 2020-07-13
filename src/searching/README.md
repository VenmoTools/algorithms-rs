# 查找算法

## 线性查找法

文件：[liner.rs](liner.rs)

## 二分查找法

文件：[binary](binary.rs)

# 概念
### 循环不变量

循环不变量是一组在循环体内、每次迭代均保持为真的性质，通常被用来证明程序的正确性

```rust
use std::convert::AsRef;

pub fn search<'a, T, E>(data_set: &'a T, ele: &'a E) -> Option<usize>
    where E: Eq + PartialEq,
          T: AsRef<[E]>
{
    let data_set = data_set.as_ref();
    for index in 0..data_set.len() {
        // --------------------------------------> 在此处 data_set[0..i-1]/data_set[0..i)中没有找到目标(每一轮循环开始时都满足这样的条件：这就是循环不变变量)
        if &data_set[index] == ele { // ---------> 确认data_set[i]是否为目标，循环体来维持循环不变量
            return Some(index); 
        }
        // --------------------------------------> 程序执行到这里表示data_set[i]不是目标 => 确定data_set[0..index]中没有找到目标
    } 
    None
}
```

## 复杂度分析

表示算法性能

通常看最差的情况，算法运行的上界

## 对数
todo!