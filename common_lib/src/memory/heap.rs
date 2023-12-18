use crate::locked::Locked;

use crate::memory::allocator::Allocator;

/// ヒープ領域を管理する構造体
pub struct Heap<A: Allocator + 'static> {
    pub start: usize,
    pub size: usize,
    pub allocator: &'static Locked<A>,
}

impl<A> Heap<A>
where
    A: Allocator + 'static,
{
    /// ヒープ領域の初期化の下準備として、それを定義する構造体を作成する関数
    ///
    /// ## Safety
    /// 呼び出し元は以下の点を保証しなければならない:
    /// - 与えるヒープ境界が有効であり、なおかつメモリとして未使用であること
    #[inline(always)]
    pub const unsafe fn new(start: usize, size: usize, allocator: &'static Locked<A>) -> Self {
        Heap {
            start,
            size,
            allocator,
        }
    }
}
