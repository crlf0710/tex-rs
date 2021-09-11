//! @ The |info| fields in the entries of the down stack or the right stack
//! have six possible settings: |y_here| or |z_here| mean that the \.{DVI}
//! command refers to |y| or |z|, respectively (or to |w| or |x|, in the
//! case of horizontal motion); |yz_OK| means that the \.{DVI} command is
//! \\{down} (or \\{right}) but can be changed to either |y| or |z| (or
//! to either |w| or |x|); |y_OK| means that it is \\{down} and can be changed
//! to |y| but not |z|; |z_OK| is similar; and |d_fixed| means it must stay
//! \\{down}.
//!
//! The four settings |yz_OK|, |y_OK|, |z_OK|, |d_fixed| would not need to
//! be distinguished from each other if we were simply solving the
//! digit-subscripting problem mentioned above. But in \TeX's case there is
//! a complication because of the nested structure of |push| and |pop|
//! commands. Suppose we add parentheses to the digit-subscripting problem,
//! redefining hits so that $\delta_y\ldots \delta_y$ is a hit if all $y$'s between
//! the $\delta$'s are enclosed in properly nested parentheses, and if the
//! parenthesis level of the right-hand $\delta_y$ is deeper than or equal to
//! that of the left-hand one. Thus, `(' and `)' correspond to `|push|'
//! and `|pop|'. Now if we want to assign a subscript to the final 1 in the
//! sequence
//! $$2_y\,7_d\,1_d\,(\,8_z\,2_y\,8_z\,)\,1$$
//! we cannot change the previous $1_d$ to $1_y$, since that would invalidate
//! the $2_y\ldots2_y$ hit. But we can change it to $1_z$, scoring a hit
//! since the intervening $8_z$'s are enclosed in parentheses.
//!
//! The program below removes movement nodes that are introduced after a |push|,
//! before it outputs the corresponding |pop|.
//
// @d y_here=1 {|info| when the movement entry points to a |y| command}
/// `info` when the movement entry points to a `y` command
pub(crate) const y_here: quarterword = 1;
// @d z_here=2 {|info| when the movement entry points to a |z| command}
/// `info` when the movement entry points to a `z` command
pub(crate) const z_here: quarterword = 2;
// @d yz_OK=3 {|info| corresponding to an unconstrained \\{down} command}
/// `info` corresponding to an unconstrained `down` command
pub(crate) const yz_OK: quarterword = 3;
// @d y_OK=4 {|info| corresponding to a \\{down} that can't become a |z|}
// @d z_OK=5 {|info| corresponding to a \\{down} that can't become a |y|}
// @d d_fixed=6 {|info| corresponding to a \\{down} that can't change}

use crate::section_0113::quarterword;
