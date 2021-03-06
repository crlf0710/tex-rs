//! @* \[28] Conditional processing.
//! We consider now the way \TeX\ handles various kinds of \.{\\if} commands.
//
// @d if_char_code=0 { `\.{\\if}' }
/// `\if`
pub(crate) const if_char_code: quarterword = 0;
// @d if_cat_code=1 { `\.{\\ifcat}' }
/// `\ifcat`
pub(crate) const if_cat_code: quarterword = 1;
// @d if_int_code=2 { `\.{\\ifnum}' }
/// `\ifnum`
pub(crate) const if_int_code: quarterword = 2;
// @d if_dim_code=3 { `\.{\\ifdim}' }
/// `\ifdim`
pub(crate) const if_dim_code: quarterword = 3;
// @d if_odd_code=4 { `\.{\\ifodd}' }
// @d if_vmode_code=5 { `\.{\\ifvmode}' }
/// `\ifvmode`
pub(crate) const if_vmode_code: quarterword = 5;
// @d if_hmode_code=6 { `\.{\\ifhmode}' }
// @d if_mmode_code=7 { `\.{\\ifmmode}' }
/// `\ifmmode`
pub(crate) const if_mmode_code: quarterword = 7;
// @d if_inner_code=8 { `\.{\\ifinner}' }
// @d if_void_code=9 { `\.{\\ifvoid}' }
/// `\ifvoid'
pub(crate) const if_void_code: quarterword = 9;
// @d if_hbox_code=10 { `\.{\\ifhbox}' }
/// `\ifhbox'
pub(crate) const if_hbox_code: quarterword = 10;
// @d if_vbox_code=11 { `\.{\\ifvbox}' }
/// `\ifvbox`
pub(crate) const if_vbox_code: quarterword = 11;
// @d ifx_code=12 { `\.{\\ifx}' }
/// `\ifx`
pub(crate) const ifx_code: quarterword = 12;
// @d if_eof_code=13 { `\.{\\ifeof}' }
///  `\ifeof` 
pub(crate) const if_eof_code: quarterword = 13;
// @d if_true_code=14 { `\.{\\iftrue}' }
/// `\iftrue`
pub(crate) const if_true_code: quarterword = 14;
// @d if_false_code=15 { `\.{\\iffalse}' }
///  `\iffalse`
pub(crate) const if_false_code: quarterword = 15;
// @d if_case_code=16 { `\.{\\ifcase}' }
/// `\ifcase`
pub(crate) const if_case_code: quarterword = 16;

// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
#[allow(unused_variables)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_0487(globals: &mut TeXGlobals) {
    // primitive("if",if_test,if_char_code);
    primitive(globals, strpool_str!("if"), if_test, if_char_code as _);
    // @!@:if_char_}{\.{\\if} primitive@>
    // primitive("ifcat",if_test,if_cat_code);
    // @!@:if_cat_code_}{\.{\\ifcat} primitive@>
    // primitive("ifnum",if_test,if_int_code);
    primitive(globals, strpool_str!("ifnum"), if_test, if_int_code as _);
    // @!@:if_int_}{\.{\\ifnum} primitive@>
    // primitive("ifdim",if_test,if_dim_code);
    primitive(globals, strpool_str!("ifdim"), if_test, if_dim_code as _);
    // @!@:if_dim_}{\.{\\ifdim} primitive@>
    // primitive("ifodd",if_test,if_odd_code);
    // @!@:if_odd_}{\.{\\ifodd} primitive@>
    // primitive("ifvmode",if_test,if_vmode_code);
    primitive(globals, strpool_str!("ifvmode"), if_test, if_vmode_code as _);
    // @!@:if_vmode_}{\.{\\ifvmode} primitive@>
    // primitive("ifhmode",if_test,if_hmode_code);
    // @!@:if_hmode_}{\.{\\ifhmode} primitive@>
    // primitive("ifmmode",if_test,if_mmode_code);
    primitive(globals, strpool_str!("ifmmode"), if_test, if_mmode_code as _);
    // @!@:if_mmode_}{\.{\\ifmmode} primitive@>
    // primitive("ifinner",if_test,if_inner_code);
    // @!@:if_inner_}{\.{\\ifinner} primitive@>
    // primitive("ifvoid",if_test,if_void_code);
    primitive(globals, strpool_str!("ifvoid"), if_test, if_void_code as _);
    // @!@:if_void_}{\.{\\ifvoid} primitive@>
    // primitive("ifhbox",if_test,if_hbox_code);
    // @!@:if_hbox_}{\.{\\ifhbox} primitive@>
    // primitive("ifvbox",if_test,if_vbox_code);
    // @!@:if_vbox_}{\.{\\ifvbox} primitive@>
    // primitive("ifx",if_test,ifx_code);
    primitive(globals, strpool_str!("ifx"), if_test, ifx_code as _);
    // @!@:ifx_}{\.{\\ifx} primitive@>
    // primitive("ifeof",if_test,if_eof_code);
    primitive(globals, strpool_str!("ifeof"), if_test, if_eof_code as _);
    // @!@:if_eof_}{\.{\\ifeof} primitive@>
    // primitive("iftrue",if_test,if_true_code);
    primitive(globals, strpool_str!("iftrue"), if_test, if_true_code as _);
    // @!@:if_true_}{\.{\\iftrue} primitive@>
    // primitive("iffalse",if_test,if_false_code);
    primitive(globals, strpool_str!("iffalse"), if_test, if_false_code as _);
    // @!@:if_false_}{\.{\\iffalse} primitive@>
    // primitive("ifcase",if_test,if_case_code);
    primitive(globals, strpool_str!("ifcase"), if_test, if_case_code as _);
    // @!@:if_case_}{\.{\\ifcase} primitive@>
}

use crate::pascal::word;
use crate::section_0004::TeXGlobals;
use crate::section_0113::quarterword;
use crate::section_0210::*;
use crate::section_0264::primitive;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
