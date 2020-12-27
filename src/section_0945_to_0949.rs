//! @ After |new_trie_op| has compressed the necessary opcode information,
//! plenty of information is available to unscramble the data into the
//! final form needed by our hyphenation algorithm.
//!
//! @<Sort \(t)the hyphenation op tables into proper order@>=
//! op_start[0]:=-min_quarterword;
//! for j:=1 to 255 do op_start[j]:=op_start[j-1]+qo(trie_used[j-1]);
//! for j:=1 to trie_op_ptr do
//!   trie_op_hash[j]:=op_start[trie_op_lang[j]]+trie_op_val[j]; {destination}
//! for j:=1 to trie_op_ptr do while trie_op_hash[j]>j do
//!   begin k:=trie_op_hash[j];@/
//!   t:=hyf_distance[k]; hyf_distance[k]:=hyf_distance[j]; hyf_distance[j]:=t;@/
//!   t:=hyf_num[k]; hyf_num[k]:=hyf_num[j]; hyf_num[j]:=t;@/
//!   t:=hyf_next[k]; hyf_next[k]:=hyf_next[j]; hyf_next[j]:=t;@/
//!   trie_op_hash[j]:=trie_op_hash[k]; trie_op_hash[k]:=k;
//!   end
//!
//! @ Before we forget how to initialize the data structures that have been
//! mentioned so far, let's write down the code that gets them started.
//!
//! @<Initialize table entries...@>=
//! for k:=-trie_op_size to trie_op_size do trie_op_hash[k]:=0;
//! for k:=0 to 255 do trie_used[k]:=min_quarterword;
//! trie_op_ptr:=0;
//!
//! @ The linked trie that is used to preprocess hyphenation patterns appears
//! in several global arrays. Each node represents an instruction of the form
//! ``if you see character |c|, then perform operation |o|, move to the
//! next character, and go to node |l|; otherwise go to node |r|.''
//! The four quantities |c|, |o|, |l|, and |r| are stored in four arrays
//! |trie_c|, |trie_o|, |trie_l|, and |trie_r|. The root of the trie
//! is |trie_l[0]|, and the number of nodes is |trie_ptr|. Null trie
//! pointers are represented by zero. To initialize the trie, we simply
//! set |trie_l[0]| and |trie_ptr| to zero. We also set |trie_c[0]| to some
//! arbitrary value, since the algorithm may access it.
//!
//! The algorithms maintain the condition
//! $$\hbox{|trie_c[trie_r[z]]>trie_c[z]|\qquad
//! whenever |z<>0| and |trie_r[z]<>0|};$$ in other words, sibling nodes are
//! ordered by their |c| fields.
//!
//! @d trie_root==trie_l[0] {root of the linked trie}
//!
//! @<Glob...@>=
//! @!init @!trie_c:packed array[trie_pointer] of packed_ASCII_code;
//!   {characters to match}
//! @t\hskip10pt@>@!trie_o:packed array[trie_pointer] of quarterword;
//!   {operations to perform}
//! @t\hskip10pt@>@!trie_l:packed array[trie_pointer] of trie_pointer;
//!   {left subtrie links}
//! @t\hskip10pt@>@!trie_r:packed array[trie_pointer] of trie_pointer;
//!   {right subtrie links}
//! @t\hskip10pt@>@!trie_ptr:trie_pointer; {the number of nodes in the trie}
//! @t\hskip10pt@>@!trie_hash:packed array[trie_pointer] of trie_pointer;
//!   {used to identify equivalent subtries}
//! tini
//!
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
