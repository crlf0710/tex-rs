//! ` `
// @<Declare subprocedures for |line_break|@>=
// @t\4@>@<Declare the function called |reconstitute|@>
// procedure hyphenate;
#[allow(unused_variables, unused_assignments)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip(globals)))]
pub(crate) fn hyphenate(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label common_ending,done,found,found1,found2,not_found,exit;
    // var @<Local variables for hyphenation@>@;
    // begin @<Find hyphen locations for the word in |hc|, or |return|@>;
    crate::section_0923::Find_hyphen_locations_for_the_word_in_hc__or_return!(globals);
    // @<If no hyphens were found, |return|@>;
    crate::section_0902::If_no_hyphens_were_found__return!(globals);
    // @<Replace nodes |ha..hb| by a sequence of nodes that includes
    //   the discretionary hyphens@>;
    crate::section_0903::Replace_nodes_ha_to_hb_by_a_sequence_of_nodes_that_includes_the_discretionary_hyphens!(
        globals
    );
    // exit:end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
