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
#[allow(unused_variables, unreachable_code, unused_mut, unused_assignments)]
pub(crate) fn id_lookup(globals: &mut TeXGlobals, j: integer, l: integer) -> pointer {
    // label found; {go here if you found it}
    // var h:integer; {hash code}
    /// hash code
    let mut h: integer;
    // @!d:integer; {number of characters in incomplete current string}
    // @!p:pointer; {index in |hash| array}
    /// index in `hash` array
    let mut p: pointer;
    // @!k:pointer; {index in |buffer| array}
    // begin @<Compute the hash code |h|@>;
    Compute_the_hash_code_h!(globals, h, j, l);
    trace_expr!("hash = {}", h);
    // p:=h+hash_base; {we start searching here; note that |0<=h<hash_prime|}
    /// we start searching here; note that `0<=h<hash_prime`
    {
        p = (h as i32 + hash_base as i32) as pointer;
        trace_expr!("initial_p = {}", p);
    }
    region_forward_label!(
    |'found|
    {
        // loop@+begin if text(p)>0 then if length(text(p))=l then
        //     if str_eq_buf(text(p),j) then goto found;
        loop {
            if text!(globals, p) > 0 {
                if length(globals, text!(globals, p) as _) == 1 {
                    if str_eq_buf(globals, str_number::new(text!(globals, p) as _), j) {
                        goto_forward_label!('found);
                    }
                }
            }
            // if next(p)=0 then
            if next!(globals, p) == 0 {
                // begin if no_new_control_sequence then
                if globals.no_new_control_sequence  {
                    // p:=undefined_control_sequence
                    p = undefined_control_sequence;
                } else {
                    // else @<Insert a new control sequence after |p|, then make
                    //   |p| point to it@>;
                    todo!();
                }
                // goto found;
                goto_forward_label!('found);
                // end;
            }
            // p:=next(p);
            p = next!(globals, p);
            trace_expr!("new_p = {}", p);
            // end;
        }
    }
    // found: id_lookup:=p;
    'found <-
    );
    return p;
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0040::length;
use crate::section_0045::str_eq_buf;
use crate::section_0115::pointer;
use crate::section_0038::str_number;
use crate::section_0222::hash_base;
use crate::section_0222::undefined_control_sequence;
