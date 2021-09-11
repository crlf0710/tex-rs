//! ` `

// @<Enter a hyphenation exception@>=
pub(crate) macro Enter_a_hyphenation_exception($globals:expr, $n:expr, $p:expr) {{
    crate::trace_span!("Enter a hyphenation exception");
    // begin incr(n); hc[n]:=cur_lang; str_room(n); h:=0;
    incr!($n);
    $globals.hc[$n.get() as usize] = $globals.cur_lang;
    str_room($globals, $n.get() as integer * character_max_room);

    /// an index into `hyph_word` and `hyph_list`
    let mut h: hyph_pointer;

    h = 0.into();
    // for j:=1 to n do
    for j in 1..=$n.get() {
        // begin h:=(h+h+hc[j]) mod hyph_size;
        h = hyph_pointer::new(
            ((h.get() as integer
                + h.get() as integer
                + $globals.hc[j as usize].numeric_value() as integer)
                % hyph_size) as _,
        );
        // append_char(hc[j]);
        append_char(make_globals_string_view!($globals), $globals.hc[j as usize]);
        // end;
    }

    /// strings being compared or stored
    let s: str_number;
    // s:=make_string;
    s = make_string(make_globals_string_view!($globals));
    // @<Insert the \(p)pair |(s,p)| into the exception table@>;
    crate::section_0940::Insert_the_pair_s_p_into_the_exception_table!($globals, h, s, $p);
    // end
    use crate::pascal::integer;
    use crate::section_0004::make_globals_string_view;
    use crate::section_0004::TeXGlobalsStringView;
    use crate::section_0012::hyph_size;
    use crate::section_0016::incr;
    use crate::section_0038::str_number;
    use crate::section_0042::append_char;
    use crate::section_0042::character_max_room;
    use crate::section_0042::str_room;
    use crate::section_0043::make_string;
    use crate::section_0925::hyph_pointer;
}}
