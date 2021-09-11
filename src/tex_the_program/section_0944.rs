//! @ It's tempting to remove the |overflow| stops in the following procedure;
//! |new_trie_op| could return |min_quarterword| (thereby simply ignoring
//! part of a hyphenation pattern) instead of aborting the job. However, that would
//! lead to different hyphenation results on different installations of \TeX\
//! using the same patterns. The |overflow| stops are necessary for portability
//! of patterns.
//
// @<Declare procedures for preprocessing hyph...@>=
// function new_trie_op(@!d,@!n:small_number;@!v:quarterword):quarterword;
pub(crate) fn new_trie_op(
    globals: &mut TeXGlobals,
    d: small_number,
    n: small_number,
    v: quarterword,
) -> quarterword {
    // label exit;
    // var h:-trie_op_size..trie_op_size; {trial hash location}
    /// trial hash location
    let mut h: i16_from_m_to_n<trie_op_size_NEG_TYPENUM, trie_op_size_POS_TYPENUM>;
    // @!u:quarterword; {trial op code}
    /// trial op code
    let mut u: quarterword;
    // @!l:0..trie_op_size; {pointer to stored data}
    /// pointer to stored data
    let mut l: u16_from_0_to_n<trie_op_size_TYPENUM>;
    // begin h:=abs(n+313*d+361*v+1009*cur_lang) mod (trie_op_size+trie_op_size)
    //   - trie_op_size;
    h = i16_from_m_to_n::new(
        ((n.get() as integer
            + 313 * d.get() as integer
            + 361 * v as integer
            + 1009 * globals.cur_lang.numeric_value() as integer)
            % (trie_op_size + trie_op_size) as integer
            - trie_op_size as integer) as _,
    );
    // loop@+  begin l:=trie_op_hash[h];
    loop {
        l = globals.trie_op_hash[h];
        // if l=0 then {empty position found for a new op}
        if l == 0 {
            /// empty position found for a new op
            const _: () = ();
            // begin if trie_op_ptr=trie_op_size then
            if globals.trie_op_ptr == trie_op_size {
                todo!("overflow");
                // overflow("pattern memory ops",trie_op_size);
            }
            // u:=trie_used[cur_lang];
            u = trie_used!(globals, globals.cur_lang);
            // if u=max_quarterword then
            if u == max_quarterword {
                todo!("overflow");
                // overflow("pattern memory ops per language",
                //   max_quarterword-min_quarterword);
            }
            // incr(trie_op_ptr); incr(u); trie_used[cur_lang]:=u;
            incr!(globals.trie_op_ptr);
            incr!(u);
            trie_used_assign!(globals, globals.cur_lang, u);
            // hyf_distance[trie_op_ptr]:=d;
            globals.hyf_distance[globals.trie_op_ptr.get()] = d;
            // hyf_num[trie_op_ptr]:=n; hyf_next[trie_op_ptr]:=v;
            globals.hyf_num[globals.trie_op_ptr.get()] = n;
            globals.hyf_next[globals.trie_op_ptr.get()] = v;
            // trie_op_lang[trie_op_ptr]:=cur_lang; trie_op_hash[h]:=trie_op_ptr;
            globals.trie_op_lang[globals.trie_op_ptr.get()] = globals.cur_lang;
            globals.trie_op_hash[h] = globals.trie_op_ptr;
            // trie_op_val[trie_op_ptr]:=u; new_trie_op:=u; return;
            globals.trie_op_val[globals.trie_op_ptr.get()] = u;
            return u;
            // end;
        }
        // if (hyf_distance[l]=d)and(hyf_num[l]=n)and(hyf_next[l]=v)
        //  and(trie_op_lang[l]=cur_lang) then
        if globals.hyf_distance[l.get()] == d
            && globals.hyf_num[l.get()] == n
            && globals.hyf_next[l.get()] == v
            && globals.trie_op_lang[l.get()] == globals.cur_lang
        {
            // begin new_trie_op:=trie_op_val[l]; return;
            return globals.trie_op_val[l.get()];
            // end;
        }
        // if h>-trie_op_size then decr(h)@+else h:=trie_op_size;
        if h > trie_op_size_NEG_TYPENUM::I16 {
            decr!(h);
        } else {
            h = trie_op_size_POS_TYPENUM::I16.into();
        }
        // end;
    }
    // exit:end;
}

use crate::pascal::i16_from_m_to_n;
use crate::pascal::integer;
use crate::pascal::u16_from_0_to_n;
use crate::section_0004::TeXGlobals;
use crate::section_0011::trie_op_size;
use crate::section_0011::trie_op_size_NEG_TYPENUM;
use crate::section_0011::trie_op_size_POS_TYPENUM;
use crate::section_0011::trie_op_size_TYPENUM;
use crate::section_0016::decr;
use crate::section_0016::incr;
use crate::section_0101::small_number;
use crate::section_0110::max_quarterword;
use crate::section_0113::quarterword;
use crate::section_0943::trie_used;
use crate::section_0943::trie_used_assign;
use typenum::Integer;
