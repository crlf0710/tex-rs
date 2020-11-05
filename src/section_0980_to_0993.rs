//! @* \[45] The page builder.
//! When \TeX\ appends new material to its main vlist in vertical mode, it uses
//! a method something like |vsplit| to decide where a page ends, except that
//! the calculations are done ``on line'' as new items come in.
//! The main complication in this process is that insertions must be put
//! into their boxes and removed from the vlist, in a more-or-less optimum manner.
//!
//! We shall use the term ``current page'' for that part of the main vlist that
//! is being considered as a candidate for being broken off and sent to the
//! user's output routine. The current page starts at |link(page_head)|, and
//! it ends at |page_tail|.  We have |page_head=page_tail| if this list is empty.
//! @^current page@>
//!
//! Utter chaos would reign if the user kept changing page specifications
//! while a page is being constructed, so the page builder keeps the pertinent
//! specifications frozen as soon as the page receives its first box or
//! insertion.  The global variable |page_contents| is |empty| when the
//! current page contains only mark nodes and content-less whatsit nodes; it
//! is |inserts_only| if the page contains only insertion nodes in addition to
//! marks and whatsits.  Glue nodes, kern nodes, and penalty nodes are
//! discarded until a box or rule node appears, at which time |page_contents|
//! changes to |box_there|.  As soon as |page_contents| becomes non-|empty|,
//! the current |vsize| and |max_depth| are squirreled away into |page_goal|
//! and |page_max_depth|; the latter values will be used until the page has
//! been forwarded to the user's output routine. The \.{\\topskip} adjustment
//! is made when |page_contents| changes to |box_there|.
//!
//! Although |page_goal| starts out equal to |vsize|, it is decreased by the
//! scaled natural height-plus-depth of the insertions considered so far, and by
//! the \.{\\skip} corrections for those insertions. Therefore it represents
//! the size into which the non-inserted material should fit, assuming that
//! all insertions in the current page have been made.
//!
//! The global variables |best_page_break| and |least_page_cost| correspond
//! respectively to the local variables |best_place| and |least_cost| in the
//! |vert_break| routine that we have already studied; i.e., they record the
//! location and value of the best place currently known for breaking the
//! current page. The value of |page_goal| at the time of the best break is
//! stored in |best_size|.
//!
//! @d inserts_only=1
//!   {|page_contents| when an insert node has been contributed, but no boxes}
//! @d box_there=2 {|page_contents| when a box or rule has been contributed}
//!
//! @<Glob...@>=
//! @!page_tail:pointer; {the final node on the current page}
//! @!page_contents:empty..box_there; {what is on the current page so far?}
//! @!page_max_depth:scaled; {maximum box depth on page being built}
//! @!best_page_break:pointer; {break here to get the best page known so far}
//! @!least_page_cost:integer; {the score for this currently best page}
//! @!best_size:scaled; {its |page_goal|}
//!
//! @ The page builder has another data structure to keep track of insertions.
//! This is a list of four-word nodes, starting and ending at |page_ins_head|.
//! That is, the first element of the list is node |r@t$_1$@>=link(page_ins_head)|;
//! node $r_j$ is followed by |r@t$_{j+1}$@>=link(r@t$_j$@>)|; and if there are
//! |n| items we have |r@t$_{n+1}$@>=page_ins_head|. The |subtype| field of
//! each node in this list refers to an insertion number; for example, `\.{\\insert
//! 250}' would correspond to a node whose |subtype| is |qi(250)|
//! (the same as the |subtype| field of the relevant |ins_node|). These |subtype|
//! fields are in increasing order, and |subtype(page_ins_head)=
//! qi(255)|, so |page_ins_head| serves as a convenient sentinel
//! at the end of the list. A record is present for each insertion number that
//! appears in the current page.
//!
//! The |type| field in these nodes distinguishes two possibilities that
//! might occur as we look ahead before deciding on the optimum page break.
//! If |type(r)=inserting|, then |height(r)| contains the total of the
//! height-plus-depth dimensions of the box and all its inserts seen so far.
//! If |type(r)=split_up|, then no more insertions will be made into this box,
//! because at least one previous insertion was too big to fit on the current
//! page; |broken_ptr(r)| points to the node where that insertion will be
//! split, if \TeX\ decides to split it, |broken_ins(r)| points to the
//! insertion node that was tentatively split, and |height(r)| includes also the
//! natural height plus depth of the part that would be split off.
//!
//! In both cases, |last_ins_ptr(r)| points to the last |ins_node|
//! encountered for box |qo(subtype(r))| that would be at least partially
//! inserted on the next page; and |best_ins_ptr(r)| points to the last
//! such |ins_node| that should actually be inserted, to get the page with
//! minimum badness among all page breaks considered so far. We have
//! |best_ins_ptr(r)=null| if and only if no insertion for this box should
//! be made to produce this optimum page.
//!
//! The data structure definitions here use the fact that the |@!height| field
//! appears in the fourth word of a box node.
//! @^data structure assumptions@>
//!
//! @d page_ins_node_size=4 {number of words for a page insertion node}
//! @d inserting=0 {an insertion class that has not yet overflowed}
//! @d split_up=1 {an overflowed insertion class}
//! @d broken_ptr(#)==link(#+1)
//!   {an insertion for this class will break here if anywhere}
//! @d broken_ins(#)==info(#+1) {this insertion might break at |broken_ptr|}
//! @d last_ins_ptr(#)==link(#+2) {the most recent insertion for this |subtype|}
//! @d best_ins_ptr(#)==info(#+2) {the optimum most recent insertion}
//!
//! @<Initialize the special list heads...@>=
//! subtype(page_ins_head):=qi(255);
//! type(page_ins_head):=split_up; link(page_ins_head):=page_ins_head;
//!
//! @ An array |page_so_far| records the heights and depths of everything
//! on the current page. This array contains six |scaled| numbers, like the
//! similar arrays already considered in |line_break| and |vert_break|; and it
//! also contains |page_goal| and |page_depth|, since these values are
//! all accessible to the user via |set_page_dimen| commands. The
//! value of |page_so_far[1]| is also called |page_total|.  The stretch
//! and shrink components of the \.{\\skip} corrections for each insertion are
//! included in |page_so_far|, but the natural space components of these
//! corrections are not, since they have been subtracted from |page_goal|.
//!
//! The variable |page_depth| records the depth of the current page; it has been
//! adjusted so that it is at most |page_max_depth|. The variable
//! |last_glue| points to the glue specification of the most recent node
//! contributed from the contribution list, if this was a glue node; otherwise
//! |last_glue=max_halfword|. (If the contribution list is nonempty,
//! however, the value of |last_glue| is not necessarily accurate.)
//! The variables |last_penalty| and |last_kern| are similar.  And
//! finally, |insert_penalties| holds the sum of the penalties associated with
//! all split and floating insertions.
//!
//! @d page_goal==page_so_far[0] {desired height of information on page being built}
//! @d page_total==page_so_far[1] {height of the current page}
//! @d page_shrink==page_so_far[6] {shrinkability of the current page}
//! @d page_depth==page_so_far[7] {depth of the current page}
//!
//! @<Glob...@>=
//! @!page_so_far:array [0..7] of scaled; {height and glue of the current page}
//! @!last_glue:pointer; {used to implement \.{\\lastskip}}
//! @!last_penalty:integer; {used to implement \.{\\lastpenalty}}
//! @!last_kern:scaled; {used to implement \.{\\lastkern}}
//! @!insert_penalties:integer; {sum of the penalties for held-over insertions}
//!
//! @ @<Put each...@>=
//! primitive("pagegoal",set_page_dimen,0);
//! @!@:page_goal_}{\.{\\pagegoal} primitive@>
//! primitive("pagetotal",set_page_dimen,1);
//! @!@:page_total_}{\.{\\pagetotal} primitive@>
//! primitive("pagestretch",set_page_dimen,2);
//! @!@:page_stretch_}{\.{\\pagestretch} primitive@>
//! primitive("pagefilstretch",set_page_dimen,3);
//! @!@:page_fil_stretch_}{\.{\\pagefilstretch} primitive@>
//! primitive("pagefillstretch",set_page_dimen,4);
//! @!@:page_fill_stretch_}{\.{\\pagefillstretch} primitive@>
//! primitive("pagefilllstretch",set_page_dimen,5);
//! @!@:page_filll_stretch_}{\.{\\pagefilllstretch} primitive@>
//! primitive("pageshrink",set_page_dimen,6);
//! @!@:page_shrink_}{\.{\\pageshrink} primitive@>
//! primitive("pagedepth",set_page_dimen,7);
//! @!@:page_depth_}{\.{\\pagedepth} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! set_page_dimen: case chr_code of
//! 0: print_esc("pagegoal");
//! 1: print_esc("pagetotal");
//! 2: print_esc("pagestretch");
//! 3: print_esc("pagefilstretch");
//! 4: print_esc("pagefillstretch");
//! 5: print_esc("pagefilllstretch");
//! 6: print_esc("pageshrink");
//! othercases print_esc("pagedepth")
//! endcases;
//!
//! @ @d print_plus_end(#)==print(#);@+end
//! @d print_plus(#)==if page_so_far[#]<>0 then
//!   begin print(" plus "); print_scaled(page_so_far[#]); print_plus_end
//!
//! @p procedure print_totals;
//! begin print_scaled(page_total);
//! print_plus(2)("");
//! print_plus(3)("fil");
//! print_plus(4)("fill");
//! print_plus(5)("filll");
//! if page_shrink<>0 then
//!   begin print(" minus "); print_scaled(page_shrink);
//!   end;
//! end;
//!
//! @ @<Show the status of the current page@>=
//! if page_head<>page_tail then
//!   begin print_nl("### current page:");
//!   if output_active then print(" (held over for next output)");
//! @.held over for next output@>
//!   show_box(link(page_head));
//!   if page_contents>empty then
//!     begin print_nl("total height "); print_totals;
//! @:total_height}{\.{total height}@>
//!     print_nl(" goal height "); print_scaled(page_goal);
//! @.goal height@>
//!     r:=link(page_ins_head);
//!     while r<>page_ins_head do
//!       begin print_ln; print_esc("insert"); t:=qo(subtype(r));
//!       print_int(t); print(" adds ");
//!       if count(t)=1000 then t:=height(r)
//!       else t:=x_over_n(height(r),1000)*count(t);
//!       print_scaled(t);
//!       if type(r)=split_up then
//!         begin q:=page_head; t:=0;
//!         repeat q:=link(q);
//!         if (type(q)=ins_node)and(subtype(q)=subtype(r)) then incr(t);
//!         until q=broken_ins(r);
//!         print(", #"); print_int(t); print(" might split");
//!         end;
//!       r:=link(r);
//!       end;
//!     end;
//!   end
//!
//! @ Here is a procedure that is called when the |page_contents| is changing
//! from |empty| to |inserts_only| or |box_there|.
//!
//! @d set_page_so_far_zero(#)==page_so_far[#]:=0
//!
//! @p procedure freeze_page_specs(@!s:small_number);
//! begin page_contents:=s;
//! page_goal:=vsize; page_max_depth:=max_depth;
//! page_depth:=0; do_all_six(set_page_so_far_zero);
//! least_page_cost:=awful_bad;
//! @!stat if tracing_pages>0 then
//!   begin begin_diagnostic;
//!   print_nl("%% goal height="); print_scaled(page_goal);
//! @.goal height@>
//!   print(", max depth="); print_scaled(page_max_depth);
//!   end_diagnostic(false);
//!   end;@;@+tats@;@/
//! end;
//!
//! @ Pages are built by appending nodes to the current list in \TeX's
//! vertical mode, which is at the outermost level of the semantic nest. This
//! vlist is split into two parts; the ``current page'' that we have been
//! talking so much about already, and the ``contribution list'' that receives
//! new nodes as they are created.  The current page contains everything that
//! the page builder has accounted for in its data structures, as described
//! above, while the contribution list contains other things that have been
//! generated by other parts of \TeX\ but have not yet been
//! seen by the page builder.
//! The contribution list starts at |link(contrib_head)|, and it ends at the
//! current node in \TeX's vertical mode.
//!
//! When \TeX\ has appended new material in vertical mode, it calls the procedure
//! |build_page|, which tries to catch up by moving nodes from the contribution
//! list to the current page. This procedure will succeed in its goal of
//! emptying the contribution list, unless a page break is discovered, i.e.,
//! unless the current page has grown to the point where the optimum next
//! page break has been determined. In the latter case, the nodes after the
//! optimum break will go back onto the contribution list, and control will
//! effectively pass to the user's output routine.
//!
//! We make |type(page_head)=glue_node|, so that an initial glue node on
//! the current page will not be considered a valid breakpoint.
//!
//! @<Initialize the special list...@>=
//! type(page_head):=glue_node; subtype(page_head):=normal;
//!
//! @ The global variable |output_active| is true during the time the
//! user's output routine is driving \TeX.
//!
//! @<Glob...@>=
//! @!output_active:boolean; {are we in the midst of an output routine?}
//!
//! @ @<Set init...@>=
//! output_active:=false; insert_penalties:=0;
//!
//! @ The page builder is ready to start a fresh page if we initialize
//! the following state variables. (However, the page insertion list is initialized
//! elsewhere.)
//!
//! @<Start a new current page@>=
//! page_contents:=empty; page_tail:=page_head; link(page_head):=null;@/
//! last_glue:=max_halfword; last_penalty:=0; last_kern:=0;
//! page_depth:=0; page_max_depth:=0
//!
//! @ At certain times box 255 is supposed to be void (i.e., |null|),
//! or an insertion box is supposed to be ready to accept a vertical list.
//! If not, an error message is printed, and the following subroutine
//! flushes the unwanted contents, reporting them to the user.
//!
//! @p procedure box_error(@!n:eight_bits);
//! begin error; begin_diagnostic;
//! print_nl("The following box has been deleted:");
//! @.The following...deleted@>
//! show_box(box(n)); end_diagnostic(true);
//! flush_node_list(box(n)); box(n):=null;
//! end;
//!
//! @ The following procedure guarantees that a given box register
//! does not contain an \.{\\hbox}.
//!
//! @p procedure ensure_vbox(@!n:eight_bits);
//! var p:pointer; {the box register contents}
//! begin p:=box(n);
//! if p<>null then if type(p)=hlist_node then
//!   begin print_err("Insertions can only be added to a vbox");
//! @.Insertions can only...@>
//!   help3("Tut tut: You're trying to \insert into a")@/
//!     ("\box register that now contains an \hbox.")@/
//!     ("Proceed, and I'll discard its present contents.");
//!   box_error(n);
//!   end;
//! end;
//!
