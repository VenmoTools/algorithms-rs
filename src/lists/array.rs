use std::alloc::{alloc, dealloc, Layout, realloc, rust_oom};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut, Index};
use std::ptr;
// Project algorithms-rs
// Create by VenmoSnake 2020/7/13 17:53
//
use std::ptr::NonNull;

use crate::error;

const DEFAULT_CAP: usize = 5;

struct RawPtr<T> {
    ptr: NonNull<T>,
    _mark: PhantomData<T>,
}

impl<T> RawPtr<T> {
    pub fn new(p: *mut T) -> Self {
        Self {
            ptr: NonNull::new(p).unwrap(),
            _mark: PhantomData,
        }
    }
    #[allow(dead_code)]
    pub fn empty() -> Self {
        Self {
            ptr: NonNull::new(mem::align_of::<T>() as *mut T).unwrap(),
            _mark: PhantomData,
        }
    }
    /// return inner pointer
    pub fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }
    /// return inner pointer as mutable
    pub fn as_mut(&mut self) -> *mut T {
        self.ptr.as_ptr()
    }
    /// return relative offset of pointer
    pub unsafe fn offset(&self, count: usize) -> *mut T {
        self.ptr.as_ptr().offset(count as isize)
    }
    /// read pointer of offset pointer to value
    pub unsafe fn read(&self, offset: usize) -> T {
        ptr::read(self.offset(offset))
    }
    /// write pointer of offset pointer to value
    pub unsafe fn write(&self, offset: usize, ele: T) {
        ptr::write(self.offset(offset), ele)
    }
    /// copy src offset to dest offset
    pub unsafe fn copy(&self, src_offset: usize, dest_offset: usize, count: usize) {
        ptr::copy(
            self.offset(src_offset),
            self.offset(dest_offset),
            count,
        )
    }
}

unsafe impl<T: Send> Send for RawPtr<T> {}

unsafe impl<T: Sync> Sync for RawPtr<T> {}

pub struct Array<T> {
    buf: RawArray<T>,
    size: usize,
}

impl<T> Array<T> {
    /// create new Array with default capacity
    /// ```no_run
    /// use algorithms_rs::lists::array::Array;
    /// let mut arr = Array::new();
    /// arr.append(5);
    /// ```
    pub fn new() -> Self {
        Self {
            buf: RawArray::with_capacity(DEFAULT_CAP),
            size: 0,
        }
    }

