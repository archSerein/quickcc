pub mod astgen;
pub mod helper;
pub mod types;

use types::VarDec;
type SymbolInfo = (VarDec, usize, Option<Vec<VarDec>>, usize);
type SymbolKey = (String, usize);

pub const DEFAULT_OFFSET: usize = 0;
