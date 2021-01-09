//! @ The main reason for wanting |str_toks| is the next function,
//! |the_toks|, which has similar input/output characteristics.
//!
//! This procedure is supposed to scan something like `\.{\\skip\\count12}',
//! i.e., whatever can follow `\.{\\the}', and it constructs a token list
//! containing something like `\.{-3.0pt minus 0.5fill}'.
//
// @p function the_toks:pointer;
pub(crate) fn the_toks(globals: &mut TeXGlobals) -> TeXResult<pointer> {
    // var old_setting:0..max_selector; {holds |selector| setting}
    /// holds `selector` setting
    let old_setting;
    // @!p,@!q,@!r:pointer; {used for copying a token list}
    // @!b:pool_pointer; {base of temporary string}
    /// base of temporary string
    let b: pool_pointer;
    // begin get_x_token; scan_something_internal(tok_val,false);
    get_x_token(globals)?;
    scan_something_internal(
        globals,
        small_number::new(cur_val_level_kind::tok_val as _),
        false,
    )?;
    let the_toks;
    // if cur_val_level>=ident_val then @<Copy the token list@>
    if globals.cur_val_level >= cur_val_level_kind::ident_val {
        Copy_the_token_list!(globals, the_toks);
    }
    // else begin old_setting:=selector; selector:=new_string; b:=pool_ptr;
    else {
        old_setting = globals.selector;
        globals.selector = new_string.into();
        b = globals.pool_ptr;
        // case cur_val_level of
        // int_val:print_int(cur_val);
        if globals.cur_val_level == cur_val_level_kind::int_val {
            print_int(globals, globals.cur_val);
        }
        // dimen_val:begin print_scaled(cur_val); print("pt");
        //   end;
        else if globals.cur_val_level == cur_val_level_kind::dimen_val {
            print_scaled(globals, scaled::new_from_inner(globals.cur_val));
            print(globals, strpool_str!("pt").get() as _);
        }
        // glue_val: begin print_spec(cur_val,"pt"); delete_glue_ref(cur_val);
        //   end;
        else if globals.cur_val_level == cur_val_level_kind::glue_val {
            print_spec(globals, globals.cur_val, strpool_str!("pt"));
            delete_glue_ref(globals, globals.cur_val as pointer);
        }
        // mu_val: begin print_spec(cur_val,"mu"); delete_glue_ref(cur_val);
        //   end;
        else if globals.cur_val_level == cur_val_level_kind::mu_val {
            print_spec(globals, globals.cur_val, strpool_str!("mu"));
            delete_glue_ref(globals, globals.cur_val as pointer);
        }
        else {
            // end; {there are no other cases}
            /// there are no other cases
            unreachable!()
        }
        // selector:=old_setting; the_toks:=str_toks(b);
        globals.selector = old_setting;
        the_toks = str_toks(globals, b);
        // end;
    }
    // end;
    ok_nojump!(the_toks)
}

use crate::section_0004::TeXGlobals;
use crate::section_0038::pool_pointer;
use crate::section_0054::new_string;
use crate::section_0059::print;
use crate::section_0065::print_int;
use crate::section_0081::TeXResult;
use crate::section_0101::small_number;
use crate::section_0101::scaled;
use crate::section_0103::print_scaled;
use crate::section_0115::pointer;
use crate::section_0178::print_spec;
use crate::section_0201::delete_glue_ref;
use crate::section_0380::get_x_token;
use crate::section_0410::cur_val_level_kind;
use crate::section_0413::scan_something_internal;
use crate::section_0464::str_toks;
