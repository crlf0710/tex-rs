//! @ The final member of \TeX's value-scanning trio is |scan_glue|, which
//! makes |cur_val| point to a glue specification. The reference count of that
//! glue spec will take account of the fact that |cur_val| is pointing to~it.
//!
//! The |level| parameter should be either |glue_val| or |mu_val|.
//!
//! Since |scan_dimen| was so much more complex than |scan_int|, we might expect
//! |scan_glue| to be even worse. But fortunately, it is very simple, since
//! most of the work has already been done.
//
// @p procedure scan_glue(@!level:small_number);
//   {sets |cur_val| to a glue spec pointer}
/// sets `cur_val` to a glue spec pointer
pub(crate) fn scan_glue(globals: &mut TeXGlobals, level: small_number) -> TeXResult<()> {
    // label exit;
    // var negative:boolean; {should the answer be negated?}
    /// should the answer be negated?
    let mut negative: boolean;
    // @!q:pointer; {new glue specification}
    // @!mu:boolean; {does |level=mu_val|?}
    /// does `level=mu_val`?
    let mu: boolean;
    // begin mu:=(level=mu_val); @<Get the next non-blank non-sign...@>;
    mu = level == small_number::new(cur_val_level_kind::mu_val as _);
    Get_the_next_non_blank_non_sign_token__set_negative_appropriately!(globals, negative);
    // if (cur_cmd>=min_internal)and(cur_cmd<=max_internal) then
    if globals.cur_cmd >= min_internal && globals.cur_cmd <= max_internal {
        todo!("1");
        // begin scan_something_internal(level,negative);
        // if cur_val_level>=glue_val then
        //   begin if cur_val_level<>level then mu_error;
        //   return;
        //   end;
        // if cur_val_level=int_val then scan_dimen(mu,false,true)
        // else if level=mu_val then mu_error;
        // end
    }
    // else  begin back_input; scan_dimen(mu,false,false);
    else {
        back_input(globals);
        scan_dimen(globals, mu, false, false)?;
        // if negative then negate(cur_val);
        if negative {
            negate!(globals.cur_val);
        }
        // end;
    }
    // @<Create a new glue specification whose width is |cur_val|; scan for its
    //   stretch and shrink components@>;
    Create_a_new_glue_specification_whose_width_is_cur_val__scan_for_its_stretch_and_shrink_components!(globals, mu);
    // exit:end;
    ok_nojump!()
}

use crate::pascal::boolean;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::small_number;
use crate::section_0208::min_internal;
use crate::section_0209::max_internal;
use crate::section_0325::back_input;
use crate::section_0410::cur_val_level_kind;
use crate::section_0448::scan_dimen;