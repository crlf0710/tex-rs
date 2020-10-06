//! @* \[16] The semantic nest.
//! \TeX\ is typically in the midst of building many lists at once. For example,
//! when a math formula is being processed, \TeX\ is in math mode and
//! working on an mlist; this formula has temporarily interrupted \TeX\ from
//! being in horizontal mode and building the hlist of a paragraph; and this
//! paragraph has temporarily interrupted \TeX\ from being in vertical mode
//! and building the vlist for the next page of a document. Similarly, when a
//! \.{\\vbox} occurs inside of an \.{\\hbox}, \TeX\ is temporarily
//! interrupted from working in restricted horizontal mode, and it enters
//! internal vertical mode.  The ``semantic nest'' is a stack that
//! keeps track of what lists and modes are currently suspended.
//!
//! At each level of processing we are in one of six modes:
//!
//! \yskip\hang|vmode| stands for vertical mode (the page builder);
//!
//! \hang|hmode| stands for horizontal mode (the paragraph builder);
//!
//! \hang|mmode| stands for displayed formula mode;
//!
//! \hang|-vmode| stands for internal vertical mode (e.g., in a \.{\\vbox});
//!
//! \hang|-hmode| stands for restricted horizontal mode (e.g., in an \.{\\hbox});
//!
//! \hang|-mmode| stands for math formula mode (not displayed).
//!
//! \yskip\noindent The mode is temporarily set to zero while processing \.{\\write}
//! texts in the |ship_out| routine.
//!
//! Numeric values are assigned to |vmode|, |hmode|, and |mmode| so that
//! \TeX's ``big semantic switch'' can select the appropriate thing to
//! do by computing the value |abs(mode)+cur_cmd|, where |mode| is the current
//! mode and |cur_cmd| is the current command code.
//!
//! @d vmode=1 {vertical mode}
//! @d hmode=vmode+max_command+1 {horizontal mode}
//! @d mmode=hmode+max_command+1 {math mode}
//!
//! @p procedure print_mode(@!m:integer); {prints the mode represented by |m|}
//! begin if m>0 then
//!   case m div (max_command+1) of
//!   0:print("vertical");
//!   1:print("horizontal");
//!   2:print("display math");
//!   end
//! else if m=0 then print("no")
//! else  case (-m) div (max_command+1) of
//!   0:print("internal vertical");
//!   1:print("restricted horizontal");
//!   2:print("math");
//!   end;
//! print(" mode");
//! end;
//!
//! @ The state of affairs at any semantic level can be represented by
//! five values:
//!
//! \yskip\hang|mode| is the number representing the semantic mode, as
//! just explained.
//!
//! \yskip\hang|head| is a |pointer| to a list head for the list being built;
//! |link(head)| therefore points to the first element of the list, or
//! to |null| if the list is empty.
//!
//! \yskip\hang|tail| is a |pointer| to the final node of the list being
//! built; thus, |tail=head| if and only if the list is empty.
//!
//! \yskip\hang|prev_graf| is the number of lines of the current paragraph that
//! have already been put into the present vertical list.
//!
//! \yskip\hang|aux| is an auxiliary |memory_word| that gives further information
//! that is needed to characterize the situation.
//!
//! \yskip\noindent
//! In vertical mode, |aux| is also known as |prev_depth|; it is the scaled
//! value representing the depth of the previous box, for use in baseline
//! calculations, or it is |<=-1000|pt if the next box on the vertical list is to
//! be exempt from baseline calculations.  In horizontal mode, |aux| is also
//! known as |space_factor| and |clang|; it holds the current space factor used in
//! spacing calculations, and the current language used for hyphenation.
//! (The value of |clang| is undefined in restricted horizontal mode.)
//! In math mode, |aux| is also known as |incompleat_noad|; if
//! not |null|, it points to a record that represents the numerator of a
//! generalized fraction for which the denominator is currently being formed
//! in the current list.
//!
//! There is also a sixth quantity, |mode_line|, which correlates
//! the semantic nest with the user's input; |mode_line| contains the source
//! line number at which the current level of nesting was entered. The negative
//! of this line number is the |mode_line| at the level of the
//! user's output routine.
//!
//! In horizontal mode, the |prev_graf| field is used for initial language data.
//!
//! The semantic nest is an array called |nest| that holds the |mode|, |head|,
//! |tail|, |prev_graf|, |aux|, and |mode_line| values for all semantic levels
//! below the currently active one. Information about the currently active
//! level is kept in the global quantities |mode|, |head|, |tail|, |prev_graf|,
//! |aux|, and |mode_line|, which live in a \PASCAL\ record that is ready to
//! be pushed onto |nest| if necessary.
//!
//! @d ignore_depth==-65536000 {|prev_depth| value that is ignored}
//!
//! @<Types...@>=
//! @!list_state_record=record@!mode_field:-mmode..mmode;@+
//!   @!head_field,@!tail_field: pointer;
//!   @!pg_field,@!ml_field: integer;@+
//!   @!aux_field: memory_word;
//!   end;
//!
//! @ @d mode==cur_list.mode_field {current mode}
//! @d head==cur_list.head_field {header node of current list}
//! @d tail==cur_list.tail_field {final node on current list}
//! @d prev_graf==cur_list.pg_field {number of paragraph lines accumulated}
//! @d aux==cur_list.aux_field {auxiliary data about the current list}
//! @d prev_depth==aux.sc {the name of |aux| in vertical mode}
//! @d space_factor==aux.hh.lh {part of |aux| in horizontal mode}
//! @d clang==aux.hh.rh {the other part of |aux| in horizontal mode}
//! @d incompleat_noad==aux.int {the name of |aux| in math mode}
//! @d mode_line==cur_list.ml_field {source file line number at beginning of list}
//!
//! @<Glob...@>=
//! @!nest:array[0..nest_size] of list_state_record;
//! @!nest_ptr:0..nest_size; {first unused location of |nest|}
//! @!max_nest_stack:0..nest_size; {maximum of |nest_ptr| when pushing}
//! @!cur_list:list_state_record; {the ``top'' semantic state}
//! @!shown_mode:-mmode..mmode; {most recent mode shown by \.{\\tracingcommands}}
//!
//! @ Here is a common way to make the current list grow:
//!
//! @d tail_append(#)==begin link(tail):=#; tail:=link(tail);
//!   end
//!
//! @ We will see later that the vertical list at the bottom semantic level is split
//! into two parts; the ``current page'' runs from |page_head| to |page_tail|,
//! and the ``contribution list'' runs from |contrib_head| to |tail| of
//! semantic level zero. The idea is that contributions are first formed in
//! vertical mode, then ``contributed'' to the current page (during which time
//! the page-breaking decisions are made). For now, we don't need to know
//! any more details about the page-building process.
//!
//! @<Set init...@>=
//! nest_ptr:=0; max_nest_stack:=0;
//! mode:=vmode; head:=contrib_head; tail:=contrib_head;
//! prev_depth:=ignore_depth; mode_line:=0;
//! prev_graf:=0; shown_mode:=0;
//! @<Start a new current page@>;
//!
//! @ When \TeX's work on one level is interrupted, the state is saved by
//! calling |push_nest|. This routine changes |head| and |tail| so that
//! a new (empty) list is begun; it does not change |mode| or |aux|.
//!
//! @p procedure push_nest; {enter a new semantic level, save the old}
//! begin if nest_ptr>max_nest_stack then
//!   begin max_nest_stack:=nest_ptr;
//!   if nest_ptr=nest_size then overflow("semantic nest size",nest_size);
//! @:TeX capacity exceeded semantic nest size}{\quad semantic nest size@>
//!   end;
//! nest[nest_ptr]:=cur_list; {stack the record}
//! incr(nest_ptr); head:=get_avail; tail:=head; prev_graf:=0; mode_line:=line;
//! end;
//!
//! @ Conversely, when \TeX\ is finished on the current level, the former
//! state is restored by calling |pop_nest|. This routine will never be
//! called at the lowest semantic level, nor will it be called unless |head|
//! is a node that should be returned to free memory.
//!
//! @p procedure pop_nest; {leave a semantic level, re-enter the old}
//! begin free_avail(head); decr(nest_ptr); cur_list:=nest[nest_ptr];
//! end;
//!
//! @ Here is a procedure that displays what \TeX\ is working on, at all levels.
//!
//! @p procedure@?print_totals; forward;@t\2@>
//! procedure show_activities;
//! var p:0..nest_size; {index into |nest|}
//! @!m:-mmode..mmode; {mode}
//! @!a:memory_word; {auxiliary}
//! @!q,@!r:pointer; {for showing the current page}
//! @!t:integer; {ditto}
//! begin nest[nest_ptr]:=cur_list; {put the top level into the array}
//! print_nl(""); print_ln;
//! for p:=nest_ptr downto 0 do
//!   begin m:=nest[p].mode_field; a:=nest[p].aux_field;
//!   print_nl("### "); print_mode(m);
//!   print(" entered at line "); print_int(abs(nest[p].ml_field));
//!   if m=hmode then if nest[p].pg_field <> @'40600000 then
//!     begin print(" (language"); print_int(nest[p].pg_field mod @'200000);
//!     print(":hyphenmin"); print_int(nest[p].pg_field div @'20000000);
//!     print_char(","); print_int((nest[p].pg_field div @'200000) mod @'100);
//!     print_char(")");
//!     end;
//!   if nest[p].ml_field<0 then print(" (\output routine)");
//!   if p=0 then
//!     begin @<Show the status of the current page@>;
//!     if link(contrib_head)<>null then
//!       print_nl("### recent contributions:");
//!     end;
//!   show_box(link(nest[p].head_field));
//!   @<Show the auxiliary field, |a|@>;
//!   end;
//! end;
//!
//! @ @<Show the auxiliary...@>=
//! case abs(m) div (max_command+1) of
//! 0: begin print_nl("prevdepth ");
//!   if a.sc<=ignore_depth then print("ignored")
//!   else print_scaled(a.sc);
//!   if nest[p].pg_field<>0 then
//!     begin print(", prevgraf ");
//!     print_int(nest[p].pg_field); print(" line");
//!     if nest[p].pg_field<>1 then print_char("s");
//!     end;
//!   end;
//! 1: begin print_nl("spacefactor "); print_int(a.hh.lh);
//!   if m>0 then@+ if a.hh.rh>0 then
//!     begin print(", current language "); print_int(a.hh.rh);@+
//!     end;
//!   end;
//! 2: if a.int<>null then
//!   begin print("this will be denominator of:"); show_box(a.int);@+
//!   end;
//! end {there are no other cases}
//!
