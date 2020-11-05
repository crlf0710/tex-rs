//! @ Here is where we clear the parameters that are supposed to revert to their
//! default values after every paragraph and when internal vertical mode is entered.
//!
//! @<Declare act...@>=
//! procedure normal_paragraph;
pub(crate) fn normal_paragraph() {
//! begin if looseness<>0 then eq_word_define(int_base+looseness_code,0);
//! if hang_indent<>0 then eq_word_define(dimen_base+hang_indent_code,0);
//! if hang_after<>1 then eq_word_define(int_base+hang_after_code,1);
//! if par_shape_ptr<>null then eq_define(par_shape_loc,shape_ref,null);
//! end;
}
