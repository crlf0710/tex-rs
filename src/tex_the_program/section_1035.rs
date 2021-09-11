//! @ If |link(cur_q)| is nonnull when |wrapup| is invoked, |cur_q| points to
//! the list of characters that were consumed while building the ligature
//! character~|cur_l|.
//!
//! A discretionary break is not inserted for an explicit hyphen when we are in
//! restricted horizontal mode. In particular, this avoids putting discretionary
//! nodes inside of other discretionaries.
//! @^inner loop@>
//
// @d pack_lig(#)== {the parameter is either |rt_hit| or |false|}
pub(crate) macro pack_lig($globals:expr, $v:expr) {{
    /// the parameter is either `rt_hit` or `false`
    const _: () = ();
    // begin main_p:=new_ligature(main_f,cur_l,link(cur_q));
    $globals.main_p = new_ligature(
        $globals,
        $globals.main_f,
        ASCII_code::from($globals.cur_l as integer),
        link!($globals, $globals.cur_q),
    )?;
    // if lft_hit then
    if $globals.lft_hit {
        // begin subtype(main_p):=2; lft_hit:=false;
        subtype!($globals, $globals.main_p) = 2;
        $globals.lft_hit = false;
        // end;
    }
    // if # then if lig_stack=null then
    if $v && $globals.lig_stack == null {
        // begin incr(subtype(main_p)); rt_hit:=false;
        incr!(subtype!($globals, $globals.main_p));
        $globals.rt_hit = false;
        // end;
    }
    // link(cur_q):=main_p; tail:=main_p; ligature_present:=false;
    link!($globals, $globals.cur_q) = $globals.main_p;
    tail!($globals) = $globals.main_p;
    $globals.ligature_present = false;
    // end
    use crate::pascal::integer;
    use crate::section_0016::incr;
    use crate::section_0018::ASCII_code;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0133::subtype;
    use crate::section_0144::new_ligature;
    use crate::section_0213::tail;
}}

// @d wrapup(#)==if cur_l<non_char then
pub(crate) macro wrapup($globals:expr, $v:expr) {{
    crate::trace_span!("wrapup");
    if $globals.cur_l < non_char {
        // begin if link(cur_q)>null then
        if link!($globals, $globals.cur_q) > null {
            // if character(tail)=qi(hyphen_char[main_f]) then ins_disc:=true;
            if character!($globals, tail!($globals)).numeric_value() as integer
                == $globals.hyphen_char[$globals.main_f]
            {
                $globals.ins_disc = true;
            }
        }
        // if ligature_present then pack_lig(#);
        if $globals.ligature_present {
            pack_lig!($globals, $v);
        }
        // if ins_disc then
        if $globals.ins_disc {
            // begin ins_disc:=false;
            $globals.ins_disc = false;
            // if mode>0 then tail_append(new_disc);
            if mode!($globals) > 0 {
                tail_append!($globals, new_disc($globals)?);
            }
            // end;
        }
        // end
    }
    use crate::pascal::integer;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0134::character;
    use crate::section_0145::new_disc;
    use crate::section_0213::mode;
    use crate::section_0213::tail;
    use crate::section_0214::tail_append;
    use crate::section_0549::non_char;
}}

// @<Make a ligature node, if |ligature_present|;...@>=
pub(crate) macro Make_a_ligature_node__if_ligature_present__insert_a_null_discretionary__if_appropriate($globals:expr) {{
    // wrapup(rt_hit)
    wrapup!($globals, $globals.rt_hit);
}}
