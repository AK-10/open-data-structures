// pub mod array;
// use array::Array;

trait List<T> {
    fn size(&self) -> usize;
    fn get(self, i: usize) -> Result<Option<T>, &'static str>;
    fn set(&mut self, i: usize, elem: T) -> Result<Option<T>, &'static str>;
    fn add(&mut self, i: usize, elem: T);
    fn remove(&mut self, i: usize) -> Result<Option<T>, &'static str>;
}

// OMG: vec使わないとどうしようもないことに気づいた
// TODO: vecの実装
// a: 配列(capacityのみ持つ)
// 値が存在しない(size: 10, capacity: 100)のような場合を作るために，Option型にしている(Noneで埋める)
// n: size(要素数)
struct ArrayStack<T: Copy> {
    a: Box<[Option<T>]>,
    n: usize,
}

impl<T: Copy> ArrayStack<T> {
    pub fn new() -> ArrayStack<T> {
        ArrayStack{
            a: Box::new([]),
            n: 0
        }
    }

    fn resize(&mut self) {
        let cap = std::cmp::max(1, self.n*2);
        // 新しい配列を確保(vecという嘘)
        let mut b: Vec<Option<T>> = Vec::with_capacity(cap);

        // 計算量の合計Θ(m)
        for i in 0..cap {
            // into_boxed_sliceで余剰分は消されてしまうのでNoneで埋める
            b[i] = if i < self.n { self.a[i] } else { None }
        }

        self.a = b.into_boxed_slice();
    }
}

impl<T: Copy> List<T> for ArrayStack<T> {
    fn size(&self) -> usize {
        self.n
    }

    // 配列のためΘ(1)
    fn get(self, i: usize) -> Result<Option<T>, &'static str> {
        if i <= self.a.len() - 1 {
            Ok(self.a[i])
        } else {
            Err("err: index out of range")
        }
    }

    // 配列のためΘ(1)
    fn set(&mut self, i: usize, elem: T) -> Result<Option<T>, &'static str> {
        if i <= self.a.len() - 1 {
            let y = self.a[i];
            self.a[i] = Some(elem);

            Ok(y)
        } else {
            Err("err: index out of range")
        }
    }

    // 計算量はシフトする要素に比例 Θ(n-i) (resizeは無視したとして)
    // i番目以降を右シフトするためn-i
    fn add(&mut self, i: usize, elem: T) {
        if self.a.len() == self.n {
            self.resize();
        }
        // i番目以降をシフトさせる
        for idx in (i..=self.n - 1).rev() {
            self.a[idx+1] = self.a[idx]
        }
        self.a[i] = Some(elem);
        self.n += 1;
    }

    // 計算量はシフトする要素に比例 Θ(n-i) (resizeは無視したとして)
    // i番目以降を左シフトするためn-i
    fn remove(&mut self, i: usize) -> Result<Option<T>, &'static str> {
        let x = self.a[i];
        for idx in i..=self.n {
            self.a[idx] = self.a[idx+1];
        }
        self.n -= 1;

        if self.a.len() >= 3 {
            self.resize();
        }

        Ok(x)
    }
}

fn main() {
    println!("hi!");
}
