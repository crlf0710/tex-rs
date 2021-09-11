//! ` `
// @<Copy the token list@>=
pub(crate) macro Copy_the_token_list($globals:expr, $the_toks:expr) {{
    /// used for copying a token list
    let (mut p, mut q, mut r): (pointer, pointer, pointer);

    // begin p:=temp_head; link(p):=null;
    p = temp_head;
    link!($globals, p) = null;
    // if cur_val_level=ident_val then store_new_token(cs_token_flag+cur_val)
    if $globals.cur_val_level == cur_val_level_kind::ident_val {
        store_new_token!(
            $globals,
            cur_tok_type::from_cs($globals.cur_val as _).get(),
            p,
            q
        );
    }
    // else if cur_val<>null then
    else if $globals.cur_val as integer != null as integer {
        // begin r:=link(cur_val); {do not copy the reference count}
        /// do not copy the reference count
        const _: () = ();
        r = link!($globals, $globals.cur_val as pointer);
        // while r<>null do
        while r != null {
            // begin fast_store_new_token(info(r)); r:=link(r);
            let info_r = info_tok!($globals, r);
            fast_store_new_token!($globals, info_r, p, q);
            r = link!($globals, r);
            // end;
        }
        // end;
    }
    // the_toks:=p;
    $the_toks = p;
    // end
    use crate::pascal::integer;
    use crate::section_0115::null;
    use crate::section_0115::pointer;
    use crate::section_0118::info_tok;
    use crate::section_0118::link;
    use crate::section_0162::temp_head;
    use crate::section_0297::cur_tok_type;
    use crate::section_0371::fast_store_new_token;
    use crate::section_0371::store_new_token;
    use crate::section_0410::cur_val_level_kind;
}}
