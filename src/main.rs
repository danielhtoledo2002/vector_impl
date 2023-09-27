use std::{
    alloc::{alloc, dealloc, handle_alloc_error, realloc, Layout},
    fmt::Display,
    // ops::{Deref, DerefMut},
    ptr::NonNull,
};

#[derive(Debug)]
struct Vector<T> {
    ptr: NonNull<T>, //* mut T
    size: usize,
    curr: usize,
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        let del_layout = Layout::array::<T>(self.size).unwrap();
        while let Some(_) = self.pop() {}
        unsafe { dealloc(self.ptr.as_ptr() as *mut u8, del_layout) };
    }
}

// impl<T> Iterator for Vector<T>
// where
//     T: Iterator,
//     T::Item: IntoIterator,
// {
//     type Item = <T::Item as IntoIterator>::Item;
//     fn next(&mut self) -> Option<Self::Item> {}
// }

impl<T> Vector<T> {
    fn new(size: usize) -> Vector<T> {
        if size == 0 {
            Vector {
                ptr: NonNull::<T>::dangling(),
                size: 0,
                curr: 0,
            }
        } else {
            let x = Layout::array::<T>(size).unwrap();
            Vector {
                ptr: match NonNull::new(unsafe {
                    alloc(Layout::array::<T>(size).unwrap()) as *mut T
                }) {
                    Some(pointer) => pointer,
                    None => handle_alloc_error(x),
                },
                size: 0,
                curr: 0,
            }
        }
    }
    fn iter(&self) -> Vector<T> {
        Vector {
            ptr: self.ptr,
            size: 0,
            curr: 0,
        }
    }

    fn rezise(&mut self) {
        let new_layout = Layout::array::<T>(self.size + 1).unwrap();

        self.ptr = NonNull::new(unsafe {
            realloc(
                self.ptr.as_ptr() as *mut u8,
                Layout::array::<T>(self.size).unwrap(),
                new_layout.size(),
            ) as *mut T
        })
        .ok_or_else(|| handle_alloc_error(new_layout))
        .unwrap();
        self.size += 1;
    }
    fn is_empty(&mut self) -> bool {
        self.curr == 0
    }

    fn is_full(&mut self) -> bool {
        self.curr == self.size
    }

    fn push(&mut self, value: T)
    where
        T: Display,
    {
        if self.is_full() {
            self.rezise();
        }
        unsafe {
            self.ptr
                .as_ptr()
                .offset(self.curr.try_into().unwrap())
                .write(value);
            // println!(
            //     "{}",
            //     &self
            //         .ptr
            //         .as_ptr()
            //         .offset(self.curr.try_into().unwrap())
            //         .as_ref()
            //         .unwrap()
            // );
        }
        self.curr += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.curr -= 1;
        Some(unsafe {
            self.ptr
                .as_ptr()
                .offset(self.curr.try_into().unwrap())
                .read()
        })
    }
}
impl<'a, T> Iterator for Vector<T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.size {
            let item_ptr = unsafe { self.ptr.as_ptr().add(self.curr + 1) };
            Some(unsafe { std::ptr::read(item_ptr })
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let mut x: Vector<String> = Vector::new(1);
        x.push(String::from("adjhdasj"));
        x.push(String::from("adjhdasj"));
        x.push(String::from("ajkfhdskjha"));
        println!("{}", x.size);
    }
}

fn main() {
    let mut x: Vector<String> = Vector::new(1);
    x.push(String::from("adjhdasj"));
    x.push(String::from("adjhdasj"));
    x.push(String::from("ajkfhdskjha"));
    // println!("{}", x.size);
    for item in x.iter() {
        println!("{}", item);
    }
}
