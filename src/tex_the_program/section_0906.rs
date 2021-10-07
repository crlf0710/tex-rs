//! ` `

use super::section_0907::ASCII_code_or_non_char;

// @<Declare the function called |reconstitute|@>=
// function reconstitute(@!j,@!n:small_number;@!bchar,@!hchar:halfword):
//   small_number;
#[allow(unused)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip(globals)))]
pub(crate) fn reconstitute(
    globals: &mut TeXGlobals,
    mut j: small_number,
    n: small_number,
    mut bchar: ASCII_code_or_non_char,
    mut hchar: ASCII_code_or_non_char,
) -> TeXResult<small_number> {
    // label continue,done;
    // var @!p:pointer; {temporary register for list manipulation}
    // @!t:pointer; {a node being appended to}
    /// a node being appended to
    let mut t: pointer;
    // @!q:four_quarters; {character information or a lig/kern instruction}
    // @!cur_rh:halfword; {hyphen character for ligature testing}
    /// hyphen character for ligature testing
    let mut cur_rh;
    // @!test_char:halfword; {hyphen or other character for ligature testing}
    /// hyphen or other character for ligature testing
    let mut test_char;
    // @!w:scaled; {amount of kerning}
    /// amount of kerning
    let mut w: scaled;
    // @!k:font_index; {position of current lig/kern instruction}
    // begin hyphen_passed:=0; t:=hold_head; w:=0; link(hold_head):=null;
    globals.hyphen_passed = 0.into();
    t = hold_head;
    w = scaled::zero();
    link!(globals, hold_head) = null;
    //  {at this point |ligature_present=lft_hit=rt_hit=false|}
    /// at this point `ligature_present=lft_hit=rt_hit=false`
    const _: () = ();
    // @<Set up data structures with the cursor following position |j|@>;
    crate::section_0908::Set_up_data_structures_with_the_cursor_following_position_j!(
        globals, j, n, t, bchar, hchar, cur_rh
    );
    // continue:@<If there's a ligature or kern at the cursor position, update the data
    //   structures, possibly advancing~|j|; continue until the cursor moves@>;
    crate::region_backward_label! {
        'continue_ <-
        {
            crate::section_0909::If_there_s_a_ligature_or_kern_at_the_cursor_position__update_the_data_structures__possibly_advancing_j__continue_until_the_cursor_moves!(
                globals, j, n, t, bchar, hchar, cur_rh, test_char, w, 'continue_
            );
            // @<Append a ligature and/or kern to the translation;
            //   |goto continue| if the stack of inserted ligatures is nonempty@>;
            crate::section_0910::Append_a_ligature_and_or_kern_to_the_translation__goto_continue_if_the_stack_of_inserted_ligatures_is_nonempty!(
                globals, t, w, j, n, bchar, hchar, cur_rh, 'continue_
            );
        }
        |'continue_|
    };

    // reconstitute:=j;
    crate::ok_nojump!(j)
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0101::small_number;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0118::link;
use crate::section_0162::hold_head;
