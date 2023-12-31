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

struct VectorIter<'a, T> {
    ptr_vec: &'a Vector<T>,
    current: isize,
}
struct VectorMutIter<'a, T> {
    ptr_vec: &'a mut Vector<T>,
    current: isize,
}

struct VecIntoIter<T> {
    vec: Vector<T>,
}

impl<T> Iterator for VecIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.vec.pop()
    }
}

impl<T> IntoIterator for Vector<T> {
    type Item = T;
    type IntoIter = VecIntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        VecIntoIter { vec: self }
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        let del_layout = Layout::array::<T>(self.size).unwrap();
        while let Some(_) = self.pop() {}
        unsafe { dealloc(self.ptr.as_ptr() as *mut u8, del_layout) };
    }
}

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
        self.size == 0
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
        }
        self.curr += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.size -= 1;
        Some(unsafe {
            self.ptr
                .as_ptr()
                .offset(self.size.try_into().unwrap())
                .read()
        })
    }
}

impl<T> Vector<T> {
    fn iter(&self) -> VectorIter<T> {
        VectorIter {
            ptr_vec: self,
            current: 0,
        }
    }

    fn iter_mut(&mut self) -> VectorMutIter<T> {
        VectorMutIter {
            ptr_vec: self,
            current: 0,
        }
    }
}

impl<'a, T> Iterator for VectorIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;

        ((self.current as usize) <= self.ptr_vec.size).then(|| unsafe {
            self.ptr_vec
                .ptr
                .as_ptr()
                .offset(self.current - 1)
                .as_ref()
                .unwrap()
        })
    }
}

impl<'a, T> Iterator for VectorMutIter<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;

        ((self.current as usize) <= self.ptr_vec.size).then(|| unsafe {
            self.ptr_vec
                .ptr
                .as_ptr()
                .offset(self.current - 1)
                .as_mut()
                .unwrap()
        })
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
    x.pop();
    // println!("{}", x.size);
    for item in x.iter() {
        println!("{}", item);
    }
}
