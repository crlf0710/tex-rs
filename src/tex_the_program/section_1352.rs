//! @ When `\.{\\write 12\{...\}}' appears, we scan the token list `\.{\{...\}}'
//! without expanding its macros; the macros will be expanded later when this
//! token list is rescanned.
//
// @<Implement \.{\\write}@>=
pub(crate) macro Implement_write($globals:expr) {{
    let k: integer;
    let _p: pointer;
    // begin k:=cur_cs; new_write_whatsit(write_node_size);@/
    k = $globals.cur_cs as _;
    new_write_whatsit($globals, write_node_size.into())?;
    // cur_cs:=k; p:=scan_toks(false,false); write_tokens(tail):=def_ref;
    $globals.cur_cs = k as _;
    _p = scan_toks($globals, false, false)?;
    write_tokens!($globals, tail!($globals)) = $globals.def_ref;
    // end
    use crate::pascal::integer;
    use crate::section_0115::pointer;
    use crate::section_0213::tail;
    use crate::section_0473::scan_toks;
    use crate::section_1341::write_node_size;
    use crate::section_1341::write_tokens;
    use crate::section_1350::new_write_whatsit;
}}
