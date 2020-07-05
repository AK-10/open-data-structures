// 発展: vectorの実装
// https://doc.rust-lang.org/nomicon/vec.html 

// use std::mem;
// use std::ops::{Index, IndexMut};

// pub struct Array<T> {
//     a: *mut T,
//     length: usize,
// }

// impl<T> Array<T> {
//     pub fn new(len: usize) -> Self {

//         Array {
//             a: ,
//             length: len,
//         }
//     }
// }

// impl<T: Sized> Index<usize> for Array<T> {
//     type Output = T;
//     fn index(&self, i: usize) -> &Self::Output {
//         let mem_size = mem::size_of::<T>();
        
//     }
// }