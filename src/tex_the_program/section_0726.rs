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
pub(crate) fn mlist_to_hlist(_globals: &mut TeXGlobals) {
    // label reswitch, check_dimensions, done_with_noad, done_with_node, delete_q,
    //   done;
    // var mlist:pointer; {beginning of the given list}
    // @!penalties:boolean; {should penalty nodes be inserted?}
    // @!style:small_number; {the given style}
    // @!save_style:small_number; {holds |cur_style| during recursion}
    // @!q:pointer; {runs through the mlist}
    // @!r:pointer; {the most recent noad preceding |q|}
    // @!r_type:small_number; {the |type| of noad |r|, or |op_noad| if |r=null|}
    // @!t:small_number; {the effective |type| of noad |q| during the second pass}
    // @!p,@!x,@!y,@!z: pointer; {temporary registers for list construction}
    // @!pen:integer; {a penalty to be inserted}
    // @!s:small_number; {the size of a noad to be deleted}
    // @!max_h,@!max_d:scaled; {maximum height and depth of the list translated so far}
    // @!delta:scaled; {offset between subscript and superscript}
    // begin mlist:=cur_mlist; penalties:=mlist_penalties;
    // style:=cur_style; {tuck global parameters away as local variables}
    // q:=mlist; r:=null; r_type:=op_noad; max_h:=0; max_d:=0;
    // @<Set up the values of |cur_size| and |cur_mu|, based on |cur_style|@>;
    // while q<>null do @<Process node-or-noad |q| as much as possible in preparation
    //     for the second pass of |mlist_to_hlist|, then move to the next
    //     item in the mlist@>;
    // @<Convert \(a)a final |bin_noad| to an |ord_noad|@>;
    // @<Make a second pass over the mlist, removing all noads and inserting the
    //   proper spacing and penalties@>;
    // end;
    todo!();
}

use crate::section_0004::TeXGlobals;
