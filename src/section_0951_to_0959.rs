//! @ Each time \.{\\patterns} appears, it contributes further patterns to
//! the future trie, which will be built only when hyphenation is attempted or
//! when a format file is dumped. The boolean variable |trie_not_ready|
//! will change to |false| when the trie is compressed; this will disable
//! further patterns.
//!
//! @<Initialize table entries...@>=
//! trie_not_ready:=true; trie_root:=0; trie_c[0]:=si(0); trie_ptr:=0;
//!
//! @ Here is how the trie-compression data structures are initialized.
//! If storage is tight, it would be possible to overlap |trie_op_hash|,
//! |trie_op_lang|, and |trie_op_val| with |trie|, |trie_hash|, and |trie_taken|,
//! because we finish with the former just before we need the latter.
//!
//! @<Get ready to compress the trie@>=
//! @<Sort \(t)the hyphenation...@>;
//! for p:=0 to trie_size do trie_hash[p]:=0;
//! trie_root:=compress_trie(trie_root); {identify equivalent subtries}
//! for p:=0 to trie_ptr do trie_ref[p]:=0;
//! for p:=0 to 255 do trie_min[p]:=p+1;
//! trie_link(0):=1; trie_max:=0
//!
//! @ The |first_fit| procedure finds the smallest hole |z| in |trie| such that
//! a trie family starting at a given node |p| will fit into vacant positions
//! starting at |z|. If |c=trie_c[p]|, this means that location |z-c| must
//! not already be taken by some other family, and that |z-c+@t$c^\prime$@>|
//! must be vacant for all characters $c^\prime$ in the family. The procedure
//! sets |trie_ref[p]| to |z-c| when the first fit has been found.
//!
//! @<Declare procedures for preprocessing hyph...@>=
//! procedure first_fit(@!p:trie_pointer); {packs a family into |trie|}
//! label not_found,found;
//! var h:trie_pointer; {candidate for |trie_ref[p]|}
//! @!z:trie_pointer; {runs through holes}
//! @!q:trie_pointer; {runs through the family starting at |p|}
//! @!c:ASCII_code; {smallest character in the family}
//! @!l,@!r:trie_pointer; {left and right neighbors}
//! @!ll:1..256; {upper limit of |trie_min| updating}
//! begin c:=so(trie_c[p]);
//! z:=trie_min[c]; {get the first conceivably good hole}
//! loop@+  begin h:=z-c;@/
//!   @<Ensure that |trie_max>=h+256|@>;
//!   if trie_taken[h] then goto not_found;
//!   @<If all characters of the family fit relative to |h|, then
//!     |goto found|,\30\ otherwise |goto not_found|@>;
//!   not_found: z:=trie_link(z); {move to the next hole}
//!   end;
//! found: @<Pack the family into |trie| relative to |h|@>;
//! end;
//!
//! @ By making sure that |trie_max| is at least |h+256|, we can be sure that
//! |trie_max>z|, since |h=z-c|. It follows that location |trie_max| will
//! never be occupied in |trie|, and we will have |trie_max>=trie_link(z)|.
//!
//! @<Ensure that |trie_max>=h+256|@>=
//! if trie_max<h+256 then
//!   begin if trie_size<=h+256 then overflow("pattern memory",trie_size);
//! @:TeX capacity exceeded pattern memory}{\quad pattern memory@>
//!   repeat incr(trie_max); trie_taken[trie_max]:=false;
//!   trie_link(trie_max):=trie_max+1; trie_back(trie_max):=trie_max-1;
//!   until trie_max=h+256;
//!   end
//!
//! @ @<If all characters of the family fit relative to |h|...@>=
//! q:=trie_r[p];
//! while q>0 do
//!   begin if trie_link(h+so(trie_c[q]))=0 then goto not_found;
//!   q:=trie_r[q];
//!   end;
//! goto found
//!
//! @ @<Pack the family into |trie| relative to |h|@>=
//! trie_taken[h]:=true; trie_ref[p]:=h; q:=p;
//! repeat z:=h+so(trie_c[q]); l:=trie_back(z); r:=trie_link(z);
//! trie_back(r):=l; trie_link(l):=r; trie_link(z):=0;
//! if l<256 then
//!   begin if z<256 then ll:=z @+else ll:=256;
//!   repeat trie_min[l]:=r; incr(l);
//!   until l=ll;
//!   end;
//! q:=trie_r[q];
//! until q=0
//!
//! @ To pack the entire linked trie, we use the following recursive procedure.
//! @^recursion@>
//!
//! @<Declare procedures for preprocessing hyph...@>=
//! procedure trie_pack(@!p:trie_pointer); {pack subtries of a family}
//! var q:trie_pointer; {a local variable that need not be saved on recursive calls}
//! begin repeat q:=trie_l[p];
//! if (q>0)and(trie_ref[q]=0) then
//!   begin first_fit(q); trie_pack(q);
//!   end;
//! p:=trie_r[p];
//! until p=0;
//! end;
//!
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
