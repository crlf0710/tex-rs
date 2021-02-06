//! @ The fixing-up procedure is, of course, recursive. Since the linked trie
//! usually has overlapping subtries, the same data may be moved several
//! times; but that causes no harm, and at most as much work is done as it
//! took to build the uncompressed trie.
//! @^recursion@>
//!
//! @<Declare procedures for preprocessing hyph...@>=
//! procedure trie_fix(@!p:trie_pointer); {moves |p| and its siblings into |trie|}
//! var q:trie_pointer; {a local variable that need not be saved on recursive calls}
//! @!c:ASCII_code; {another one that need not be saved}
//! @!z:trie_pointer; {|trie| reference; this local variable must be saved}
//! begin z:=trie_ref[p];
//! repeat q:=trie_l[p]; c:=so(trie_c[p]);
//! trie_link(z+c):=trie_ref[q]; trie_char(z+c):=qi(c); trie_op(z+c):=trie_o[p];
//! if q>0 then trie_fix(q);
//! p:=trie_r[p];
//! until p=0;
//! end;
//!