    /// create new Array with given capacity
    /// ```no_run
    ///
    /// use algorithms_rs::lists::array::Array;
    /// let mut arr = Array::with_capacity(100);
    /// arr.append(5);
    /// ```
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            buf: RawArray::with_capacity(cap),
            size: 0,
        }
    }

    /// return true if size is 0
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// get array current size
    pub fn len(&self) -> usize {
        self.size
    }

    /// get array current capacity
    pub fn cap(&self) -> usize {
        self.buf.capacity
    }

    /// get element from array
    /// ```no_run
    /// use algorithms_rs::lists::array::Array;
    /// let mut arr = Array::new();
    /// arr.append(5);
    /// assert_eq!(arr.get(0),Some(&5));
    /// ```
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.size {
            return None;
        }
        Some(unsafe { &*self.buf.ptr.offset(index) })
    }

    /// get mutable element from array
    /// ```no_run
    /// use algorithms_rs::lists::array::Array;
    /// let mut arr = Array::new();
    /// arr.append(5);
    /// let ele = raw.get_mut(0).unwrap();
    /// *ele = 10;
    /// let ele = raw.get(0);
    /// assert_eq!(ele,Some(&10));
    /// ```
    pub fn get_mut(&self, index: usize) -> Option<&mut T> {
        if index >= self.size {
            return None;
        }
        Some(unsafe { &mut *self.buf.ptr.offset(index) })
    }

    /// insert element to array
    /// # Zero Size Object
    /// current version do not support zero size object, this is undefine behavior!!
    ///
    /// ```no_run
    /// use algorithms_rs::lists::array::Array;
    /// let mut arr = Array::new();
    /// arr.append(5);
    /// arr.insert(0,10).unwrap();
    /// assert_eq!(arr.get(0),&10);
    /// ```
    pub fn insert(&mut self, index: usize, ele: T) -> Result<(), error::Error> {
        assert_ne!(mem::size_of::<T>(), 0, "not support zero size object");
        if index == 0 && self.size == 0 {
            unsafe { self.buf.ptr.write(index, ele); }
            self.size += 1;
            return Ok(());
        }

        if index >= self.size {
            return Err(error::Error::IndexOfBounds);
        }
        unsafe {
            if index < self.size {
                self.buf.ptr.copy(index, index + 1, self.size - index);
            }
            self.buf.ptr.write(index, ele);
            self.size += 1;
        }
        Ok(())
    }

    /// remove element from array
    /// ```no_run
    /// use algorithms_rs::lists::array::Array;
    /// let mut arr = Array::new();
    /// arr.append(5);
    /// arr.remove(0).unwrap();
    /// assert_eq!(arr.get(0),None);
    /// ```
    pub fn remove(&mut self, index: usize) -> Result<T, error::Error> {
        if index >= self.size {
            return Err(error::Error::IndexOfBounds);
        }
        self.size -= 1;
        unsafe {
            let ele = self.buf.ptr.read(index);
            self.buf.ptr.copy(index + 1, index, self.size - index);
            Ok(ele)
        }
    }

    /// append new element to array
    /// # Zero Size Object
    /// current version do not support zero size object, this is undefine behavior!!
    /// ```no_run
    /// use algorithms_rs::lists::array::Array;
    /// let mut arr = Array::new();
    /// arr.append(5);
    /// assert_eq!(arr.get(0),Some(&5));
    /// ```
    pub fn append(&mut self, ele: T) {
        assert_ne!(mem::size_of::<T>(), 0, "not support zero size object");
        if self.buf.capacity == self.size { unsafe { self.buf.alloc() } }
        unsafe {
            self.buf.ptr.write(self.size, ele);
        }
        self.size += 1;
    }

    /// pop the last element from array
    /// ```no_run
    /// use algorithms_rs::lists::array::Array;
    /// let mut arr = Array::new();
    /// arr.append(5);
    /// assert_eq!(arr.get(0),Some(&5));
    /// arr.pop();
    /// assert_eq!(arr.get(0),None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        return if self.size == 0 {
            None
        } else {
            self.size -= 1;
            unsafe {
                Some(self.buf.ptr.read(self.size))
            }
        };
    }
}

impl<T> Array<T>
    where T: Eq + PartialEq
{
    /// return index of array with given element if not found return None
    /// ```no_run
    /// use algorithms_rs::lists::array::Array;
    /// let mut arr =  Array::new();
    /// arr.append(5);
    /// arr.append(10);
    ///
    /// assert_eq!(arr.find(&5),Some(0));
    /// ```
    pub fn find(&self, ele: &T) -> Option<usize> {
        for i in 0..self.size {
            if let Some(data) = self.get(i) {
                if ele == data {
                    return Some(i);
                }
            }
        }
        None
    }
    /// return true if contains given element otherwise return false
    /// ```no_run
    /// use algorithms_rs::lists::array::Array;
    /// let mut arr =  Array::new();
    /// arr.append(5);
    /// arr.append(10);
    ///
    /// assert!(arr.contains(&5));
    /// ```
    pub fn contains(&self, ele: &T) -> bool {
        for i in 0..self.size {
            if Some(ele) == self.get(i) {
                return true;
            }
        }
        false
    }
}

impl<T> Index<usize> for Array<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

struct RawArray<T> {
    ptr: RawPtr<T>,
    capacity: usize,
}

