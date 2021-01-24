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
//   begin main_p:=new_ligature(main_f,cur_l,link(cur_q));
//   if lft_hit then
//     begin subtype(main_p):=2; lft_hit:=false;
//     end;
//   if # then if lig_stack=null then
//     begin incr(subtype(main_p)); rt_hit:=false;
//     end;
//   link(cur_q):=main_p; tail:=main_p; ligature_present:=false;
//   end
//
// @d wrapup(#)==if cur_l<non_char then
macro_rules! wrapup {
    ($globals:expr, $v:expr) => {{
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
                todo!("pack_lig");
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
        use crate::section_0145::new_disc;
    }};
}

// @<Make a ligature node, if |ligature_present|;...@>=
macro_rules! Make_a_ligature_node__if_ligature_present__insert_a_null_discretionary__if_appropriate {
    ($globals:expr) => {{
        // wrapup(rt_hit)
        wrapup!($globals, $globals.rt_hit);
    }};
}
