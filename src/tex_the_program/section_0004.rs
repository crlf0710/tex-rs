#![allow(missing_docs)]
//! The program begins with a normal \PASCAL\ program heading, whose
//! components will be filled in later, using the conventions of \.{WEB}.
//! @.WEB@>
//! For example, the portion of the program called `\X\glob:Global
//! variables\X' below will be replaced by a sequence of variable declarations
//! that starts in $\section\glob$ of this documentation. In this way, we are able
//! to define each individual global variable when we are prepared to
//! understand what it means; we do not have to define all of the globals at
//! once.  Cross references in $\section\glob$, where it says ``See also
//! sections \gglob, \dots,'' also make it possible to look at the set of
//! all global variables, if desired.  Similar remarks apply to the other
//! portions of the program heading.
//!
//! Actually the heading shown here is not quite normal: The |program| line
//! does not mention any |output| file, because \ph\ would ask the \TeX\ user
//! to specify a file name if |output| were specified here.
//! @:PASCAL H}{\ph@>
//! @^system dependencies@>

// @d mtype==t@&y@&p@&e {this is a \.{WEB} coding trick:}
// @f mtype==type {`\&{mtype}' will be equivalent to `\&{type}'}
// @f type==true {but `|type|' will not be treated as a reserved word}
//
// @p @t\4@>@<Compiler directives@>@/
// program TEX; {all file names are defined dynamically}
// label @<Labels in the outer block@>@/
// const @<Constants in the outer block@>@/
// mtype @<Types in the outer block@>@/

// var @<Global variables@>@/

