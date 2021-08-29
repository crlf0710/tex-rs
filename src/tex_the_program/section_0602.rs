//! @ Here's a procedure that outputs a font definition. Since \TeX82 uses at
//! most 256 different fonts per job, |fnt_def1| is always used as the command code.
//
// @p procedure dvi_font_def(@!f:internal_font_number);
pub(crate) fn dvi_font_def(globals: &mut TeXGlobals, f: internal_font_number) {
    // var k:pool_pointer; {index into |str_pool|}
    // begin dvi_out(fnt_def1);
    dvi_out!(globals, fnt_def1.byte());
    // dvi_out(f-font_base-1);@/
    dvi_out!(globals, f.get() as integer - font_base as integer - 1);
    // dvi_out(qo(font_check[f].b0));
    dvi_out!(globals, globals.font_check[f][FOUR_QUARTERS_B0]);
    // dvi_out(qo(font_check[f].b1));
    dvi_out!(globals, globals.font_check[f][FOUR_QUARTERS_B1]);
    // dvi_out(qo(font_check[f].b2));
    dvi_out!(globals, globals.font_check[f][FOUR_QUARTERS_B2]);
    // dvi_out(qo(font_check[f].b3));@/
    dvi_out!(globals, globals.font_check[f][FOUR_QUARTERS_B3]);
    // dvi_four(font_size[f]);
    dvi_four(globals, globals.font_size[f].inner());
    // dvi_four(font_dsize[f]);@/
    dvi_four(globals, globals.font_dsize[f].inner());
    // dvi_out(length(font_area[f]));
    dvi_out!(globals, length(globals, globals.font_area[f].get() as _));
    // dvi_out(length(font_name[f]));
    dvi_out!(globals, length(globals, globals.font_name[f].get() as _));
    // @<Output the font name whose internal number is |f|@>;
    Output_the_font_name_whose_internal_number_is_f!(globals, f);
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0012::font_base;
use crate::section_0040::length;
use crate::section_0113::FOUR_QUARTERS_B0;
use crate::section_0113::FOUR_QUARTERS_B1;
use crate::section_0113::FOUR_QUARTERS_B2;
use crate::section_0113::FOUR_QUARTERS_B3;
use crate::section_0548::internal_font_number;
use crate::section_0586::fnt_def1;
use crate::section_0600::dvi_four;
