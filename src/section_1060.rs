//! @ All the work relating to glue creation has been relegated to the
//! following subroutine. It does not call |build_page|, because it is
//! used in at least one place where that would be a mistake.
//
// @<Declare action...@>=
// procedure append_glue;
#[allow(unused_variables)]
pub(crate) fn append_glue(globals: &mut TeXGlobals) {
    // var s:small_number; {modifier of skip command}
    // begin s:=cur_chr;
    // case s of
    // fil_code: cur_val:=fil_glue;
    // fill_code: cur_val:=fill_glue;
    // ss_code: cur_val:=ss_glue;
    // fil_neg_code: cur_val:=fil_neg_glue;
    // skip_code: scan_glue(glue_val);
    // mskip_code: scan_glue(mu_val);
    // end; {now |cur_val| points to the glue specification}
    // tail_append(new_glue(cur_val));
    // if s>=skip_code then
    //   begin decr(glue_ref_count(cur_val));
    //   if s>skip_code then subtype(tail):=mu_glue;
    //   end;
    // end;
}

use crate::section_0004::TeXGlobals;
