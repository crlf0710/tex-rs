//! ` `
//! The processing is facilitated by a subroutine called |reconstitute|. Given
//! a string of characters $x_j\ldots x_n$, there is a smallest index $m\ge j$
//! such that the ``translation'' of $x_j\ldots x_n$ by ligatures and kerning
//! has the form $y_1\ldots y_t$ followed by the translation of $x_{m+1}\ldots x_n$,
//! where $y_1\ldots y_t$ is some nonempty sequence of character, ligature, and
//! kern nodes. We call $x_j\ldots x_m$ a ``cut prefix'' of $x_j\ldots x_n$.
//! For example, if $x_1x_2x_3=\.{fly}$, and if the font contains `fl' as a
//! ligature and a kern between `fl' and `y', then $m=2$, $t=2$, and $y_1$ will
//! be a ligature node for `fl' followed by an appropriate kern node~$y_2$.
//! In the most common case, $x_j$~forms no ligature with $x_{j+1}$ and we
//! simply have $m=j$, $y_1=x_j$. If $m<n$ we can repeat the procedure on
//! $x_{m+1}\ldots x_n$ until the entire translation has been found.
//!
//! The |reconstitute| function returns the integer $m$ and puts the nodes
//! $y_1\ldots y_t$ into a linked list starting at |link(hold_head)|,
//! getting the input $x_j\ldots x_n$ from the |hu| array. If $x_j=256$,
//! we consider $x_j$ to be an implicit left boundary character; in this
//! case |j| must be strictly less than~|n|. There is a
//! parameter |bchar|, which is either 256 or an implicit right boundary character
//! assumed to be present just following~$x_n$. (The value |hu[n+1]| is never
//! explicitly examined, but the algorithm imagines that |bchar| is there.)
//!
//! If there exists an index |k| in the range $j\le k\le m$ such that |hyf[k]|
//! is odd and such that the result of |reconstitute| would have been different
//! if $x_{k+1}$ had been |hchar|, then |reconstitute| sets |hyphen_passed|
//! to the smallest such~|k|. Otherwise it sets |hyphen_passed| to zero.
//!
//! A special convention is used in the case |j=0|: Then we assume that the
//! translation of |hu[0]| appears in a special list of charnodes starting at
//! |init_list|; moreover, if |init_lig| is |true|, then |hu[0]| will be
//! a ligature character, involving a left boundary if |init_lft| is |true|.
//! This facility is provided for cases when a hyphenated
//! word is preceded by punctuation (like single or double quotes) that might
//! affect the translation of the beginning of the word.
//
// @<Glob...@>=
// @!hyphen_passed:small_number; {first hyphen in a ligature, if any}
/// first hyphen in a ligature, if any
#[globals_struct_field(TeXGlobals)]
pub(crate) static hyphen_passed: small_number = small_number::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0101::small_number;

use globals_struct::{globals_struct_field, globals_struct_use};
