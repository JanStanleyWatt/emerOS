use bootloader_api::info::{MemoryRegionKind, MemoryRegions};
use x86_64::registers::control::Cr3;
use x86_64::{
    structures::paging::{FrameAllocator, OffsetPageTable, PageTable, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

const SIZE_4KIB: usize = 4096;

/// 新しいOffsetPageTableを初期化する
///
/// ## Safety
/// 呼び出し元は次の点を保障すること:
/// 1. 全物理メモリが与えられた `physical_memory_offset`（だけずらした上）でマップされていること
/// 1. この関数が全処理の中で一度だけ呼び出されていること
pub unsafe fn init(physical_memory_offset: u64) -> OffsetPageTable<'static> {
    let phys_offset = VirtAddr::new(physical_memory_offset);
    let level_4_table = active_level_4_table(phys_offset);

    OffsetPageTable::new(level_4_table, phys_offset)
}

/// ブートローダのメモリマップから、使用可能なフレームを返す構造体
pub struct BootInfoFrameAllocator {
    memory_regions: &'static MemoryRegions,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// 渡されたメモリマップからFrameAllocatorを作る関数
    ///
    /// ## Safety
    /// 呼び出し元は参照先のメモリマップが有効であることを保証しなければならない。
    /// 特に、`Usable` なフレームは実際に未使用でなくてはならない
    #[inline(always)]
    pub const unsafe fn init(memory_regions: &'static MemoryRegions) -> Self {
        BootInfoFrameAllocator {
            memory_regions,
            next: 0,
        }
    }

    /// メモリマップによって指定された `Usable` なフレームのイテレータを返す
    /// 手順は以下の通り:
    /// 1. メモリマップから `Usable` な領域を得る
    /// 1. それぞれの `Usable` な領域をアドレス範囲にmapで変換する
    /// 1. フレームの開始アドレスのイテレータへと変換する
    /// 1. 開始アドレスから`Usable` なフレームたる `PhysFrame` 型を作る
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_regions.iter();
        let usable_regions = regions.filter(|r| r.kind == MemoryRegionKind::Usable);

        let addr_ranges = usable_regions.map(|r| r.start..r.end);

        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(SIZE_4KIB));

        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

unsafe fn active_level_4_table(phys_offset: VirtAddr) -> &'static mut PageTable {
    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = phys_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    // 	生ポインタを参照外しして返すのでこの関数はunsafe
    &mut *page_table_ptr
}

// /// 与えられた仮想アドレスを対応する物理アドレスに変換し、
// /// そのアドレスがマップされていないなら`None`を返す
// ///
// /// ## Safety
// /// 呼び出し元は全物理メモリが与えられた `physical_memory_offset`（だけずらした上）でマップされていることを保証しなくてはならない
// unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
//     translate_addr_inner(addr, physical_memory_offset)
// }

// fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
//     use x86_64::registers::control::Cr3;
//     use x86_64::structures::paging::page_table::FrameError;

//     // 有効なレベル4フレームをCR3レジスタから読む
//     let (level_4_table_frame, _) = Cr3::read();

//     let table_indexes = [
//         addr.p4_index(),
//         addr.p3_index(),
//         addr.p2_index(),
//         addr.p1_index(),
//     ];
//     let mut frame = level_4_table_frame;

//     // 複数層のページテーブルを辿る
//     for &index in &table_indexes {
//         // フレームをページテーブルの参照に変換する
//         let virt = physical_memory_offset + frame.start_address().as_u64();
//         let table_ptr: *const PageTable = virt.as_ptr();
//         let table = unsafe { &*table_ptr };

//         // ページテーブルエントリを読んで、`frame`を更新する
//         let entry = &table[index];
//         frame = match entry.frame() {
//             Ok(frame) => frame,
//             Err(FrameError::FrameNotPresent) => return None,
//             Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
//             //huge pageはサポートしていません
//         };
//     }

//     // ページオフセットを足すことで、目的の物理アドレスを計算する
//     Some(frame.start_address() + u64::from(addr.page_offset()))
// }
