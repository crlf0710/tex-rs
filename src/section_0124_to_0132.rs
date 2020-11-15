//! @ The available-space list that keeps track of the variable-size portion
//! of |mem| is a nonempty, doubly-linked circular list of empty nodes,
//! pointed to by the roving pointer |rover|.
//!
//! Each empty node has size 2 or more; the first word contains the special
//! value |max_halfword| in its |link| field and the size in its |info| field;
//! the second word contains the two pointers for double linking.
//!
//! Each nonempty node also has size 2 or more. Its first word is of type
//! |two_halves|\kern-1pt, and its |link| field is never equal to |max_halfword|.
//! Otherwise there is complete flexibility with respect to the contents
//! of its other fields and its other words.
//!
//! (We require |mem_max<max_halfword| because terrible things can happen
//! when |max_halfword| appears in the |link| field of a nonempty node.)
//!
//! @d empty_flag == max_halfword {the |link| of an empty variable-size node}
//! @d is_empty(#) == (link(#)=empty_flag) {tests for empty node}
//! @d node_size == info {the size field in empty variable-size nodes}
//! @d llink(#) == info(#+1) {left link in doubly-linked list of empty nodes}
//! @d rlink(#) == link(#+1) {right link in doubly-linked list of empty nodes}
//!
//! @<Glob...@>=
//! @!rover : pointer; {points to some node in the list of empties}
//!
//! @ A call to |get_node| with argument |s| returns a pointer to a new node
//! of size~|s|, which must be 2~or more. The |link| field of the first word
//! of this new node is set to null. An overflow stop occurs if no suitable
//! space exists.
//!
//! If |get_node| is called with $s=2^{30}$, it simply merges adjacent free
//! areas and returns the value |max_halfword|.
//!
//! @p function get_node(@!s:integer):pointer; {variable-size node allocation}
//! label found,exit,restart;
//! var p:pointer; {the node currently under inspection}
//! @!q:pointer; {the node physically after node |p|}
//! @!r:integer; {the newly allocated node, or a candidate for this honor}
//! @!t:integer; {temporary register}
//! begin restart: p:=rover; {start at some free node in the ring}
//! repeat @<Try to allocate within node |p| and its physical successors,
//!   and |goto found| if allocation was possible@>;
//! @^inner loop@>
//! p:=rlink(p); {move to the next node in the ring}
//! until p=rover; {repeat until the whole list has been traversed}
//! if s=@'10000000000 then
//!   begin get_node:=max_halfword; return;
//!   end;
//! if lo_mem_max+2<hi_mem_min then if lo_mem_max+2<=mem_bot+max_halfword then
//!   @<Grow more variable-size memory and |goto restart|@>;
//! overflow("main memory size",mem_max+1-mem_min);
//!   {sorry, nothing satisfactory is left}
//! @:TeX capacity exceeded main memory size}{\quad main memory size@>
//! found: link(r):=null; {this node is now nonempty}
//! @!stat var_used:=var_used+s; {maintain usage statistics}
//! tats@;@/
//! get_node:=r;
//! exit:end;
//!
//! @ The lower part of |mem| grows by 1000 words at a time, unless
//! we are very close to going under. When it grows, we simply link
//! a new node into the available-space list. This method of controlled
//! growth helps to keep the |mem| usage consecutive when \TeX\ is
//! implemented on ``virtual memory'' systems.
//! @^virtual memory@>
//!
//! @<Grow more variable-size memory and |goto restart|@>=
//! begin if hi_mem_min-lo_mem_max>=1998 then t:=lo_mem_max+1000
//! else t:=lo_mem_max+1+(hi_mem_min-lo_mem_max) div 2;
//!   {|lo_mem_max+2<=t<hi_mem_min|}
//! p:=llink(rover); q:=lo_mem_max; rlink(p):=q; llink(rover):=q;@/
//! if t>mem_bot+max_halfword then t:=mem_bot+max_halfword;
//! rlink(q):=rover; llink(q):=p; link(q):=empty_flag; node_size(q):=t-lo_mem_max;@/
//! lo_mem_max:=t; link(lo_mem_max):=null; info(lo_mem_max):=null;
//! rover:=q; goto restart;
//! end
//!
//! @ Empirical tests show that the routine in this section performs a
//! node-merging operation about 0.75 times per allocation, on the average,
//! after which it finds that |r>p+1| about 95\pct! of the time.
//!
//! @<Try to allocate...@>=
//! q:=p+node_size(p); {find the physical successor}
//! @^inner loop@>
//! while is_empty(q) do {merge node |p| with node |q|}
//!   begin t:=rlink(q);
//!   if q=rover then rover:=t;
//!   llink(t):=llink(q); rlink(llink(q)):=t;@/
//!   q:=q+node_size(q);
//!   end;
//! r:=q-s;
//! if r>p+1 then @<Allocate from the top of node |p| and |goto found|@>;
//! if r=p then if rlink(p)<>p then
//!   @<Allocate entire node |p| and |goto found|@>;
//! node_size(p):=q-p {reset the size in case it grew}
//!
//! @ @<Allocate from the top...@>=
//! begin node_size(p):=r-p; {store the remaining size}
//! @^inner loop@>
//! rover:=p; {start searching here next time}
//! goto found;
//! end
//!
//! @ Here we delete node |p| from the ring, and let |rover| rove around.
//!
//! @<Allocate entire...@>=
//! begin rover:=rlink(p); t:=llink(p);
//! llink(rover):=t; rlink(t):=rover;
//! goto found;
//! end
//!
//! @ Conversely, when some variable-size node |p| of size |s| is no longer needed,
//! the operation |free_node(p,s)| will make its words available, by inserting
//! |p| as a new empty node just before where |rover| now points.
//! @^inner loop@>
//!
//! @p procedure free_node(@!p:pointer; @!s:halfword); {variable-size node
//!   liberation}
//! var q:pointer; {|llink(rover)|}
//! begin node_size(p):=s; link(p):=empty_flag;
//! q:=llink(rover); llink(p):=q; rlink(p):=rover; {set both links}
//! llink(rover):=p; rlink(q):=p; {insert |p| into the ring}
//! @!stat var_used:=var_used-s;@+tats@;{maintain statistics}
//! end;
//!
//! @ Just before \.{INITEX} writes out the memory, it sorts the doubly linked
//! available space list. The list is probably very short at such times, so a
//! simple insertion sort is used. The smallest available location will be
//! pointed to by |rover|, the next-smallest by |rlink(rover)|, etc.
//!
//! @p @!init procedure sort_avail; {sorts the available variable-size nodes
//!   by location}
//! var p,@!q,@!r: pointer; {indices into |mem|}
//! @!old_rover:pointer; {initial |rover| setting}
//! begin p:=get_node(@'10000000000); {merge adjacent free areas}
//! p:=rlink(rover); rlink(rover):=max_halfword; old_rover:=rover;
//! while p<>old_rover do @<Sort \(p)|p| into the list starting at |rover|
//!   and advance |p| to |rlink(p)|@>;
//! p:=rover;
//! while rlink(p)<>max_halfword do
//!   begin llink(rlink(p)):=p; p:=rlink(p);
//!   end;
//! rlink(p):=rover; llink(rover):=p;
//! end;
//! tini
//!
//! @ The following |while| loop is guaranteed to
//! terminate, since the list that starts at
//! |rover| ends with |max_halfword| during the sorting procedure.
//!
//! @<Sort \(p)|p|...@>=
//! if p<rover then
//!   begin q:=p; p:=rlink(q); rlink(q):=rover; rover:=q;
//!   end
//! else  begin q:=rover;
//!   while rlink(q)<p do q:=rlink(q);
//!   r:=rlink(p); rlink(p):=rlink(q); rlink(q):=p; p:=r;
//!   end
//!
