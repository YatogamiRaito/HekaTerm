mod cellref;
mod clusterline;
#[allow(clippy::module_inception)]
mod line;
mod linebits;
mod storage;
mod test;
mod vecstorage;

pub use cellref::CellRef;
pub use line::{DoubleClickRange, Line};
pub mod simd;
