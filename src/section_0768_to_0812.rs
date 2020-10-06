//! @* \[37] Alignment.
//! It's sort of a miracle whenever \.{\\halign} and \.{\\valign} work, because
//! they cut across so many of the control structures of \TeX.
//!
//! Therefore the
//! present page is probably not the best place for a beginner to start reading
//! this program; it is better to master everything else first.
//!
//! Let us focus our thoughts on an example of what the input might be, in order
//! to get some idea about how the alignment miracle happens. The example doesn't
//! do anything useful, but it is sufficiently general to indicate all of the
//! special cases that must be dealt with; please do not be disturbed by its
//! apparent complexity and meaninglessness.
//! $$\vbox{\halign{\.{#}\hfil\cr
//! {}\\tabskip 2pt plus 3pt\cr
//! {}\\halign to 300pt\{u1\#v1\&\cr
//! \hskip 50pt\\tabskip 1pt plus 1fil u2\#v2\&\cr
//! \hskip 50pt u3\#v3\\cr\cr
//! \hskip 25pt a1\&\\omit a2\&\\vrule\\cr\cr
//! \hskip 25pt \\noalign\{\\vskip 3pt\}\cr
//! \hskip 25pt b1\\span b2\\cr\cr
//! \hskip 25pt \\omit\&c2\\span\\omit\\cr\}\cr}}$$
//! Here's what happens:
//!
//! \yskip
//! (0) When `\.{\\halign to 300pt\{}' is scanned, the |scan_spec| routine
//! places the 300pt dimension onto the |save_stack|, and an |align_group|
//! code is placed above it. This will make it possible to complete the alignment
//! when the matching `\.\}' is found.
//!
//! (1) The preamble is scanned next. Macros in the preamble are not expanded,
//! @^preamble@>
//! except as part of a tabskip specification. For example, if \.{u2} had been
//! a macro in the preamble above, it would have been expanded, since \TeX\
//! must look for `\.{minus...}' as part of the tabskip glue. A ``preamble list''
//! is constructed based on the user's preamble; in our case it contains the
//! following seven items:
//! $$\vbox{\halign{\.{#}\hfil\qquad&(#)\hfil\cr
//! {}\\glue 2pt plus 3pt&the tabskip preceding column 1\cr
//! {}\\alignrecord, width $-\infty$&preamble info for column 1\cr
//! {}\\glue 2pt plus 3pt&the tabskip between columns 1 and 2\cr
//! {}\\alignrecord, width $-\infty$&preamble info for column 2\cr
//! {}\\glue 1pt plus 1fil&the tabskip between columns 2 and 3\cr
//! {}\\alignrecord, width $-\infty$&preamble info for column 3\cr
//! {}\\glue 1pt plus 1fil&the tabskip following column 3\cr}}$$
//! These ``alignrecord'' entries have the same size as an |unset_node|,
//! since they will later be converted into such nodes. However, at the
//! moment they have no |type| or |subtype| fields; they have |info| fields
//! instead, and these |info| fields are initially set to the value |end_span|,
//! for reasons explained below. Furthermore, the alignrecord nodes have no
//! |height| or |depth| fields; these are renamed |u_part| and |v_part|,
//! and they point to token lists for the templates of the alignment.
//! For example, the |u_part| field in the first alignrecord points to the
//! token list `\.{u1}', i.e., the template preceding the `\.\#' for column~1.
//!
//! (2) \TeX\ now looks at what follows the \.{\\cr} that ended the preamble.
//! It is not `\.{\\noalign}' or `\.{\\omit}', so this input is put back to
//! be read again, and the template `\.{u1}' is fed to the scanner. Just
//! before reading `\.{u1}', \TeX\ goes into restricted horizontal mode.
//! Just after reading `\.{u1}', \TeX\ will see `\.{a1}', and then (when the
//! {\.\&} is sensed) \TeX\ will see `\.{v1}'. Then \TeX\ scans an |endv|
//! token, indicating the end of a column. At this point an |unset_node| is
//! created, containing the contents of the current hlist (i.e., `\.{u1a1v1}').
//! The natural width of this unset node replaces the |width| field of the
//! alignrecord for column~1; in general, the alignrecords will record the
//! maximum natural width that has occurred so far in a given column.
//!
//! (3) Since `\.{\\omit}' follows the `\.\&', the templates for column~2
//! are now bypassed. Again \TeX\ goes into restricted horizontal mode and
//! makes an |unset_node| from the resulting hlist; but this time the
//! hlist contains simply `\.{a2}'. The natural width of the new unset box
//! is remembered in the |width| field of the alignrecord for column~2.
//!
//! (4) A third |unset_node| is created for column 3, using essentially the
//! mechanism that worked for column~1; this unset box contains `\.{u3\\vrule
//! v3}'. The vertical rule in this case has running dimensions that will later
//! extend to the height and depth of the whole first row, since each |unset_node|
//! in a row will eventually inherit the height and depth of its enclosing box.
//!
//! (5) The first row has now ended; it is made into a single unset box
//! comprising the following seven items:
//! $$\vbox{\halign{\hbox to 325pt{\qquad\.{#}\hfil}\cr
//! {}\\glue 2pt plus 3pt\cr
//! {}\\unsetbox for 1 column: u1a1v1\cr
//! {}\\glue 2pt plus 3pt\cr
//! {}\\unsetbox for 1 column: a2\cr
//! {}\\glue 1pt plus 1fil\cr
//! {}\\unsetbox for 1 column: u3\\vrule v3\cr
//! {}\\glue 1pt plus 1fil\cr}}$$
//! The width of this unset row is unimportant, but it has the correct height
//! and depth, so the correct baselineskip glue will be computed as the row
//! is inserted into a vertical list.
//!
//! (6) Since `\.{\\noalign}' follows the current \.{\\cr}, \TeX\ appends
//! additional material (in this case \.{\\vskip 3pt}) to the vertical list.
//! While processing this material, \TeX\ will be in internal vertical
//! mode, and |no_align_group| will be on |save_stack|.
//!
//! (7) The next row produces an unset box that looks like this:
//! $$\vbox{\halign{\hbox to 325pt{\qquad\.{#}\hfil}\cr
//! {}\\glue 2pt plus 3pt\cr
//! {}\\unsetbox for 2 columns: u1b1v1u2b2v2\cr
//! {}\\glue 1pt plus 1fil\cr
//! {}\\unsetbox for 1 column: {\rm(empty)}\cr
//! {}\\glue 1pt plus 1fil\cr}}$$
//! The natural width of the unset box that spans columns 1~and~2 is stored
//! in a ``span node,'' which we will explain later; the |info| field of the
//! alignrecord for column~1 now points to the new span node, and the |info|
//! of the span node points to |end_span|.
//!
//! (8) The final row produces the unset box
//! $$\vbox{\halign{\hbox to 325pt{\qquad\.{#}\hfil}\cr
//! {}\\glue 2pt plus 3pt\cr
//! {}\\unsetbox for 1 column: {\rm(empty)}\cr
//! {}\\glue 2pt plus 3pt\cr
//! {}\\unsetbox for 2 columns: u2c2v2\cr
//! {}\\glue 1pt plus 1fil\cr}}$$
//! A new span node is attached to the alignrecord for column 2.
//!
//! (9) The last step is to compute the true column widths and to change all the
//! unset boxes to hboxes, appending the whole works to the vertical list that
//! encloses the \.{\\halign}. The rules for deciding on the final widths of
//! each unset column box will be explained below.
//!
//! \yskip\noindent
//! Note that as \.{\\halign} is being processed, we fearlessly give up control
//! to the rest of \TeX. At critical junctures, an alignment routine is
//! called upon to step in and do some little action, but most of the time
//! these routines just lurk in the background. It's something like
//! post-hypnotic suggestion.
//!
//! @ We have mentioned that alignrecords contain no |height| or |depth| fields.
//! Their |glue_sign| and |glue_order| are pre-empted as well, since it
//! is necessary to store information about what to do when a template ends.
//! This information is called the |extra_info| field.
//!
//! @d u_part(#)==mem[#+height_offset].int {pointer to \<u_j> token list}
//! @d v_part(#)==mem[#+depth_offset].int {pointer to \<v_j> token list}
//! @d extra_info(#)==info(#+list_offset) {info to remember during template}
//!
//! @ Alignments can occur within alignments, so a small stack is used to access
//! the alignrecord information. At each level we have a |preamble| pointer,
//! indicating the beginning of the preamble list; a |cur_align| pointer,
//! indicating the current position in the preamble list; a |cur_span| pointer,
//! indicating the value of |cur_align| at the beginning of a sequence of
//! spanned columns; a |cur_loop| pointer, indicating the tabskip glue before
//! an alignrecord that should be copied next if the current list is extended;
//! and the |align_state| variable, which indicates the nesting of braces so
//! that \.{\\cr} and \.{\\span} and tab marks are properly intercepted.
//! There also are pointers |cur_head| and |cur_tail| to the head and tail
//! of a list of adjustments being moved out from horizontal mode to
//! vertical~mode.
//!
//! The current values of these seven quantities appear in global variables;
//! when they have to be pushed down, they are stored in 5-word nodes, and
//! |align_ptr| points to the topmost such node.
//!
//! @d preamble==link(align_head) {the current preamble list}
//! @d align_stack_node_size=5 {number of |mem| words to save alignment states}
//!
//! @<Glob...@>=
//! @!cur_align:pointer; {current position in preamble list}
//! @!cur_span:pointer; {start of currently spanned columns in preamble list}
//! @!cur_loop:pointer; {place to copy when extending a periodic preamble}
//! @!align_ptr:pointer; {most recently pushed-down alignment stack node}
//! @!cur_head,@!cur_tail:pointer; {adjustment list pointers}
//!
//! @ The |align_state| and |preamble| variables are initialized elsewhere.
//!
//! @<Set init...@>=
//! align_ptr:=null; cur_align:=null; cur_span:=null; cur_loop:=null;
//! cur_head:=null; cur_tail:=null;
//!
//! @ Alignment stack maintenance is handled by a pair of trivial routines
//! called |push_alignment| and |pop_alignment|.
//!
//! @p procedure push_alignment;
//! var p:pointer; {the new alignment stack node}
//! begin p:=get_node(align_stack_node_size);
//! link(p):=align_ptr; info(p):=cur_align;
//! llink(p):=preamble; rlink(p):=cur_span;
//! mem[p+2].int:=cur_loop; mem[p+3].int:=align_state;
//! info(p+4):=cur_head; link(p+4):=cur_tail;
//! align_ptr:=p;
//! cur_head:=get_avail;
//! end;
//! @#
//! procedure pop_alignment;
//! var p:pointer; {the top alignment stack node}
//! begin free_avail(cur_head);
//! p:=align_ptr;
//! cur_tail:=link(p+4); cur_head:=info(p+4);
//! align_state:=mem[p+3].int; cur_loop:=mem[p+2].int;
//! cur_span:=rlink(p); preamble:=llink(p);
//! cur_align:=info(p); align_ptr:=link(p);
//! free_node(p,align_stack_node_size);
//! end;
//!
//! @ \TeX\ has eight procedures that govern alignments: |init_align| and
//! |fin_align| are used at the very beginning and the very end; |init_row| and
//! |fin_row| are used at the beginning and end of individual rows; |init_span|
//! is used at the beginning of a sequence of spanned columns (possibly involving
//! only one column); |init_col| and |fin_col| are used at the beginning and
//! end of individual columns; and |align_peek| is used after \.{\\cr} to see
//! whether the next item is \.{\\noalign}.
//!
//! We shall consider these routines in the order they are first used during
//! the course of a complete \.{\\halign}, namely |init_align|, |align_peek|,
//! |init_row|, |init_span|, |init_col|, |fin_col|, |fin_row|, |fin_align|.
//!
//! @ When \.{\\halign} or \.{\\valign} has been scanned in an appropriate
//! mode, \TeX\ calls |init_align|, whose task is to get everything off to a
//! good start. This mostly involves scanning the preamble and putting its
//! information into the preamble list.
//! @^preamble@>
//!
//! @p @t\4@>@<Declare the procedure called |get_preamble_token|@>@t@>@/
//! procedure@?align_peek; forward;@t\2@>@/
//! procedure@?normal_paragraph; forward;@t\2@>@/
//! procedure init_align;
//! label done, done1, done2, continue;
//! var save_cs_ptr:pointer; {|warning_index| value for error messages}
//! @!p:pointer; {for short-term temporary use}
//! begin save_cs_ptr:=cur_cs; {\.{\\halign} or \.{\\valign}, usually}
//! push_alignment; align_state:=-1000000; {enter a new alignment level}
//! @<Check for improper alignment in displayed math@>;
//! push_nest; {enter a new semantic level}
//! @<Change current mode to |-vmode| for \.{\\halign}, |-hmode| for \.{\\valign}@>;
//! scan_spec(align_group,false);@/
//! @<Scan the preamble and record it in the |preamble| list@>;
//! new_save_level(align_group);
//! if every_cr<>null then begin_token_list(every_cr,every_cr_text);
//! align_peek; {look for \.{\\noalign} or \.{\\omit}}
//! end;
//!
//! @ In vertical modes, |prev_depth| already has the correct value. But
//! if we are in |mmode| (displayed formula mode), we reach out to the
//! enclosing vertical mode for the |prev_depth| value that produces the
//! correct baseline calculations.
//!
//! @<Change current mode...@>=
//! if mode=mmode then
//!   begin mode:=-vmode; prev_depth:=nest[nest_ptr-2].aux_field.sc;
//!   end
//! else if mode>0 then negate(mode)
//!
//! @ When \.{\\halign} is used as a displayed formula, there should be
//! no other pieces of mlists present.
//!
//! @<Check for improper alignment in displayed math@>=
//! if (mode=mmode)and((tail<>head)or(incompleat_noad<>null)) then
//!   begin print_err("Improper "); print_esc("halign"); print(" inside $$'s");
//! @.Improper \\halign...@>
//!   help3("Displays can use special alignments (like \eqalignno)")@/
//!   ("only if nothing but the alignment itself is between $$'s.")@/
//!   ("So I've deleted the formulas that preceded this alignment.");
//!   error; flush_math;
//!   end
//!
//! @ @<Scan the preamble and record it in the |preamble| list@>=
//! preamble:=null; cur_align:=align_head; cur_loop:=null; scanner_status:=aligning;
//! warning_index:=save_cs_ptr; align_state:=-1000000;
//!   {at this point, |cur_cmd=left_brace|}
//! loop@+  begin @<Append the current tabskip glue to the preamble list@>;
//!   if cur_cmd=car_ret then goto done; {\.{\\cr} ends the preamble}
//!   @<Scan preamble text until |cur_cmd| is |tab_mark| or |car_ret|,
//!     looking for changes in the tabskip glue; append an
//!     alignrecord to the preamble list@>;
//!   end;
//! done: scanner_status:=normal
//!
//! @ @<Append the current tabskip glue to the preamble list@>=
//! link(cur_align):=new_param_glue(tab_skip_code);
//! cur_align:=link(cur_align)
//!
//! @ @<Scan preamble text until |cur_cmd| is |tab_mark| or |car_ret|...@>=
//! @<Scan the template \<u_j>, putting the resulting token list in |hold_head|@>;
//! link(cur_align):=new_null_box; cur_align:=link(cur_align); {a new alignrecord}
//! info(cur_align):=end_span; width(cur_align):=null_flag;
//! u_part(cur_align):=link(hold_head);
//! @<Scan the template \<v_j>, putting the resulting token list in |hold_head|@>;
//! v_part(cur_align):=link(hold_head)
//!
//! @ We enter `\.{\\span}' into |eqtb| with |tab_mark| as its command code,
//! and with |span_code| as the command modifier. This makes \TeX\ interpret it
//! essentially the same as an alignment delimiter like `\.\&', yet it is
//! recognizably different when we need to distinguish it from a normal delimiter.
//! It also turns out to be useful to give a special |cr_code| to `\.{\\cr}',
//! and an even larger |cr_cr_code| to `\.{\\crcr}'.
//!
//! The end of a template is represented by two ``frozen'' control sequences
//! called \.{\\endtemplate}. The first has the command code |end_template|, which
//! is |>outer_call|, so it will not easily disappear in the presence of errors.
//! The |get_x_token| routine converts the first into the second, which has |endv|
//! as its command code.
//!
//! @d span_code=256 {distinct from any character}
//! @d cr_code=257 {distinct from |span_code| and from any character}
//! @d cr_cr_code=cr_code+1 {this distinguishes \.{\\crcr} from \.{\\cr}}
//! @d end_template_token==cs_token_flag+frozen_end_template
//!
//! @<Put each of \TeX's primitives into the hash table@>=
//! primitive("span",tab_mark,span_code);@/
//! @!@:span_}{\.{\\span} primitive@>
//! primitive("cr",car_ret,cr_code);
//! @!@:cr_}{\.{\\cr} primitive@>
//! text(frozen_cr):="cr"; eqtb[frozen_cr]:=eqtb[cur_val];@/
//! primitive("crcr",car_ret,cr_cr_code);
//! @!@:cr_cr_}{\.{\\crcr} primitive@>
//! text(frozen_end_template):="endtemplate"; text(frozen_endv):="endtemplate";
//! eq_type(frozen_endv):=endv; equiv(frozen_endv):=null_list;
//! eq_level(frozen_endv):=level_one;@/
//! eqtb[frozen_end_template]:=eqtb[frozen_endv];
//! eq_type(frozen_end_template):=end_template;
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! tab_mark: if chr_code=span_code then print_esc("span")
//!   else chr_cmd("alignment tab character ");
//! car_ret: if chr_code=cr_code then print_esc("cr")
//!   else print_esc("crcr");
//!
//! @ The preamble is copied directly, except that \.{\\tabskip} causes a change
//! to the tabskip glue, thereby possibly expanding macros that immediately
//! follow it. An appearance of \.{\\span} also causes such an expansion.
//!
//! Note that if the preamble contains `\.{\\global\\tabskip}', the `\.{\\global}'
//! token survives in the preamble and the `\.{\\tabskip}' defines new
//! tabskip glue (locally).
//!
//! @<Declare the procedure called |get_preamble_token|@>=
//! procedure get_preamble_token;
//! label restart;
//! begin restart: get_token;
//! while (cur_chr=span_code)and(cur_cmd=tab_mark) do
//!   begin get_token; {this token will be expanded once}
//!   if cur_cmd>max_command then
//!     begin expand; get_token;
//!     end;
//!   end;
//! if cur_cmd=endv then
//!   fatal_error("(interwoven alignment preambles are not allowed)");
//! @.interwoven alignment preambles...@>
//! if (cur_cmd=assign_glue)and(cur_chr=glue_base+tab_skip_code) then
//!   begin scan_optional_equals; scan_glue(glue_val);
//!   if global_defs>0 then geq_define(glue_base+tab_skip_code,glue_ref,cur_val)
//!   else eq_define(glue_base+tab_skip_code,glue_ref,cur_val);
//!   goto restart;
//!   end;
//! end;
//!
//! @ Spaces are eliminated from the beginning of a template.
//!
//! @<Scan the template \<u_j>...@>=
//! p:=hold_head; link(p):=null;
//! loop@+  begin get_preamble_token;
//!   if cur_cmd=mac_param then goto done1;
//!   if (cur_cmd<=car_ret)and(cur_cmd>=tab_mark)and(align_state=-1000000) then
//!    if (p=hold_head)and(cur_loop=null)and(cur_cmd=tab_mark)
//!     then cur_loop:=cur_align
//!    else  begin print_err("Missing # inserted in alignment preamble");
//! @.Missing \# inserted...@>
//!     help3("There should be exactly one # between &'s, when an")@/
//!     ("\halign or \valign is being set up. In this case you had")@/
//!     ("none, so I've put one in; maybe that will work.");
//!     back_error; goto done1;
//!     end
//!   else if (cur_cmd<>spacer)or(p<>hold_head) then
//!     begin link(p):=get_avail; p:=link(p); info(p):=cur_tok;
//!     end;
//!   end;
//! done1:
//!
//! @ @<Scan the template \<v_j>...@>=
//! p:=hold_head; link(p):=null;
//! loop@+  begin continue: get_preamble_token;
//!   if (cur_cmd<=car_ret)and(cur_cmd>=tab_mark)and(align_state=-1000000) then
//!     goto done2;
//!   if cur_cmd=mac_param then
//!     begin print_err("Only one # is allowed per tab");
//! @.Only one \# is allowed...@>
//!     help3("There should be exactly one # between &'s, when an")@/
//!     ("\halign or \valign is being set up. In this case you had")@/
//!     ("more than one, so I'm ignoring all but the first.");
//!     error; goto continue;
//!     end;
//!   link(p):=get_avail; p:=link(p); info(p):=cur_tok;
//!   end;
//! done2: link(p):=get_avail; p:=link(p);
//! info(p):=end_template_token {put \.{\\endtemplate} at the end}
//!
//! @ The tricky part about alignments is getting the templates into the
//! scanner at the right time, and recovering control when a row or column
//! is finished.
//!
//! We usually begin a row after each \.{\\cr} has been sensed, unless that
//! \.{\\cr} is followed by \.{\\noalign} or by the right brace that terminates
//! the alignment. The |align_peek| routine is used to look ahead and do
//! the right thing; it either gets a new row started, or gets a \.{\\noalign}
//! started, or finishes off the alignment.
//!
//! @<Declare the procedure called |align_peek|@>=
//! procedure align_peek;
//! label restart;
//! begin restart: align_state:=1000000; @<Get the next non-blank non-call token@>;
//! if cur_cmd=no_align then
//!   begin scan_left_brace; new_save_level(no_align_group);
//!   if mode=-vmode then normal_paragraph;
//!   end
//! else if cur_cmd=right_brace then fin_align
//! else if (cur_cmd=car_ret)and(cur_chr=cr_cr_code) then
//!   goto restart {ignore \.{\\crcr}}
//! else  begin init_row; {start a new row}
//!   init_col; {start a new column and replace what we peeked at}
//!   end;
//! end;
//!
//! @ To start a row (i.e., a `row' that rhymes with `dough' but not with `bough'),
//! we enter a new semantic level, copy the first tabskip glue, and change
//! from internal vertical mode to restricted horizontal mode or vice versa.
//! The |space_factor| and |prev_depth| are not used on this semantic level,
//! but we clear them to zero just to be tidy.
//!
//! @p @t\4@>@<Declare the procedure called |init_span|@>@t@>@/
//! procedure init_row;
//! begin push_nest; mode:=(-hmode-vmode)-mode;
//! if mode=-hmode then space_factor:=0 @+else prev_depth:=0;
//! tail_append(new_glue(glue_ptr(preamble)));
//! subtype(tail):=tab_skip_code+1;@/
//! cur_align:=link(preamble); cur_tail:=cur_head; init_span(cur_align);
//! end;
//!
//! @ The parameter to |init_span| is a pointer to the alignrecord where the
//! next column or group of columns will begin. A new semantic level is
//! entered, so that the columns will generate a list for subsequent packaging.
//!
//! @<Declare the procedure called |init_span|@>=
//! procedure init_span(@!p:pointer);
//! begin push_nest;
//! if mode=-hmode then space_factor:=1000
//! else  begin prev_depth:=ignore_depth; normal_paragraph;
//!   end;
//! cur_span:=p;
//! end;
//!
//! @ When a column begins, we assume that |cur_cmd| is either |omit| or else
//! the current token should be put back into the input until the \<u_j>
//! template has been scanned.  (Note that |cur_cmd| might be |tab_mark| or
//! |car_ret|.)  We also assume that |align_state| is approximately 1000000 at
//! this time.  We remain in the same mode, and start the template if it is
//! called for.
//!
//! @p procedure init_col;
//! begin extra_info(cur_align):=cur_cmd;
//! if cur_cmd=omit then align_state:=0
//! else  begin back_input; begin_token_list(u_part(cur_align),u_template);
//!   end; {now |align_state=1000000|}
//! end;
//!
//! @ The scanner sets |align_state| to zero when the \<u_j> template ends. When
//! a subsequent \.{\\cr} or \.{\\span} or tab mark occurs with |align_state=0|,
//! the scanner activates the following code, which fires up the \<v_j> template.
//! We need to remember the |cur_chr|, which is either |cr_cr_code|, |cr_code|,
//! |span_code|, or a character code, depending on how the column text has ended.
//!
//! This part of the program had better not be activated when the preamble
//! to another alignment is being scanned, or when no alignment preamble is active.
//!
//! @<Insert the \(v)\<v_j>...@>=
//! begin if (scanner_status=aligning) or (cur_align=null) then
//!   fatal_error("(interwoven alignment preambles are not allowed)");
//! @.interwoven alignment preambles...@>
//! cur_cmd:=extra_info(cur_align); extra_info(cur_align):=cur_chr;
//! if cur_cmd=omit then begin_token_list(omit_template,v_template)
//! else begin_token_list(v_part(cur_align),v_template);
//! align_state:=1000000; goto restart;
//! end
//!
//! @ The token list |omit_template| just referred to is a constant token
//! list that contains the special control sequence \.{\\endtemplate} only.
//!
//! @<Initialize the special...@>=
//! info(omit_template):=end_template_token; {|link(omit_template)=null|}
//!
//! @ When the |endv| command at the end of a \<v_j> template comes through the
//! scanner, things really start to happen; and it is the |fin_col| routine
//! that makes them happen. This routine returns |true| if a row as well as a
//! column has been finished.
//!
//! @p function fin_col:boolean;
//! label exit;
//! var p:pointer; {the alignrecord after the current one}
//! @!q,@!r:pointer; {temporary pointers for list manipulation}
//! @!s:pointer; {a new span node}
//! @!u:pointer; {a new unset box}
//! @!w:scaled; {natural width}
//! @!o:glue_ord; {order of infinity}
//! @!n:halfword; {span counter}
//! begin if cur_align=null then confusion("endv");
//! q:=link(cur_align);@+if q=null then confusion("endv");
//! @:this can't happen endv}{\quad endv@>
//! if align_state<500000 then
//!   fatal_error("(interwoven alignment preambles are not allowed)");
//! @.interwoven alignment preambles...@>
//! p:=link(q);
//! @<If the preamble list has been traversed, check that the row has ended@>;
//! if extra_info(cur_align)<>span_code then
//!   begin unsave; new_save_level(align_group);@/
//!   @<Package an unset box for the current column and record its width@>;
//!   @<Copy the tabskip glue between columns@>;
//!   if extra_info(cur_align)>=cr_code then
//!     begin fin_col:=true; return;
//!     end;
//!   init_span(p);
//!   end;
//! align_state:=1000000; @<Get the next non-blank non-call token@>;
//! cur_align:=p;
//! init_col; fin_col:=false;
//! exit: end;
//!
//! @ @<If the preamble list has been traversed, check that the row has ended@>=
//! if (p=null)and(extra_info(cur_align)<cr_code) then
//!  if cur_loop<>null then @<Lengthen the preamble periodically@>
//!  else  begin print_err("Extra alignment tab has been changed to ");
//! @.Extra alignment tab...@>
//!   print_esc("cr");
//!   help3("You have given more \span or & marks than there were")@/
//!   ("in the preamble to the \halign or \valign now in progress.")@/
//!   ("So I'll assume that you meant to type \cr instead.");
//!   extra_info(cur_align):=cr_code; error;
//!   end
//!
//! @ @<Lengthen the preamble...@>=
//! begin link(q):=new_null_box; p:=link(q); {a new alignrecord}
//! info(p):=end_span; width(p):=null_flag; cur_loop:=link(cur_loop);
//! @<Copy the templates from node |cur_loop| into node |p|@>;
//! cur_loop:=link(cur_loop);
//! link(p):=new_glue(glue_ptr(cur_loop));
//! end
//!
//! @ @<Copy the templates from node |cur_loop| into node |p|@>=
//! q:=hold_head; r:=u_part(cur_loop);
//! while r<>null do
//!   begin link(q):=get_avail; q:=link(q); info(q):=info(r); r:=link(r);
//!   end;
//! link(q):=null; u_part(p):=link(hold_head);
//! q:=hold_head; r:=v_part(cur_loop);
//! while r<>null do
//!   begin link(q):=get_avail; q:=link(q); info(q):=info(r); r:=link(r);
//!   end;
//! link(q):=null; v_part(p):=link(hold_head)
//!
//! @ @<Copy the tabskip glue...@>=
//! tail_append(new_glue(glue_ptr(link(cur_align))));
//! subtype(tail):=tab_skip_code+1
//!
//! @ @<Package an unset...@>=
//! begin if mode=-hmode then
//!   begin adjust_tail:=cur_tail; u:=hpack(link(head),natural); w:=width(u);
//!   cur_tail:=adjust_tail; adjust_tail:=null;
//!   end
//! else  begin u:=vpackage(link(head),natural,0); w:=height(u);
//!   end;
//! n:=min_quarterword; {this represents a span count of 1}
//! if cur_span<>cur_align then @<Update width entry for spanned columns@>
//! else if w>width(cur_align) then width(cur_align):=w;
//! type(u):=unset_node; span_count(u):=n;@/
//! @<Determine the stretch order@>;
//! glue_order(u):=o; glue_stretch(u):=total_stretch[o];@/
//! @<Determine the shrink order@>;
//! glue_sign(u):=o; glue_shrink(u):=total_shrink[o];@/
//! pop_nest; link(tail):=u; tail:=u;
//! end
//!
//! @ A span node is a 2-word record containing |width|, |info|, and |link|
//! fields. The |link| field is not really a link, it indicates the number of
//! spanned columns; the |info| field points to a span node for the same
//! starting column, having a greater extent of spanning, or to |end_span|,
//! which has the largest possible |link| field; the |width| field holds the
//! largest natural width corresponding to a particular set of spanned columns.
//!
//! A list of the maximum widths so far, for spanned columns starting at a
//! given column, begins with the |info| field of the alignrecord for that
//! column.
//!
//! @d span_node_size=2 {number of |mem| words for a span node}
//!
//! @<Initialize the special list heads...@>=
//! link(end_span):=max_quarterword+1; info(end_span):=null;
//!
//! @ @<Update width entry for spanned columns@>=
//! begin q:=cur_span;
//! repeat incr(n); q:=link(link(q));
//! until q=cur_align;
//! if n>max_quarterword then confusion("256 spans"); {this can happen, but won't}
//! @^system dependencies@>
//! @:this can't happen 256 spans}{\quad 256 spans@>
//! q:=cur_span; while link(info(q))<n do q:=info(q);
//! if link(info(q))>n then
//!   begin s:=get_node(span_node_size); info(s):=info(q); link(s):=n;
//!   info(q):=s; width(s):=w;
//!   end
//! else if width(info(q))<w then width(info(q)):=w;
//! end
//!
//! @ At the end of a row, we append an unset box to the current vlist (for
//! \.{\\halign}) or the current hlist (for \.{\\valign}). This unset box
//! contains the unset boxes for the columns, separated by the tabskip glue.
//! Everything will be set later.
//!
//! @p procedure fin_row;
//! var p:pointer; {the new unset box}
//! begin if mode=-hmode then
//!   begin p:=hpack(link(head),natural);
//!   pop_nest; append_to_vlist(p);
//!   if cur_head<>cur_tail then
//!     begin link(tail):=link(cur_head); tail:=cur_tail;
//!     end;
//!   end
//! else  begin p:=vpack(link(head),natural); pop_nest;
//!   link(tail):=p; tail:=p; space_factor:=1000;
//!   end;
//! type(p):=unset_node; glue_stretch(p):=0;
//! if every_cr<>null then begin_token_list(every_cr,every_cr_text);
//! align_peek;
//! end; {note that |glue_shrink(p)=0| since |glue_shrink==shift_amount|}
//!
//! @ Finally, we will reach the end of the alignment, and we can breathe a
//! sigh of relief that memory hasn't overflowed. All the unset boxes will now be
//! set so that the columns line up, taking due account of spanned columns.
//!
//! @p procedure@?do_assignments; forward;@t\2@>@/
//! procedure@?resume_after_display; forward;@t\2@>@/
//! procedure@?build_page; forward;@t\2@>@/
//! procedure fin_align;
//! var @!p,@!q,@!r,@!s,@!u,@!v: pointer; {registers for the list operations}
//! @!t,@!w:scaled; {width of column}
//! @!o:scaled; {shift offset for unset boxes}
//! @!n:halfword; {matching span amount}
//! @!rule_save:scaled; {temporary storage for |overfull_rule|}
//! @!aux_save:memory_word; {temporary storage for |aux|}
//! begin if cur_group<>align_group then confusion("align1");
//! @:this can't happen align}{\quad align@>
//! unsave; {that |align_group| was for individual entries}
//! if cur_group<>align_group then confusion("align0");
//! unsave; {that |align_group| was for the whole alignment}
//! if nest[nest_ptr-1].mode_field=mmode then o:=display_indent
//!   else o:=0;
//! @<Go through the preamble list, determining the column widths and
//!   changing the alignrecords to dummy unset boxes@>;
//! @<Package the preamble list, to determine the actual tabskip glue amounts,
//!   and let |p| point to this prototype box@>;
//! @<Set the glue in all the unset boxes of the current list@>;
//! flush_node_list(p); pop_alignment;
//! @<Insert the \(c)current list into its environment@>;
//! end;@/
//! @t\4@>@<Declare the procedure called |align_peek|@>
//!
//! @ It's time now to dismantle the preamble list and to compute the column
//! widths. Let $w_{ij}$ be the maximum of the natural widths of all entries
//! that span columns $i$ through $j$, inclusive. The alignrecord for column~$i$
//! contains $w_{ii}$ in its |width| field, and there is also a linked list of
//! the nonzero $w_{ij}$ for increasing $j$, accessible via the |info| field;
//! these span nodes contain the value $j-i+|min_quarterword|$ in their
//! |link| fields. The values of $w_{ii}$ were initialized to |null_flag|, which
//! we regard as $-\infty$.
//!
//! The final column widths are defined by the formula
//! $$w_j=\max_{1\L i\L j}\biggl( w_{ij}-\sum_{i\L k<j}(t_k+w_k)\biggr),$$
//! where $t_k$ is the natural width of the tabskip glue between columns
//! $k$ and~$k+1$. However, if $w_{ij}=-\infty$ for all |i| in the range
//! |1<=i<=j| (i.e., if every entry that involved column~|j| also involved
//! column~|j+1|), we let $w_j=0$, and we zero out the tabskip glue after
//! column~|j|.
//!
//! \TeX\ computes these values by using the following scheme: First $w_1=w_{11}$.
//! Then replace $w_{2j}$ by $\max(w_{2j},w_{1j}-t_1-w_1)$, for all $j>1$.
//! Then $w_2=w_{22}$. Then replace $w_{3j}$ by $\max(w_{3j},w_{2j}-t_2-w_2)$
//! for all $j>2$; and so on. If any $w_j$ turns out to be $-\infty$, its
//! value is changed to zero and so is the next tabskip.
//!
//! @<Go through the preamble list,...@>=
//! q:=link(preamble);
//! repeat flush_list(u_part(q)); flush_list(v_part(q));
//! p:=link(link(q));
//! if width(q)=null_flag then
//!   @<Nullify |width(q)| and the tabskip glue following this column@>;
//! if info(q)<>end_span then
//!   @<Merge the widths in the span nodes of |q| with those of |p|,
//!     destroying the span nodes of |q|@>;
//! type(q):=unset_node; span_count(q):=min_quarterword; height(q):=0;
//! depth(q):=0; glue_order(q):=normal; glue_sign(q):=normal;
//! glue_stretch(q):=0; glue_shrink(q):=0; q:=p;
//! until q=null
//!
//! @ @<Nullify |width(q)| and the tabskip glue following this column@>=
//! begin width(q):=0; r:=link(q); s:=glue_ptr(r);
//! if s<>zero_glue then
//!   begin add_glue_ref(zero_glue); delete_glue_ref(s);
//!   glue_ptr(r):=zero_glue;
//!   end;
//! end
//!
//! @ Merging of two span-node lists is a typical exercise in the manipulation of
//! linearly linked data structures. The essential invariant in the following
//! |repeat| loop is that we want to dispense with node |r|, in |q|'s list,
//! and |u| is its successor; all nodes of |p|'s list up to and including |s|
//! have been processed, and the successor of |s| matches |r| or precedes |r|
//! or follows |r|, according as |link(r)=n| or |link(r)>n| or |link(r)<n|.
//!
//! @<Merge the widths...@>=
//! begin t:=width(q)+width(glue_ptr(link(q)));
//! r:=info(q); s:=end_span; info(s):=p; n:=min_quarterword+1;
//! repeat width(r):=width(r)-t; u:=info(r);
//! while link(r)>n do
//!   begin s:=info(s); n:=link(info(s))+1;
//!   end;
//! if link(r)<n then
//!   begin info(r):=info(s); info(s):=r; decr(link(r)); s:=r;
//!   end
//! else  begin if width(r)>width(info(s)) then width(info(s)):=width(r);
//!   free_node(r,span_node_size);
//!   end;
//! r:=u;
//! until r=end_span;
//! end
//!
//! @ Now the preamble list has been converted to a list of alternating unset
//! boxes and tabskip glue, where the box widths are equal to the final
//! column sizes. In case of \.{\\valign}, we change the widths to heights,
//! so that a correct error message will be produced if the alignment is
//! overfull or underfull.
//!
//! @<Package the preamble list...@>=
//! save_ptr:=save_ptr-2; pack_begin_line:=-mode_line;
//! if mode=-vmode then
//!   begin rule_save:=overfull_rule;
//!   overfull_rule:=0; {prevent rule from being packaged}
//!   p:=hpack(preamble,saved(1),saved(0)); overfull_rule:=rule_save;
//!   end
//! else  begin q:=link(preamble);
//!   repeat height(q):=width(q); width(q):=0; q:=link(link(q));
//!   until q=null;
//!   p:=vpack(preamble,saved(1),saved(0));
//!   q:=link(preamble);
//!   repeat width(q):=height(q); height(q):=0; q:=link(link(q));
//!   until q=null;
//!   end;
//! pack_begin_line:=0
//!
//! @ @<Set the glue in all the unset...@>=
//! q:=link(head); s:=head;
//! while q<>null do
//!   begin if not is_char_node(q) then
//!     if type(q)=unset_node then
//!       @<Set the unset box |q| and the unset boxes in it@>
//!     else if type(q)=rule_node then
//!       @<Make the running dimensions in rule |q| extend to the
//!         boundaries of the alignment@>;
//!   s:=q; q:=link(q);
//!   end
//!
//! @ @<Make the running dimensions in rule |q| extend...@>=
//! begin if is_running(width(q)) then width(q):=width(p);
//! if is_running(height(q)) then height(q):=height(p);
//! if is_running(depth(q)) then depth(q):=depth(p);
//! if o<>0 then
//!   begin r:=link(q); link(q):=null; q:=hpack(q,natural);
//!   shift_amount(q):=o; link(q):=r; link(s):=q;
//!   end;
//! end
//!
//! @ The unset box |q| represents a row that contains one or more unset boxes,
//! depending on how soon \.{\\cr} occurred in that row.
//!
//! @<Set the unset box |q| and the unset boxes in it@>=
//! begin if mode=-vmode then
//!   begin type(q):=hlist_node; width(q):=width(p);
//!   end
//! else  begin type(q):=vlist_node; height(q):=height(p);
//!   end;
//! glue_order(q):=glue_order(p); glue_sign(q):=glue_sign(p);
//! glue_set(q):=glue_set(p); shift_amount(q):=o;
//! r:=link(list_ptr(q)); s:=link(list_ptr(p));
//! repeat @<Set the glue in node |r| and change it from an unset node@>;
//! r:=link(link(r)); s:=link(link(s));
//! until r=null;
//! end
//!
//! @ A box made from spanned columns will be followed by tabskip glue nodes and
//! by empty boxes as if there were no spanning. This permits perfect alignment
//! of subsequent entries, and it prevents values that depend on floating point
//! arithmetic from entering into the dimensions of any boxes.
//!
//! @<Set the glue in node |r|...@>=
//! n:=span_count(r); t:=width(s); w:=t; u:=hold_head;
//! while n>min_quarterword do
//!   begin decr(n);
//!   @<Append tabskip glue and an empty box to list |u|,
//!     and update |s| and |t| as the prototype nodes are passed@>;
//!   end;
//! if mode=-vmode then
//!   @<Make the unset node |r| into an |hlist_node| of width |w|,
//!     setting the glue as if the width were |t|@>
//! else @<Make the unset node |r| into a |vlist_node| of height |w|,
//!     setting the glue as if the height were |t|@>;
//! shift_amount(r):=0;
//! if u<>hold_head then {append blank boxes to account for spanned nodes}
//!   begin link(u):=link(r); link(r):=link(hold_head); r:=u;
//!   end
//!
//! @ @<Append tabskip glue and an empty box to list |u|...@>=
//! s:=link(s); v:=glue_ptr(s); link(u):=new_glue(v); u:=link(u);
//! subtype(u):=tab_skip_code+1; t:=t+width(v);
//! if glue_sign(p)=stretching then
//!   begin if stretch_order(v)=glue_order(p) then
//!     t:=t+round(float(glue_set(p))*stretch(v));
//! @^real multiplication@>
//!   end
//! else if glue_sign(p)=shrinking then
//!   begin if shrink_order(v)=glue_order(p) then
//!     t:=t-round(float(glue_set(p))*shrink(v));
//!   end;
//! s:=link(s); link(u):=new_null_box; u:=link(u); t:=t+width(s);
//! if mode=-vmode then width(u):=width(s)@+else
//!   begin type(u):=vlist_node; height(u):=width(s);
//!   end
//!
//! @ @<Make the unset node |r| into an |hlist_node| of width |w|...@>=
//! begin height(r):=height(q); depth(r):=depth(q);
//! if t=width(r) then
//!   begin glue_sign(r):=normal; glue_order(r):=normal;
//!   set_glue_ratio_zero(glue_set(r));
//!   end
//! else if t>width(r) then
//!   begin glue_sign(r):=stretching;
//!   if glue_stretch(r)=0 then set_glue_ratio_zero(glue_set(r))
//!   else glue_set(r):=unfloat((t-width(r))/glue_stretch(r));
//! @^real division@>
//!   end
//! else  begin glue_order(r):=glue_sign(r); glue_sign(r):=shrinking;
//!   if glue_shrink(r)=0 then set_glue_ratio_zero(glue_set(r))
//!   else if (glue_order(r)=normal)and(width(r)-t>glue_shrink(r)) then
//!     set_glue_ratio_one(glue_set(r))
//!   else glue_set(r):=unfloat((width(r)-t)/glue_shrink(r));
//!   end;
//! width(r):=w; type(r):=hlist_node;
//! end
//!
//! @ @<Make the unset node |r| into a |vlist_node| of height |w|...@>=
//! begin width(r):=width(q);
//! if t=height(r) then
//!   begin glue_sign(r):=normal; glue_order(r):=normal;
//!   set_glue_ratio_zero(glue_set(r));
//!   end
//! else if t>height(r) then
//!   begin glue_sign(r):=stretching;
//!   if glue_stretch(r)=0 then set_glue_ratio_zero(glue_set(r))
//!   else glue_set(r):=unfloat((t-height(r))/glue_stretch(r));
//! @^real division@>
//!   end
//! else  begin glue_order(r):=glue_sign(r); glue_sign(r):=shrinking;
//!   if glue_shrink(r)=0 then set_glue_ratio_zero(glue_set(r))
//!   else if (glue_order(r)=normal)and(height(r)-t>glue_shrink(r)) then
//!     set_glue_ratio_one(glue_set(r))
//!   else glue_set(r):=unfloat((height(r)-t)/glue_shrink(r));
//!   end;
//! height(r):=w; type(r):=vlist_node;
//! end
//!
//! @ We now have a completed alignment, in the list that starts at |head|
//! and ends at |tail|. This list will be merged with the one that encloses
//! it. (In case the enclosing mode is |mmode|, for displayed formulas,
//! we will need to insert glue before and after the display; that part of the
//! program will be deferred until we're more familiar with such operations.)
//!
//! In restricted horizontal mode, the |clang| part of |aux| is undefined;
//! an over-cautious \PASCAL\ runtime system may complain about this.
//! @^dirty \PASCAL@>
//!
//! @<Insert the \(c)current list into its environment@>=
//! aux_save:=aux; p:=link(head); q:=tail; pop_nest;
//! if mode=mmode then @<Finish an alignment in a display@>
//! else  begin aux:=aux_save; link(tail):=p;
//!   if p<>null then tail:=q;
//!   if mode=vmode then build_page;
//!   end
//!
