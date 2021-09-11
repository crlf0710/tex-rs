//! @ The |lig_kern| array contains instructions in a simple programming language
//! that explains what to do for special letter pairs. Each word in this array is a
//! |@!lig_kern_command| of four bytes.
//!
//! \yskip\hang first byte: |skip_byte|, indicates that this is the final program
//!   step if the byte is 128 or more, otherwise the next step is obtained by
//!   skipping this number of intervening steps.\par
//! \hang second byte: |next_char|, ``if |next_char| follows the current character,
//!   then perform the operation and stop, otherwise continue.''\par
//! \hang third byte: |op_byte|, indicates a ligature step if less than~128,
//!   a kern step otherwise.\par
//! \hang fourth byte: |remainder|.\par
//! \yskip\noindent
//! In a kern step, an
//! additional space equal to |kern[256*(op_byte-128)+remainder]| is inserted
//! between the current character and |next_char|. This amount is
//! often negative, so that the characters are brought closer together
//! by kerning; but it might be positive.
//!
//! There are eight kinds of ligature steps, having |op_byte| codes $4a+2b+c$ where
//! $0\le a\le b+c$ and $0\le b,c\le1$. The character whose code is
//! |remainder| is inserted between the current character and |next_char|;
//! then the current character is deleted if $b=0$, and |next_char| is
//! deleted if $c=0$; then we pass over $a$~characters to reach the next
//! current character (which may have a ligature/kerning program of its own).
//!
//! If the very first instruction of the |lig_kern| array has |skip_byte=255|,
//! the |next_char| byte is the so-called boundary character of this font;
//! the value of |next_char| need not lie between |bc| and~|ec|.
//! If the very last instruction of the |lig_kern| array has |skip_byte=255|,
//! there is a special ligature/kerning program for a boundary character at the
//! left, beginning at location |256*op_byte+remainder|.
//! The interpretation is that \TeX\ puts implicit boundary characters
//! before and after each consecutive string of characters from the same font.
//! These implicit characters do not appear in the output, but they can affect
//! ligatures and kerning.
//!
//! If the very first instruction of a character's |lig_kern| program has
//! |skip_byte>128|, the program actually begins in location
//! |256*op_byte+remainder|. This feature allows access to large |lig_kern|
//! arrays, because the first instruction must otherwise
//! appear in a location |<=255|.
//!
//! Any instruction with |skip_byte>128| in the |lig_kern| array must satisfy
//! the condition
//! $$\hbox{|256*op_byte+remainder<nl|.}$$
//! If such an instruction is encountered during
//! normal program execution, it denotes an unconditional halt; no ligature
//! or kerning command is performed.
//
// @d stop_flag==qi(128) {value indicating `\.{STOP}' in a lig/kern program}
/// value indicating `STOP` in a lig/kern program
pub(crate) const stop_flag: quarterword = qi!(128);
// @d kern_flag==qi(128) {op code for a kern step}
/// op code for a kern step
pub(crate) const kern_flag: quarterword = qi!(128);

#[derive(Copy, Clone, RefCast)]
#[repr(transparent)]
pub(crate) struct lig_kern_cmd(four_quarters);

impl Default for lig_kern_cmd {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl lig_kern_cmd {
    // @d skip_byte(#)==#.b0
    pub(crate) fn skip_byte(self) -> quarterword {
        self.0[FOUR_QUARTERS_B0]
    }
    // @d next_char(#)==#.b1
    pub(crate) fn next_char(self) -> quarterword {
        self.0[FOUR_QUARTERS_B1]
    }
    // @d op_byte(#)==#.b2
    pub(crate) fn op_byte(self) -> quarterword {
        self.0[FOUR_QUARTERS_B2]
    }
    // @d rem_byte(#)==#.b3
    pub(crate) fn rem_byte(self) -> quarterword {
        self.0[FOUR_QUARTERS_B3]
    }
}

pub(crate) struct MEMORY_WORD_LIG_KERN_CMD;

impl Index<MEMORY_WORD_LIG_KERN_CMD> for memory_word {
    type Output = lig_kern_cmd;
    fn index(&self, _: MEMORY_WORD_LIG_KERN_CMD) -> &lig_kern_cmd {
        lig_kern_cmd::ref_cast(&self[MEMORY_WORD_QQQQ])
    }
}

impl IndexMut<MEMORY_WORD_LIG_KERN_CMD> for memory_word {
    fn index_mut(&mut self, _: MEMORY_WORD_LIG_KERN_CMD) -> &mut lig_kern_cmd {
        lig_kern_cmd::ref_cast_mut(&mut self[MEMORY_WORD_QQQQ])
    }
}

use crate::section_0112::qi;
use crate::section_0113::four_quarters;
use crate::section_0113::memory_word;
use crate::section_0113::quarterword;
use crate::section_0113::MEMORY_WORD_QQQQ;
use crate::section_0113::{FOUR_QUARTERS_B0, FOUR_QUARTERS_B1, FOUR_QUARTERS_B2, FOUR_QUARTERS_B3};
use core::ops::{Index, IndexMut};
use ref_cast::RefCast;
