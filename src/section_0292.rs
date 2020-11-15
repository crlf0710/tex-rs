//! @ The procedure |show_token_list|, which prints a symbolic form of
//! the token list that starts at a given node |p|, illustrates these
//! conventions. The token list being displayed should not begin with a reference
//! count. However, the procedure is intended to be robust, so that if the
//! memory links are awry or if |p| is not really a pointer to a token list,
//! nothing catastrophic will happen.
//!
//! An additional parameter |q| is also given; this parameter is either null
//! or it points to a node in the token list where a certain magic computation
//! takes place that will be explained later. (Basically, |q| is non-null when
//! we are printing the two-line context information at the time of an error
//! message; |q| marks the place corresponding to where the second line
//! should begin.)
//!
//! For example, if |p| points to the node containing the first \.a in the
//! token list above, then |show_token_list| will print the string
//! $$\hbox{`\.{a\#1\#2\ \\b\ ->\#1\\-a\ \#\#1\#2\ \#2}';}$$
//! and if |q| points to the node containing the second \.a,
//! the magic computation will be performed just before the second \.a is printed.
//!
//! The generation will stop, and `\.{\\ETC.}' will be printed, if the length
//! of printing exceeds a given limit~|l|. Anomalous entries are printed in the
//! form of control sequences that are not followed by a blank space, e.g.,
//! `\.{\\BAD.}'; this cannot be confused with actual control sequences because
//! a real control sequence named \.{BAD} would come out `\.{\\BAD\ }'.
//
// @<Declare the procedure called |show_token_list|@>=
// procedure show_token_list(@!p,@!q:integer;@!l:integer);
#[allow(unused_variables)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
pub(crate) fn show_token_list(globals: &mut TeXGlobals, mut p: integer, q: integer, l:integer) {
    // label exit;
    // var m,@!c:integer; {pieces of a token}
    // @!match_chr:ASCII_code; {character used in a `|match|'}
    /// character used in a `match`
    let mut match_chr: ASCII_code;
    // @!n:ASCII_code; {the highest parameter number, as an ASCII digit}
    /// the highest parameter number, as an ASCII digit
    let mut n: ASCII_code;
    // begin match_chr:="#"; n:="0"; tally:=0;
    match_chr = ASCII_code_literal!(b'#');
    n = ASCII_code_literal!(b'0');
    globals.tally = 0;
    // while (p<>null) and (tally<l) do
    while p != null as _ && globals.tally < l {
        // begin if p=q then @<Do magic computation@>;
        if p == q {
            todo!();
        }
        // @<Display token |p|, and |return| if there are problems@>;
        Display_token_p__and_return_if_there_are_problems!(globals, p, n, match_chr);
        // p:=link(p);
        p = link!(globals, p as pointer) as _;
        // end;
    }
    // if p<>null then print_esc("ETC.");
    if p != null as _ {
        print_esc(globals, strpool_str!("ETC."));
    }
    // @.ETC@>
    // exit:
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0018::ASCII_code;
use crate::section_0063::print_esc;
use crate::section_0115::null;
use crate::section_0115::pointer;