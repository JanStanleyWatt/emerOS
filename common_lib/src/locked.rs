/// 外部トレイトを実装するためのハックとしてspin::Mutexをラップする型
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    /// 初期化関数。ロックしたい型をこれに収める
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    /// ミューテックス式のロックを実行する
    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}
