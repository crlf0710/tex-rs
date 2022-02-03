//! ` `
//!
//! Here is the overall plan of |mlist_to_hlist|, and the list of its
//! local variables.
//
// @d done_with_noad=80 {go here when a noad has been fully translated}
// @d done_with_node=81 {go here when a node has been fully converted}
// @d check_dimensions=82 {go here to update |max_h| and |max_d|}
// @d delete_q=83 {go here to delete |q| and move to the next node}
//
// @p@t\4@>@<Declare math construction procedures@>
// procedure mlist_to_hlist;
#[allow(unused_variables, unused_assignments, unused_mut)]
pub(crate) fn mlist_to_hlist(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label reswitch, check_dimensions, done_with_noad, done_with_node, delete_q,
    //   done;
    // var mlist:pointer; {beginning of the given list}
    /// beginning of the given list
    let mlist;
    // @!penalties:boolean; {should penalty nodes be inserted?}
    /// should penalty nodes be inserted?
    let penalties;
    // @!style:small_number; {the given style}
    /// the given style
    let style;
    // @!save_style:small_number; {holds |cur_style| during recursion}
    // @!q:pointer; {runs through the mlist}
    /// runs through the mlist
    let mut q;
    // @!r:pointer; {the most recent noad preceding |q|}
    /// the most recent noad preceding `q`
    let mut r;
    // @!r_type:small_number; {the |type| of noad |r|, or |op_noad| if |r=null|}
    /// the `type` of noad `r`, or `op_noad` if `r=null`
    let mut r_type;
    // @!t:small_number; {the effective |type| of noad |q| during the second pass}
    // @!p,@!x,@!y,@!z: pointer; {temporary registers for list construction}
    // @!pen:integer; {a penalty to be inserted}
    // @!s:small_number; {the size of a noad to be deleted}
    // @!max_h,@!max_d:scaled; {maximum height and depth of the list translated so far}
    /// maximum height and depth of the list translated so far
    let (mut max_h, mut max_d);
    // @!delta:scaled; {offset between subscript and superscript}
    /// offset between subscript and superscript
    let mut delta: scaled;
    // begin mlist:=cur_mlist; penalties:=mlist_penalties;
    mlist = globals.cur_mlist;
    penalties = globals.mlist_penalties;
    // style:=cur_style; {tuck global parameters away as local variables}
    style = globals.cur_style;
    /// tuck global parameters away as local variables
    const _: () = ();
    // q:=mlist; r:=null; r_type:=op_noad; max_h:=0; max_d:=0;
    q = mlist;
    r = null;
    r_type = op_noad;
    max_h = scaled::zero();
    max_d = scaled::zero();
    // @<Set up the values of |cur_size| and |cur_mu|, based on |cur_style|@>;
    crate::section_0703::Set_up_the_values_of_cur_size_and_cur_mu__based_on_cur_style!(globals);
    // while q<>null do @<Process node-or-noad |q| as much as possible in preparation
    //     for the second pass of |mlist_to_hlist|, then move to the next
    //     item in the mlist@>;
    while q != null {
        crate::section_0727::Process_node_or_noad_q_as_much_as_possible_in_preparation_for_the_second_pass_of_mlist_to_hlist__then_move_to_the_next_item_in_the_mlist!(
            globals, q, r, r_type, max_h, max_d, delta
        );
    }
    // @<Convert \(a)a final |bin_noad| to an |ord_noad|@>;
    crate::section_0729::Convert_a_final_bin_noad_to_an_ord_noad!(globals, r, r_type);
    // @<Make a second pass over the mlist, removing all noads and inserting the
    //   proper spacing and penalties@>;
    crate::section_0760::Make_a_second_pass_over_the_mlist__removing_all_noads_and_inserting_the_proper_spacing_and_penalties!(
        globals, mlist, style, penalties, max_h, max_d
    );
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0115::null;
use crate::section_0682::op_noad;
