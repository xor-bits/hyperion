use super::tss::Tss;
use crate::trace;
use x86_64::{
    instructions::tables::load_tss,
    registers::segmentation::{Segment, CS, SS},
    structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
};

//

#[derive(Debug)]
pub struct Gdt {
    inner: GlobalDescriptorTable,
    selectors: SegmentSelectors,
}

#[derive(Debug)]
struct SegmentSelectors {
    kernel_code: SegmentSelector,
    kernel_data: SegmentSelector,
    // user_code: SegmentSelector,
    // user_data: SegmentSelector,
    // tss: SegmentSelector,
}

//

impl Gdt {
    pub fn new(tss: &'static Tss) -> Self {
        let mut inner = GlobalDescriptorTable::new();

        let selectors = SegmentSelectors {
            kernel_code: inner.add_entry(Descriptor::kernel_code_segment()),
            kernel_data: inner.add_entry(Descriptor::kernel_data_segment()),
            // user_code: inner.add_entry(Descriptor::user_code_segment()),
            // user_data: inner.add_entry(Descriptor::user_data_segment()),
            // tss: inner.add_entry(Descriptor::tss_segment(&tss.inner)),
        };

        Self { inner, selectors }
    }

    pub fn load(&'static self) {
        trace!("Loading GDT");
        self.inner.load();

        unsafe {
            CS::set_reg(self.selectors.kernel_code);
            SS::set_reg(self.selectors.kernel_data);
            // load_tss(self.selectors.tss);
        }

        trace!("incorrect gdt={self:?}");
    }
}
