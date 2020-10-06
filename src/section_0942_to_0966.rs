//! @* \[43] Initializing the hyphenation tables.
//! The trie for \TeX's hyphenation algorithm is built from a sequence of
//! patterns following a \.{\\patterns} specification. Such a specification
//! is allowed only in \.{INITEX}, since the extra memory for auxiliary tables
//! and for the initialization program itself would only clutter up the
//! production version of \TeX\ with a lot of deadwood.
//!
//! The first step is to build a trie that is linked, instead of packed
//! into sequential storage, so that insertions are readily made.
//! After all patterns have been processed, \.{INITEX}
//! compresses the linked trie by identifying common subtries. Finally the
//! trie is packed into the efficient sequential form that the hyphenation
//! algorithm actually uses.
//!
//! @<Declare subprocedures for |line_break|@>=
//! @!init @<Declare procedures for preprocessing hyphenation patterns@>@;
//! tini
//!
//! @ Before we discuss trie building in detail, let's consider the simpler
//! problem of creating the |hyf_distance|, |hyf_num|, and |hyf_next| arrays.
//!
//! Suppose, for example, that \TeX\ reads the pattern `\.{ab2cde1}'. This is
//! a pattern of length 5, with $n_0\ldots n_5=0\,0\,2\,0\,0\,1$ in the
//! notation above. We want the corresponding |trie_op| code |v| to have
//! |hyf_distance[v]=3|, |hyf_num[v]=2|, and |hyf_next[v]=@t$v^\prime$@>|,
//! where the auxiliary |trie_op| code $v^\prime$ has
//! |hyf_distance[@t$v^\prime$@>]=0|, |hyf_num[@t$v^\prime$@>]=1|, and
//! |hyf_next[@t$v^\prime$@>]=min_quarterword|.
//!
//! \TeX\ computes an appropriate value |v| with the |new_trie_op| subroutine
//! below, by setting
//! $$\hbox{|@t$v^\prime$@>:=new_trie_op(0,1,min_quarterword)|,\qquad
//! |v:=new_trie_op(3,2,@t$v^\prime$@>)|.}$$
//! This subroutine looks up its three
//! parameters in a special hash table, assigning a new value only if these
//! three have not appeared before for the current language.
//!
//! The hash table is called |trie_op_hash|, and the number of entries it contains
//! is |trie_op_ptr|.
//!
//! @<Glob...@>=
//! @!init@! trie_op_hash:array[-trie_op_size..trie_op_size] of 0..trie_op_size;
//!   {trie op codes for quadruples}
//! @!trie_used:array[ASCII_code] of quarterword;
//!   {largest opcode used so far for this language}
//! @!trie_op_lang:array[1..trie_op_size] of ASCII_code;
//!   {language part of a hashed quadruple}
//! @!trie_op_val:array[1..trie_op_size] of quarterword;
//!   {opcode corresponding to a hashed quadruple}
//! @!trie_op_ptr:0..trie_op_size; {number of stored ops so far}
//! tini
//!
//! @ It's tempting to remove the |overflow| stops in the following procedure;
//! |new_trie_op| could return |min_quarterword| (thereby simply ignoring
//! part of a hyphenation pattern) instead of aborting the job. However, that would
//! lead to different hyphenation results on different installations of \TeX\
//! using the same patterns. The |overflow| stops are necessary for portability
//! of patterns.
//!
//! @<Declare procedures for preprocessing hyph...@>=
//! function new_trie_op(@!d,@!n:small_number;@!v:quarterword):quarterword;
//! label exit;
//! var h:-trie_op_size..trie_op_size; {trial hash location}
//! @!u:quarterword; {trial op code}
//! @!l:0..trie_op_size; {pointer to stored data}
//! begin h:=abs(n+313*d+361*v+1009*cur_lang) mod (trie_op_size+trie_op_size)
//!   - trie_op_size;
//! loop@+  begin l:=trie_op_hash[h];
//!   if l=0 then {empty position found for a new op}
//!     begin if trie_op_ptr=trie_op_size then
//!       overflow("pattern memory ops",trie_op_size);
//!     u:=trie_used[cur_lang];
//!     if u=max_quarterword then
//!       overflow("pattern memory ops per language",
//!         max_quarterword-min_quarterword);
//!     incr(trie_op_ptr); incr(u); trie_used[cur_lang]:=u;
//!     hyf_distance[trie_op_ptr]:=d;
//!     hyf_num[trie_op_ptr]:=n; hyf_next[trie_op_ptr]:=v;
//!     trie_op_lang[trie_op_ptr]:=cur_lang; trie_op_hash[h]:=trie_op_ptr;
//!     trie_op_val[trie_op_ptr]:=u; new_trie_op:=u; return;
//!     end;
//!   if (hyf_distance[l]=d)and(hyf_num[l]=n)and(hyf_next[l]=v)
//!    and(trie_op_lang[l]=cur_lang) then
//!     begin new_trie_op:=trie_op_val[l]; return;
//!     end;
//!   if h>-trie_op_size then decr(h)@+else h:=trie_op_size;
//!   end;
//! exit:end;
//!
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
//! @ The compressed trie will be packed into the |trie| array using a
//! ``top-down first-fit'' procedure. This is a little tricky, so the reader
//! should pay close attention: The |trie_hash| array is cleared to zero
//! again and renamed |trie_ref| for this phase of the operation; later on,
//! |trie_ref[p]| will be nonzero only if the linked trie node |p| is the
//! smallest character
//! in a family and if the characters |c| of that family have been allocated to
//! locations |trie_ref[p]+c| in the |trie| array. Locations of |trie| that
//! are in use will have |trie_link=0|, while the unused holes in |trie|
//! will be doubly linked with |trie_link| pointing to the next larger vacant
//! location and |trie_back| pointing to the next smaller one. This double
//! linking will have been carried out only as far as |trie_max|, where
//! |trie_max| is the largest index of |trie| that will be needed.
//! To save time at the low end of the trie, we maintain array entries
//! |trie_min[c]| pointing to the smallest hole that is greater than~|c|.
//! Another array |trie_taken| tells whether or not a given location is
//! equal to |trie_ref[p]| for some |p|; this array is used to ensure that
//! distinct nodes in the compressed trie will have distinct |trie_ref|
//! entries.
//!
//! @d trie_ref==trie_hash {where linked trie families go into |trie|}
//! @d trie_back(#)==trie[#].lh {backward links in |trie| holes}
//!
//! @<Glob...@>=
//! @!init@!trie_taken:packed array[1..trie_size] of boolean;
//!   {does a family start here?}
//! @t\hskip10pt@>@!trie_min:array[ASCII_code] of trie_pointer;
//!   {the first possible slot for each character}
//! @t\hskip10pt@>@!trie_max:trie_pointer; {largest location used in |trie|}
//! @t\hskip10pt@>@!trie_not_ready:boolean; {is the trie still in linked form?}
//! tini
//!
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
//! @ Now let's go back to the easier problem, of building the linked
//! trie.  When \.{INITEX} has scanned the `\.{\\patterns}' control
//! sequence, it calls on |new_patterns| to do the right thing.
//!
//! @<Declare procedures for preprocessing hyph...@>=
//! procedure new_patterns; {initializes the hyphenation pattern data}
//! label done, done1;
//! var k,@!l:0..64; {indices into |hc| and |hyf|;
//!                   not always in |small_number| range}
//! @!digit_sensed:boolean; {should the next digit be treated as a letter?}
//! @!v:quarterword; {trie op code}
//! @!p,@!q:trie_pointer; {nodes of trie traversed during insertion}
//! @!first_child:boolean; {is |p=trie_l[q]|?}
//! @!c:ASCII_code; {character being inserted}
//! begin if trie_not_ready then
//!   begin set_cur_lang; scan_left_brace; {a left brace must follow \.{\\patterns}}
//!   @<Enter all of the patterns into a linked trie, until coming to a right
//!   brace@>;
//!   end
//! else begin print_err("Too late for "); print_esc("patterns");
//!   help1("All patterns must be given before typesetting begins.");
//!   error; link(garbage):=scan_toks(false,false); flush_list(def_ref);
//!   end;
//! end;
//!
//! @ Novices are not supposed to be using \.{\\patterns}, so the error
//! messages are terse. (Note that all error messages appear in \TeX's string
//! pool, even if they are used only by \.{INITEX}.)
//!
//! @<Enter all of the patterns into a linked trie...@>=
//! k:=0; hyf[0]:=0; digit_sensed:=false;
//! loop@+  begin get_x_token;
//!   case cur_cmd of
//!   letter,other_char:@<Append a new letter or a hyphen level@>;
//!   spacer,right_brace: begin if k>0 then
//!       @<Insert a new pattern into the linked trie@>;
//!     if cur_cmd=right_brace then goto done;
//!     k:=0; hyf[0]:=0; digit_sensed:=false;
//!     end;
//!   othercases begin print_err("Bad "); print_esc("patterns");
//! @.Bad \\patterns@>
//!     help1("(See Appendix H.)"); error;
//!     end
//!   endcases;
//!   end;
//! done:
//!
//! @ @<Append a new letter or a hyphen level@>=
//! if digit_sensed or(cur_chr<"0")or(cur_chr>"9") then
//!   begin if cur_chr="." then cur_chr:=0 {edge-of-word delimiter}
//!   else  begin cur_chr:=lc_code(cur_chr);
//!     if cur_chr=0 then
//!       begin print_err("Nonletter");
//! @.Nonletter@>
//!       help1("(See Appendix H.)"); error;
//!       end;
//!     end;
//!   if k<63 then
//!     begin incr(k); hc[k]:=cur_chr; hyf[k]:=0; digit_sensed:=false;
//!     end;
//!   end
//! else if k<63 then
//!   begin hyf[k]:=cur_chr-"0"; digit_sensed:=true;
//!   end
//!
//! @ When the following code comes into play, the pattern $p_1\ldots p_k$
//! appears in |hc[1..k]|, and the corresponding sequence of numbers $n_0\ldots
//! n_k$ appears in |hyf[0..k]|.
//!
//! @<Insert a new pattern into the linked trie@>=
//! begin @<Compute the trie op code, |v|, and set |l:=0|@>;
//! q:=0; hc[0]:=cur_lang;
//! while l<=k do
//!   begin c:=hc[l]; incr(l); p:=trie_l[q]; first_child:=true;
//!   while (p>0)and(c>so(trie_c[p])) do
//!     begin q:=p; p:=trie_r[q]; first_child:=false;
//!     end;
//!   if (p=0)or(c<so(trie_c[p])) then
//!     @<Insert a new trie node between |q| and |p|, and
//!       make |p| point to it@>;
//!   q:=p; {now node |q| represents $p_1\ldots p_{l-1}$}
//!   end;
//! if trie_o[q]<>min_quarterword then
//!   begin print_err("Duplicate pattern");
//! @.Duplicate pattern@>
//!   help1("(See Appendix H.)"); error;
//!   end;
//! trie_o[q]:=v;
//! end
//!
//! @ @<Insert a new trie node between |q| and |p|...@>=
//! begin if trie_ptr=trie_size then overflow("pattern memory",trie_size);
//! @:TeX capacity exceeded pattern memory}{\quad pattern memory@>
//! incr(trie_ptr); trie_r[trie_ptr]:=p; p:=trie_ptr; trie_l[p]:=0;
//! if first_child then trie_l[q]:=p@+else trie_r[q]:=p;
//! trie_c[p]:=si(c); trie_o[p]:=min_quarterword;
//! end
//!
//! @ @<Compute the trie op code, |v|...@>=
//! if hc[1]=0 then hyf[0]:=0;
//! if hc[k]=0 then hyf[k]:=0;
//! l:=k; v:=min_quarterword;
//! loop@+  begin if hyf[l]<>0 then v:=new_trie_op(k-l,hyf[l],v);
//!   if l>0 then decr(l)@+else goto done1;
//!   end;
//! done1:
//!
//! @ Finally we put everything together: Here is how the trie gets to its
//! final, efficient form.
//! The following packing routine is rigged so that the root of the linked
//! tree gets mapped into location 1 of |trie|, as required by the hyphenation
//! algorithm. This happens because the first call of |first_fit| will
//! ``take'' location~1.
//!
//! @<Declare procedures for preprocessing hyphenation patterns@>=
//! procedure init_trie;
//! var @!p:trie_pointer; {pointer for initialization}
//! @!j,@!k,@!t:integer; {all-purpose registers for initialization}
//! @!r,@!s:trie_pointer; {used to clean up the packed |trie|}
//! @!h:two_halves; {template used to zero out |trie|'s holes}
//! begin @<Get ready to compress the trie@>;
//! if trie_root<>0 then
//!   begin first_fit(trie_root); trie_pack(trie_root);
//!   end;
//! @<Move the data into |trie|@>;
//! trie_not_ready:=false;
//! end;
//!
