//! もしアロケータの実装が増えたら、ここにそのモジュールを追加していく

pub mod fixed_size_block;

/// 外部トレイトを実装するためのハックとしてspin::Mutexをラップする型
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}
