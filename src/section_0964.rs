//! @ @<Insert a new trie node between |q| and |p|...@>=
//! begin if trie_ptr=trie_size then overflow("pattern memory",trie_size);
//! @:TeX capacity exceeded pattern memory}{\quad pattern memory@>
//! incr(trie_ptr); trie_r[trie_ptr]:=p; p:=trie_ptr; trie_l[p]:=0;
//! if first_child then trie_l[q]:=p@+else trie_r[q]:=p;
//! trie_c[p]:=si(c); trie_o[p]:=min_quarterword;
//! end
//!
