//! @ Here is the subroutine that searches the hash table for an identifier
//! that matches a given string of length |l>1| appearing in |buffer[j..
//! (j+l-1)]|. If the identifier is found, the corresponding hash table address
//! is returned. Otherwise, if the global variable |no_new_control_sequence|
//! is |true|, the dummy address |undefined_control_sequence| is returned.
//! Otherwise the identifier is inserted into the hash table and its location
//! is returned.
//
// @p function id_lookup(@!j,@!l:integer):pointer; {search the hash table}
/// search the hash table
#[allow(unused_variables)]
pub(crate) fn id_lookup(globals: &mut TeXGlobals, j: integer, l: integer) -> pointer {
    // label found; {go here if you found it}
    // var h:integer; {hash code}
    // @!d:integer; {number of characters in incomplete current string}
    // @!p:pointer; {index in |hash| array}
    // @!k:pointer; {index in |buffer| array}
    // begin @<Compute the hash code |h|@>;
    // p:=h+hash_base; {we start searching here; note that |0<=h<hash_prime|}
    // loop@+begin if text(p)>0 then if length(text(p))=l then
    //     if str_eq_buf(text(p),j) then goto found;
    //   if next(p)=0 then
    //     begin if no_new_control_sequence then
    //       p:=undefined_control_sequence
    //     else @<Insert a new control sequence after |p|, then make
    //       |p| point to it@>;
    //     goto found;
    //     end;
    //   p:=next(p);
    //   end;
    // found: id_lookup:=p;
    // end;
    todo!();
}

use crate::section_0004::TeXGlobals;
use crate::pascal::integer;
use crate::section_0115::pointer;