#[globals_struct]
#[globals_struct_field_view(TeXGlobalsFilenameView, make_globals_filename_view)]
#[globals_struct_field_view(TeXGlobalsIoView, make_globals_io_view)]
#[globals_struct_field_view(TeXGlobalsLogView, make_globals_log_view)]
#[globals_struct_field_view(TeXGlobalsStringView, make_globals_string_view)]
#[globals_struct_field_view(TeXGlobalsIoStringView, make_globals_io_string_view)]
#[globals_struct_field_view(TeXGlobalsIoStringLogView, make_globals_io_string_log_view)]
#[globals_struct_field_view_decl_macro_ctor]
pub mod TeXGlobals {
    include!("src/tex_the_program/section_0011.rs");
    include!("src/tex_the_program/section_0013.rs");
    include!("src/tex_the_program/section_0026.rs");
    include!("src/tex_the_program/section_0030.rs");
    include!("src/tex_the_program/section_0032.rs");
    include!("src/tex_the_program/section_0039.rs");
    include!("src/tex_the_program/section_0050.rs");
    include!("src/tex_the_program/section_0054.rs");
    include!("src/tex_the_program/section_0073.rs");
    include!("src/tex_the_program/section_0076.rs");
    include!("src/tex_the_program/section_0079.rs");
    include!("src/tex_the_program/section_0096.rs");
    include!("src/tex_the_program/section_0104.rs");
    include!("src/tex_the_program/section_0115.rs");
    include!("src/tex_the_program/section_0116.rs");
    include!("src/tex_the_program/section_0117.rs");
    include!("src/tex_the_program/section_0118.rs");
    include!("src/tex_the_program/section_0124.rs");
    include!("src/tex_the_program/section_0165.rs");
    include!("src/tex_the_program/section_0173.rs");
    include!("src/tex_the_program/section_0181.rs");
    include!("src/tex_the_program/section_0213.rs");
    include!("src/tex_the_program/section_0246.rs");
    include!("src/tex_the_program/section_0253.rs");
    include!("src/tex_the_program/section_0256.rs");
    include!("src/tex_the_program/section_0271.rs");
    include!("src/tex_the_program/section_0286.rs");
    include!("src/tex_the_program/section_0297.rs");
    include!("src/tex_the_program/section_0301.rs");
    include!("src/tex_the_program/section_0304.rs");
    include!("src/tex_the_program/section_0305.rs");
    include!("src/tex_the_program/section_0308.rs");
    include!("src/tex_the_program/section_0309.rs");
    include!("src/tex_the_program/section_0310.rs");
    include!("src/tex_the_program/section_0333.rs");
    include!("src/tex_the_program/section_0361.rs");
    include!("src/tex_the_program/section_0382.rs");
    include!("src/tex_the_program/section_0387.rs");
    include!("src/tex_the_program/section_0388.rs");
    include!("src/tex_the_program/section_0410.rs");
    include!("src/tex_the_program/section_0438.rs");
    include!("src/tex_the_program/section_0447.rs");
    include!("src/tex_the_program/section_0480.rs");
    include!("src/tex_the_program/section_0489.rs");
    include!("src/tex_the_program/section_0493.rs");
    include!("src/tex_the_program/section_0512.rs");
    include!("src/tex_the_program/section_0513.rs");
    include!("src/tex_the_program/section_0520.rs");
    include!("src/tex_the_program/section_0527.rs");
    include!("src/tex_the_program/section_0532.rs");
    include!("src/tex_the_program/section_0539.rs");
    include!("src/tex_the_program/section_0549.rs");
    include!("src/tex_the_program/section_0550.rs");
    include!("src/tex_the_program/section_0592.rs");
    include!("src/tex_the_program/section_0595.rs");
    include!("src/tex_the_program/section_0605.rs");
    include!("src/tex_the_program/section_0616.rs");
    include!("src/tex_the_program/section_0646.rs");
    include!("src/tex_the_program/section_0647.rs");
    include!("src/tex_the_program/section_0661.rs");
    include!("src/tex_the_program/section_0719.rs");
    include!("src/tex_the_program/section_0770.rs");
    include!("src/tex_the_program/section_0814.rs");
    include!("src/tex_the_program/section_0821.rs");
    include!("src/tex_the_program/section_0823.rs");
    include!("src/tex_the_program/section_0825.rs");
    include!("src/tex_the_program/section_0828.rs");
    include!("src/tex_the_program/section_0833.rs");
    include!("src/tex_the_program/section_0839.rs");
    include!("src/tex_the_program/section_0847.rs");
    include!("src/tex_the_program/section_0872.rs");
    include!("src/tex_the_program/section_0892.rs");
    include!("src/tex_the_program/section_0900.rs");
    include!("src/tex_the_program/section_0905.rs");
    include!("src/tex_the_program/section_0907.rs");
    include!("src/tex_the_program/section_0921.rs");
    include!("src/tex_the_program/section_0926.rs");
    include!("src/tex_the_program/section_0943.rs");
    include!("src/tex_the_program/section_0947.rs");
    include!("src/tex_the_program/section_0950.rs");
    include!("src/tex_the_program/section_0971.rs");
    include!("src/tex_the_program/section_0980.rs");
    include!("src/tex_the_program/section_0982.rs");
    include!("src/tex_the_program/section_0989.rs");
    include!("src/tex_the_program/section_1032.rs");
    include!("src/tex_the_program/section_1074.rs");
    include!("src/tex_the_program/section_1266.rs");
    include!("src/tex_the_program/section_1299.rs");
    include!("src/tex_the_program/section_1305.rs");
    include!("src/tex_the_program/section_1331.rs");
    include!("src/tex_the_program/section_1342.rs");
    include!("src/tex_the_program/section_1345.rs");
    include!("src/latex_support.rs");
    include!("src/unicode_support.rs");
}

crate::impl_debug_with_literal!(TeXGlobals, "TeXGlobals");
crate::impl_debug_with_literal!(TeXGlobalsIoView['view], "TeXGlobalsIoView");
crate::impl_debug_with_literal!(TeXGlobalsIoStringView['view], "TeXGlobalsIoStringView");

// @#
// procedure initialize; {this procedure gets things started properly}
/// this procedure gets things started properly
#[allow(unused_variables)]
pub(crate) fn initialize(globals: &mut TeXGlobals) {
    // var @<Local variables for initialization@>@/
    // begin @<Initialize whatever \TeX\ might access@>@;
    crate::section_0008::Initialize_whatever_TeX_might_access!(globals);
    // end;@#
}

// @t\4@>@<Basic printing procedures@>@/
// @t\4@>@<Error handling procedures@>@/

use globals_struct::{globals_struct, globals_struct_field_view};
