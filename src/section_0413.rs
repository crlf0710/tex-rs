//! @ OK, we're ready for |scan_something_internal| itself. A second parameter,
//! |negative|, is set |true| if the value that is found should be negated.
//! It is assumed that |cur_cmd| and |cur_chr| represent the first token of
//! the internal quantity to be scanned; an error will be signalled if
//! |cur_cmd<min_internal| or |cur_cmd>max_internal|.
//!
//! @d scanned_result_end(#)==cur_val_level:=#;@+end
//! @d scanned_result(#)==@+begin cur_val:=#;scanned_result_end
//!
//! @p procedure scan_something_internal(@!level:small_number;@!negative:boolean);
//!   {fetch an internal parameter}
//! var m:halfword; {|chr_code| part of the operand token}
//! @!p:0..nest_size; {index into |nest|}
//! begin m:=cur_chr;
//! case cur_cmd of
//! def_code: @<Fetch a character code from some table@>;
//! toks_register,assign_toks,def_family,set_font,def_font: @<Fetch a token list or
//!   font identifier, provided that |level=tok_val|@>;
//! assign_int: scanned_result(eqtb[m].int)(int_val);
//! assign_dimen: scanned_result(eqtb[m].sc)(dimen_val);
//! assign_glue: scanned_result(equiv(m))(glue_val);
//! assign_mu_glue: scanned_result(equiv(m))(mu_val);
//! set_aux: @<Fetch the |space_factor| or the |prev_depth|@>;
//! set_prev_graf: @<Fetch the |prev_graf|@>;
//! set_page_int:@<Fetch the |dead_cycles| or the |insert_penalties|@>;
//! set_page_dimen: @<Fetch something on the |page_so_far|@>;
//! set_shape: @<Fetch the |par_shape| size@>;
//! set_box_dimen: @<Fetch a box dimension@>;
//! char_given,math_given: scanned_result(cur_chr)(int_val);
//! assign_font_dimen: @<Fetch a font dimension@>;
//! assign_font_int: @<Fetch a font integer@>;
//! register: @<Fetch a register@>;
//! last_item: @<Fetch an item in the current node, if appropriate@>;
//! othercases @<Complain that \.{\\the} can't do this; give zero result@>
//! endcases;@/
//! while cur_val_level>level do @<Convert \(c)|cur_val| to a lower level@>;
//! @<Fix the reference count, if any, and negate |cur_val| if |negative|@>;
//! end;
//!
