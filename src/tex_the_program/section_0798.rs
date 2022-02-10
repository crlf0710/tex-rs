//! ` `

// @<Update width entry for spanned columns@>=
pub(crate) macro Update_width_entry_for_spanned_columns($globals:expr, $n:expr, $w:expr) {{
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
    let info_q = info_inner!($globals, q);
    // if link(info(q))>n then
    if link!($globals, info_q) > $n {
        /// a new span node
        let s;
        // begin s:=get_node(span_node_size); info(s):=info(q); link(s):=n;
        s = get_node($globals, span_node_size as _)?;
        info_inner!($globals, s) = info_inner!($globals, q);
        link!($globals, s) = $n;
        // info(q):=s; width(s):=w;
        info_inner!($globals, q) = s;
        width!($globals, s) = $w;
        // end
    }
    // else if width(info(q))<w then width(info(q)):=w;
    else if width!($globals, info_q) < $w {
        width!($globals, info_q) = $w;
    }
    // end
    use crate::section_0016::incr;
    use crate::section_0095::confusion;
    use crate::section_0110::max_quarterword;
    use crate::section_0115::pointer;
    use crate::section_0118::info_inner;
    use crate::section_0118::link;
    use crate::section_0125::get_node;
    use crate::section_0135::width;
    use crate::section_0797::span_node_size;
}}
