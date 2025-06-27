//implementing the rc type

use std::cell::Cell;
pub mod Rc {
struct RcInner<T> {
    value: T,
    count: Cell<usize>,
}

pub struct Rc<T> {
    val: *mut RcInner<T>,
}

impl <T> Rc <T> {
    fn new(val: T) -> Self {
        let box = Box::new(RcInner{
            value : T,
            count : Cell::new(1),
        });
        Rc {
            val : Box::into_raw(box),
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let a = unsafe {&*self.val};
        let n = a.count.get();
        a.count.set(a+1);
        Rc {
            val : self.val;
        }
    }
}

impl <T> std::ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &(&*self.val).value
        }
    }
}

impl <T> Drop for Rc<T> {
    fn drop(&mut self) {
        let a = unsafe { &*self.val };
        let b = a.count.get();
        if b == 1 {
            //we are the last reference and thus we have to deallocate the heap allocated box 
            drop(a);
            let _ = Box::from_raw(self.val);     
            /* box_raw does not take a raw const pointer as input
            instead it takes a raw mut pointer as input..we could still use val if it is a raw const
            pointer by using the concept og nonull provided by the ptr module of the standard library  */
        }
        else {
            //decrement the count by 1 that is..
            a.count.set(b-1);
        }
    }
}
}