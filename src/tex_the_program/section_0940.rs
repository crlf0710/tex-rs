//! ` `

// @<Insert the \(p)pair |(s,p)|...@>=
pub(crate) macro Insert_the_pair_s_p_into_the_exception_table($globals:expr, $h:expr, $s:expr, $p:expr) {{
    crate::trace_span!("Insert the pair `(s,p)`...");
    // if hyph_count=hyph_size then overflow("exception dictionary",hyph_size);
    if $globals.hyph_count.get() as integer == hyph_size {
        todo!("overflow");
    }
    // @:TeX capacity exceeded exception dictionary}{\quad exception dictionary@>
    // incr(hyph_count);
    incr!($globals.hyph_count);
    // while hyph_word[h]<>0 do
    while $globals.hyph_word[$h] != 0 {
        todo!("!=0")
        // begin @<If the string |hyph_word[h]| is less than \(or)or equal to
        // |s|, interchange |(hyph_word[h],hyph_list[h])| with |(s,p)|@>;
        // if h>0 then decr(h)@+else h:=hyph_size;
        // end;
    }
    // hyph_word[h]:=s; hyph_list[h]:=p
    $globals.hyph_word[$h] = $s;
    $globals.hyph_list[$h] = $p.into();
    use crate::pascal::integer;
    use crate::section_0012::hyph_size;
    use crate::section_0016::incr;
}}
