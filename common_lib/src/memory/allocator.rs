pub mod fixed_size_block;

/// アロケータ共通の初期化操作を定義するトレイト
pub trait Allocator {
    /// アロケータを引数で与えられたヒープ境界で初期化する
    ///
    /// ## Safety
    /// 呼び出し元は以下の点を保証しなければならない:
    /// 1. 与えるヒープ境界が有効であり、なおかつヒープが未使用であること
    /// 1. この関数が全処理の中で一度だけ呼び出されていること
    unsafe fn init(&mut self, heap_start: usize, heap_size: usize);
}
