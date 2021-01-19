//! @ OK, we're ready for |scan_something_internal| itself. A second parameter,
//! |negative|, is set |true| if the value that is found should be negated.
//! It is assumed that |cur_cmd| and |cur_chr| represent the first token of
//! the internal quantity to be scanned; an error will be signalled if
//! |cur_cmd<min_internal| or |cur_cmd>max_internal|.
//
// @d scanned_result_end(#)==cur_val_level:=#;@+end
const _: () = ();
// @d scanned_result(#)==@+begin cur_val:=#;scanned_result_end
macro_rules! scanned_result {
    ($globals:expr, $val:expr, $level:expr) => {{
        $globals.cur_val = $val;
        $globals.cur_val_level = $level;
    }};
}
//
// @p procedure scan_something_internal(@!level:small_number;@!negative:boolean);
//   {fetch an internal parameter}
/// fetch an internal parameter
#[allow(unused_variables)]
pub(crate) fn scan_something_internal(
    globals: &mut TeXGlobals,
    level: small_number,
    negative: boolean,
) -> TeXResult<()> {
    // var m:halfword; {|chr_code| part of the operand token}
    /// `chr_code` part of the operand token
    let mut m: chr_code_type;
    // @!p:0..nest_size; {index into |nest|}
    // begin m:=cur_chr;
    m = globals.cur_chr;
    trace_expr!("cur_cmd = {}", globals.cur_cmd);
    // case cur_cmd of
    // def_code: @<Fetch a character code from some table@>;
    if globals.cur_cmd == def_code {
        Fetch_a_character_code_from_some_table!(globals, m);
    }
    // toks_register,assign_toks,def_family,set_font,def_font: @<Fetch a token list or
    //   font identifier, provided that |level=tok_val|@>;
    else if globals.cur_cmd == toks_register
        || globals.cur_cmd == assign_toks
        || globals.cur_cmd == def_family
        || globals.cur_cmd == set_font
        || globals.cur_cmd == def_font
    {
        Fetch_a_token_list_or_font_identifier__provided_that_level_is_tok_val!(globals, level, m);
    }
    // assign_int: scanned_result(eqtb[m].int)(int_val);
    else if globals.cur_cmd == assign_int {
        scanned_result!(
            globals,
            globals.eqtb[m.get() as pointer][MEMORY_WORD_INT],
            cur_val_level_kind::int_val
        );
    }
    // assign_dimen: scanned_result(eqtb[m].sc)(dimen_val);
    else if globals.cur_cmd == assign_dimen {
        scanned_result!(
            globals,
            globals.eqtb[m.get() as pointer][MEMORY_WORD_SC].inner(),
            cur_val_level_kind::dimen_val
        );
    }
    // assign_glue: scanned_result(equiv(m))(glue_val);
    else if globals.cur_cmd == assign_glue {
        scanned_result!(
            globals,
            equiv!(globals, m.get() as pointer) as _,
            cur_val_level_kind::glue_val
        );
    }
    // assign_mu_glue: scanned_result(equiv(m))(mu_val);
    else if globals.cur_cmd == assign_mu_glue {
        scanned_result!(
            globals,
            equiv!(globals, m.get() as pointer) as _,
            cur_val_level_kind::mu_val
        );
    }
    // set_aux: @<Fetch the |space_factor| or the |prev_depth|@>;
    // set_prev_graf: @<Fetch the |prev_graf|@>;
    // set_page_int:@<Fetch the |dead_cycles| or the |insert_penalties|@>;
    // set_page_dimen: @<Fetch something on the |page_so_far|@>;
    else if globals.cur_cmd == set_page_dimen {
        Fetch_something_on_the_page_so_far!(globals, m);
    }
    // set_shape: @<Fetch the |par_shape| size@>;
    // set_box_dimen: @<Fetch a box dimension@>;
    else if globals.cur_cmd == set_box_dimen {
        Fetch_a_box_dimension!(globals, m);
    }
    // char_given,math_given: scanned_result(cur_chr)(int_val);
    else if globals.cur_cmd == char_given || globals.cur_cmd == math_given {
        scanned_result!(
            globals,
            globals.cur_chr.get() as _,
            cur_val_level_kind::int_val
        );
    }
    // assign_font_dimen: @<Fetch a font dimension@>;
    else if globals.cur_cmd == assign_font_dimen {
        Fetch_a_font_dimension!(globals);
    }
    // assign_font_int: @<Fetch a font integer@>;
    else if globals.cur_cmd == assign_font_int {
        Fetch_a_font_integer!(globals, m);
    }
    // register: @<Fetch a register@>;
    else if globals.cur_cmd == register {
        Fetch_a_register!(globals, m);
    }
    // last_item: @<Fetch an item in the current node, if appropriate@>;
    else if globals.cur_cmd == last_item {
        Fetch_an_item_in_the_current_node__if_appropriate!(globals);
    }
    // othercases @<Complain that \.{\\the} can't do this; give zero result@>
    else {
        Complain_that_the_cant_do_this__give_zero_result!(globals, level);
    }
    // endcases;@/
    // while cur_val_level>level do @<Convert \(c)|cur_val| to a lower level@>;
    while globals.cur_val_level as integer > level.get() as integer {
        Convert_cur_val_to_a_lower_level!(globals);
    }
    // @<Fix the reference count, if any, and negate |cur_val| if |negative|@>;
    Fix_the_reference_count__if_any__and_negate_cur_val_if_negative!(globals, negative);
    // end;
    ok_nojump!()
}

use crate::pascal::boolean;
use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::small_number;
use crate::section_0101::MEMORY_WORD_SC;
use crate::section_0113::MEMORY_WORD_INT;
use crate::section_0115::pointer;
use crate::section_0208::*;
use crate::section_0209::*;
use crate::section_0297::chr_code_type;
use crate::section_0410::cur_val_level_kind;