impl<T> RawArray<T> {
    #[allow(dead_code)]
    fn new() -> Self {
        assert_ne!(mem::size_of::<T>(), 0, "not ready");
        Self {
            ptr: RawPtr::empty(),
            capacity: 0,
        }
    }

    fn with_capacity(cap: usize) -> Self {
        let align = mem::align_of::<T>();
        let size = mem::size_of::<T>() * cap;
        let (ptr, layout) = unsafe {
            let layout = Layout::from_size_align_unchecked(size, align);
            let ptr = alloc(layout);
            (ptr, layout)
        };
        if ptr.is_null() {
            rust_oom(layout);
        }
        Self {
            capacity: cap,
            ptr: RawPtr::new(ptr as *mut T),
        }
    }

    unsafe fn alloc(&mut self) {
        let align = mem::align_of::<T>();
        let size = mem::size_of::<T>();
        assert_ne!(size, 0, "capacity overflow");
        let (new_cap, ptr, layout) = if self.capacity == 0 {
            let layout = Layout::from_size_align_unchecked(size, align);
            let ptr = alloc(layout);
            (1, ptr, layout)
        } else {
            let cap = self.capacity * 2;
            let old_size = self.capacity * size;
            assert!(old_size <= (isize::max_value() as usize) / 2);
            let n_size = old_size * 2;
            let layout = Layout::from_size_align_unchecked(old_size, align);
            let ptr = realloc(
                self.ptr.as_mut() as *mut u8,
                layout,
                n_size,
            );
            (cap, ptr, layout)
        };
        if ptr.is_null() {
            rust_oom(layout);
        }
        self.ptr = RawPtr::new(ptr as *mut T);
        self.capacity = new_cap;
    }
}

impl<T> Deref for Array<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe {
            std::slice::from_raw_parts(self.buf.ptr.as_ptr(), self.size)
        }
    }
}

impl<T> DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            std::slice::from_raw_parts_mut(self.buf.ptr.as_ptr(), self.size)
        }
    }
}

impl<T> Drop for RawArray<T> {
    fn drop(&mut self) {
        if self.capacity != 0 {
            let align = mem::align_of::<T>();
            let size = mem::size_of::<T>();
            let nums = size * self.capacity;
            unsafe {
                dealloc(
                    self.ptr.as_mut() as *mut u8,
                    Layout::from_size_align_unchecked(nums, align),
                )
            }
        }
    }
}

#[test]
fn test() -> Result<(), error::Error> {
    let mut raw = Array::new();
    raw.append(80);
    raw.append(100);

    assert_eq!(raw.get(0), Some(&80));
    assert_eq!(raw.get(1), Some(&100));

    assert_eq!(raw[0], 80);
    assert_eq!(raw[1], 100);

    raw.pop();
    assert_eq!(raw.get(1), None);
    raw.pop();
    assert_eq!(raw.get(0), None);
    assert_eq!(raw.pop(), None);

    raw.insert(0, 5).unwrap();
    assert_eq!(raw.get(0), Some(&5));
    raw.insert(0, 55).unwrap();
    assert_eq!(raw.get(0), Some(&55));

    assert_eq!(raw.find(&55), Some(0));
    assert_eq!(raw.find(&5), Some(1));

    raw.remove(1).unwrap();
    assert_eq!(raw.get(1), None);

    assert!(raw.contains(&55));

    for i in 0..100 {
        raw.append(i);
    }
    assert_eq!(raw.len(), 101);
    assert_eq!(raw.get(100), Some(&99));

    print_arr(&raw);

    let ele = raw.get_mut(0).unwrap();
    *ele = 10;
    let ele = raw.get(0);
    assert_eq!(ele, Some(&10));
    Ok(())
}

fn print_arr<T: Debug>(arr: &Array<T>) {
    for i in 0..arr.len() {
        if let Some(data) = arr.get(i) {
            println!("{:?}", data);
        }
    }
}