use crate::line::cellref::CellRef;
use alloc::sync::Arc;
#[cfg(feature = "use_serde")]
use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;
use wezterm_cell::Cell;

extern crate alloc;
use alloc::vec::Vec;

#[cfg_attr(feature = "use_serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct VecStorage {
    cells: Vec<Cell>,
}

impl PartialEq for VecStorage {
    fn eq(&self, other: &Self) -> bool {
        if self.cells.len() != other.cells.len() {
            return false;
        }
        let bytes_len = self.cells.len() * core::mem::size_of::<Cell>();
        let a = unsafe { core::slice::from_raw_parts(self.cells.as_ptr().cast::<u8>(), bytes_len) };
        let b =
            unsafe { core::slice::from_raw_parts(other.cells.as_ptr().cast::<u8>(), bytes_len) };
        crate::line::simd::lines_equal(a, b)
    }
}

impl VecStorage {
    pub(crate) const fn new(cells: Vec<Cell>) -> Self {
        Self { cells }
    }

    #[cfg_attr(not(feature = "use_image"), allow(unused_mut, unused_variables))]
    pub(crate) fn set_cell(&mut self, idx: usize, mut cell: Cell, clear_image_placement: bool) {
        #[cfg(feature = "use_image")]
        if !clear_image_placement && let Some(images) = self.cells[idx].attrs().images() {
            for image in images {
                if image.has_placement_id() {
                    cell.attrs_mut().attach_image(Box::new(image));
                }
            }
        }
        self.cells[idx] = cell;
    }

    pub(crate) fn scan_and_create_hyperlinks(
        &mut self,
        line: &str,
        matches: Vec<crate::hyperlink::RuleMatch>,
    ) -> bool {
        // The capture range is measured in bytes but we need to translate
        // that to the index of the column.  This is complicated a bit further
        // because double wide sequences have a blank column cell after them
        // in the cells array, but the string we match against excludes that
        // string.
        let mut cell_idx = 0;
        let mut has_implicit_hyperlinks = false;
        for (byte_idx, _grapheme) in line.grapheme_indices(true) {
            let cell = &mut self.cells[cell_idx];
            let mut matched = false;
            for m in &matches {
                if m.range.contains(&byte_idx) {
                    let attrs = cell.attrs_mut();
                    // Don't replace existing links
                    if attrs.hyperlink().is_none() {
                        attrs.set_hyperlink(Some(Arc::clone(&m.link)));
                        matched = true;
                    }
                }
            }
            cell_idx += cell.width();
            if matched {
                has_implicit_hyperlinks = true;
            }
        }

        has_implicit_hyperlinks
    }
}

impl core::ops::Deref for VecStorage {
    type Target = Vec<Cell>;

    fn deref(&self) -> &Vec<Cell> {
        &self.cells
    }
}

impl core::ops::DerefMut for VecStorage {
    fn deref_mut(&mut self) -> &mut Vec<Cell> {
        &mut self.cells
    }
}

/// Iterates over a slice of Cell, yielding only visible cells
pub struct VecStorageIter<'a> {
    pub cells: core::slice::Iter<'a, Cell>,
    pub idx: usize,
    pub skip_width: usize,
}

impl<'a> Iterator for VecStorageIter<'a> {
    type Item = CellRef<'a>;

    fn next(&mut self) -> Option<CellRef<'a>> {
        while self.skip_width > 0 {
            self.skip_width -= 1;
            let _ = self.cells.next()?;
            self.idx += 1;
        }
        let cell = self.cells.next()?;
        let cell_index = self.idx;
        self.idx += 1;
        self.skip_width = cell.width().saturating_sub(1);
        Some(CellRef::CellRef { cell_index, cell })
    }
}
