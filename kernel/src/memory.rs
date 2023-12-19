use bootloader_api::info::{MemoryRegions, Optional};
use core::cell::OnceCell;

use common_lib::{
    locked::Locked,
    memory::{allocator::fixed_size_block::FixedSizeBlockAllocator, heap::Heap},
};

const HEAP_START: usize = 0x_4444_4444_0000;
const HEAP_SIZE: usize = 32000 * 1024; // 32 MiB

// 固定サイズブロックアロケータを使う
#[global_allocator]
static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

/// メモリ管理機能の初期化
#[cfg(target_arch = "x86_64")]
pub(crate) fn init(physical_memory_offset: Optional<u64>, memory_regions: &'static MemoryRegions) {
    use amd64_lib::memory::{
        self,
        heap::{self},
        paging::BootInfoFrameAllocator,
    };

    // まずは物理メモリのオフセットを取り出す
    let physical_memory_offset = match physical_memory_offset {
        Optional::Some(addr) => addr,
        Optional::None => panic!("Failed to get physical memory offset"),
    };

    // ヒープとアロケータの初期化
    // 手順は以下の通り:
    // 1. ヒープ領域とそのアロケートに使うアロケータの登録
    // 1. OffsetPageTableの初期化
    // 1. 引数から渡されたメモリマップからFrameAllocatorを作る
    // 1. 最後にヒープ領域を初期化する（アロケータも、この時初期化する）
    let heap_init = OnceCell::new();
    heap_init.get_or_init(|| unsafe {
        let heap = Heap::new(HEAP_START, HEAP_SIZE, &ALLOCATOR);
        let mapper = &mut memory::paging::init(physical_memory_offset);
        let frame_allocator = &mut BootInfoFrameAllocator::init(memory_regions);

        heap::init(heap, mapper, frame_allocator).expect("heap initialization failed");
    });
}
