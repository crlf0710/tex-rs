//! @ The final portion of a \.{TFM} file is the |param| array, which is another
//! sequence of |fix_word| values.
//!
//! \yskip\hang|param[1]=slant| is the amount of italic slant, which is used
//! to help position accents. For example, |slant=.25| means that when you go
//! up one unit, you also go .25 units to the right. The |slant| is a pure
//! number; it's the only |fix_word| other than the design size itself that is
//! not scaled by the design size.
//!
//! \hang|param[2]=space| is the normal spacing between words in text.
//! Note that character |" "| in the font need not have anything to do with
//! blank spaces.
//!
//! \hang|param[3]=space_stretch| is the amount of glue stretching between words.
//!
//! \hang|param[4]=space_shrink| is the amount of glue shrinking between words.
//!
//! \hang|param[5]=x_height| is the size of one ex in the font; it is also
//! the height of letters for which accents don't have to be raised or lowered.
//!
//! \hang|param[6]=quad| is the size of one em in the font.
//!
//! \hang|param[7]=extra_space| is the amount added to |param[2]| at the
//! ends of sentences.
//!
//! \yskip\noindent
//! If fewer than seven parameters are present, \TeX\ sets the missing parameters
//! to zero. Fonts used for math symbols are required to have
//! additional parameter information, which is explained later.
//
// @d slant_code=1
// @d space_code=2
pub(crate) const space_code: quarterword = 2;
// @d space_stretch_code=3
// @d space_shrink_code=4
pub(crate) const space_shrink_code: quarterword = 4;
// @d x_height_code=5
// @d quad_code=6
// @d extra_space_code=7
pub(crate) const extra_space_code: quarterword = 7;

use crate::section_0113::quarterword;
