//! @ When the whole trie has been allocated into the sequential table, we
//! must go through it once again so that |trie| contains the correct
//! information. Null pointers in the linked trie will be represented by the
//! value~0, which properly implements an ``empty'' family.
//!
//! @<Move the data into |trie|@>=
//! h.rh:=0; h.b0:=min_quarterword; h.b1:=min_quarterword; {|trie_link:=0|,
//!   |trie_op:=min_quarterword|, |trie_char:=qi(0)|}
//! if trie_root=0 then {no patterns were given}
//!   begin for r:=0 to 256 do trie[r]:=h;
//!   trie_max:=256;
//!   end
//! else begin trie_fix(trie_root); {this fixes the non-holes in |trie|}
//!   r:=0; {now we will zero out all the holes}
//!   repeat s:=trie_link(r); trie[r]:=h; r:=s;
//!   until r>trie_max;
//!   end;
//! trie_char(0):=qi("?"); {make |trie_char(c)<>c| for all |c|}
//!
