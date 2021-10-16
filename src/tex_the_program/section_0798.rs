//! ` `

// @<Update width entry for spanned columns@>=
pub(crate) macro Update_width_entry_for_spanned_columns($globals:expr, $n:expr) {{
    /// temporary pointers for list manipulation
    let mut q: pointer;
    // begin q:=cur_span;
    q = $globals.cur_span;
    // repeat incr(n); q:=link(link(q));
    loop {
        incr!($n);
        q = link!($globals, link!($globals, q));
        // until q=cur_align;
        if q == $globals.cur_align {
            break;
        }
    }
    // if n>max_quarterword then confusion("256 spans"); {this can happen, but won't}
    if $n > max_quarterword as _ {
        /// this can happen, but won't
        const _: () = ();
        confusion($globals, crate::strpool_str!("256 spans"))?;
    }
    // @^system dependencies@>
    // @:this can't happen 256 spans}{\quad 256 spans@>
    // q:=cur_span; while link(info(q))<n do q:=info(q);
    q = $globals.cur_span;
    while link!($globals, info_inner!($globals, q)) < $n {
        q = info_inner!($globals, q);
    }
    // if link(info(q))>n then
    //   begin s:=get_node(span_node_size); info(s):=info(q); link(s):=n;
    //   info(q):=s; width(s):=w;
    //   end
    // else if width(info(q))<w then width(info(q)):=w;
    // end
    todo!("update");
    use crate::section_0016::incr;
    use crate::section_0095::confusion;
    use crate::section_0110::max_quarterword;
    use crate::section_0115::pointer;
    use crate::section_0118::info_inner;
    use crate::section_0118::link;
}}
