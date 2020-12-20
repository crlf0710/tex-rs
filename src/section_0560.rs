//! @ \TeX\ checks the information of a \.{TFM} file for validity as the
//! file is being read in, so that no further checks will be needed when
//! typesetting is going on. The somewhat tedious subroutine that does this
//! is called |read_font_info|. It has four parameters: the user font
//! identifier~|u|, the file name and area strings |nom| and |aire|, and the
//! ``at'' size~|s|. If |s|~is negative, it's the negative of a scale factor
//! to be applied to the design size; |s=-1000| is the normal case.
//! Otherwise |s| will be substituted for the design size; in this
//! case, |s| must be positive and less than $2048\rm\,pt$
//! (i.e., it must be less than $2^{27}$ when considered as an integer).
//!
//! The subroutine opens and closes a global file variable called |tfm_file|.
//! It returns the value of the internal font number that was just loaded.
//! If an error is detected, an error message is issued and no font
//! information is stored; |null_font| is returned in this case.
//
// @d bad_tfm=11 {label for |read_font_info|}
// @d abort==goto bad_tfm {do this when the \.{TFM} data is wrong}
//
// @p function read_font_info(@!u:pointer;@!nom,@!aire:str_number;
//   @!s:scaled):internal_font_number; {input a \.{TFM} file}
/// input a `TFM` file
#[allow(unused_variables, unused_assignments)]
pub(crate) fn read_font_info(globals: &mut TeXGlobals, u: pointer, nom: str_number, aire: str_number, s:scaled) -> internal_font_number {
    // label done,bad_tfm,not_found;
    // var k:font_index; {index into |font_info|}
    // @!file_opened:boolean; {was |tfm_file| successfully opened?}
    /// was |tfm_file| successfully opened?
    let file_opened: boolean;
    // @!lf,@!lh,@!bc,@!ec,@!nw,@!nh,@!nd,@!ni,@!nl,@!nk,@!ne,@!np:halfword;
    //   {sizes of subfiles}
    // @!f:internal_font_number; {the new font's number}
    // @!g:internal_font_number; {the number to return}
    /// the number to return
    let g: internal_font_number;
    // @!a,@!b,@!c,@!d:eight_bits; {byte variables}
    // @!qw:four_quarters;@!sw:scaled; {accumulators}
    // @!bch_label:integer; {left boundary start location, or infinity}
    // @!bchar:0..256; {right boundary character, or 256}
    // @!z:scaled; {the design size or the ``at'' size}
    // @!alpha:integer;@!beta:1..16;
    //   {auxiliary quantities used in fixed-point multiplication}
    // begin g:=null_font;@/
    g = internal_font_number::new(null_font as _);
    todo!();
    // @<Read and check the font data; |abort| if the \.{TFM} file is
    //   malformed; if there's no room for this font, say so and |goto
    //   done|; otherwise |incr(font_ptr)| and |goto done|@>;
    // bad_tfm: @<Report that the font won't be loaded@>;
    // done: if file_opened then b_close(tfm_file);
    if file_opened {
        b_close(&mut globals.tfm_file);
    }
    // read_font_info:=g;
    return g;
    // end;
}

use crate::pascal::boolean;
use crate::section_0004::TeXGlobals;
use crate::section_0028::b_close;
use crate::section_0038::str_number;
use crate::section_0101::scaled;
use crate::section_0115::pointer;
use crate::section_0232::null_font;
use crate::section_0548::internal_font_number;