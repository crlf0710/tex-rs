//! @ Let us suppose that a linked trie has already been constructed.
//! Experience shows that we can often reduce its size by recognizing common
//! subtries; therefore another hash table is introduced for this purpose,
//! somewhat similar to |trie_op_hash|. The new hash table will be
//! initialized to zero.
//!
//! The function |trie_node(p)| returns |p| if |p| is distinct from other nodes
//! that it has seen, otherwise it returns the number of the first equivalent
//! node that it has seen.
//!
//! Notice that we might make subtries equivalent even if they correspond to
//! patterns for different languages, in which the trie ops might mean quite
//! different things. That's perfectly all right.
//!
//! @<Declare procedures for preprocessing hyph...@>=
//! function trie_node(@!p:trie_pointer):trie_pointer; {converts
//!   to a canonical form}
//! label exit;
//! var h:trie_pointer; {trial hash location}
//! @!q:trie_pointer; {trial trie node}
//! begin h:=abs(trie_c[p]+1009*trie_o[p]+@|
//!     2718*trie_l[p]+3142*trie_r[p]) mod trie_size;
//! loop@+  begin q:=trie_hash[h];
//!   if q=0 then
//!     begin trie_hash[h]:=p; trie_node:=p; return;
//!     end;
//!   if (trie_c[q]=trie_c[p])and(trie_o[q]=trie_o[p])and@|
//!     (trie_l[q]=trie_l[p])and(trie_r[q]=trie_r[p]) then
//!     begin trie_node:=q; return;
//!     end;
//!   if h>0 then decr(h)@+else h:=trie_size;
//!   end;
//! exit:end;
//!
//! @ A neat recursive procedure is now able to compress a trie by
//! traversing it and applying |trie_node| to its nodes in ``bottom up''
//! fashion. We will compress the entire trie by clearing |trie_hash| to
//! zero and then saying `|trie_root:=compress_trie(trie_root)|'.
//! @^recursion@>
//!
//! @<Declare procedures for preprocessing hyph...@>=
//! function compress_trie(@!p:trie_pointer):trie_pointer;
//! begin if p=0 then compress_trie:=0
//! else  begin trie_l[p]:=compress_trie(trie_l[p]);
//!   trie_r[p]:=compress_trie(trie_r[p]);
//!   compress_trie:=trie_node(p);
//!   end;
//! end;
//!
