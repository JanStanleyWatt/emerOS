use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

use super::allocator::Locked;

use common_lib::allocator::Allocator;

/// ヒープ領域を管理する構造体
pub struct Heap<A: Allocator + 'static> {
    start: usize,
    size: usize,
    allocator: &'static Locked<A>,
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

/// ヒープ領域の初期化を行う。このとき、アロケータの初期化も同時に行う
///
/// ## Safety
/// 呼び出し元は以下の点を保証しなければならない:
/// - この関数が全処理の中で一度だけ呼び出されていること
pub unsafe fn init<A: Allocator>(
    heap: Heap<A>,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    // ヒープに対応付けたいページ範囲を作成する
    let page_range = {
        let heap_start = VirtAddr::new(heap.start as u64);
        let heap_end = heap_start + heap.size - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    // ページをヒープにマッピングする
    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }

    // アロケータの初期化
    unsafe {
        heap.allocator.lock().init(heap.start, heap.size);
    }

    Ok(())
}
