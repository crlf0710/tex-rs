//! @ The first task is to move the list from |head| to |temp_head| and go
//! into the enclosing semantic level. We also append the \.{\\parfillskip}
//! glue to the end of the paragraph, removing a space (or other glue node) if
//! it was there, since spaces usually precede blank lines and instances of
//! `\.{\$\$}'. The |par_fill_skip| is preceded by an infinite penalty, so
//! it will never be considered as a potential breakpoint.
//!
//! This code assumes that a |glue_node| and a |penalty_node| occupy the
//! same number of |mem|~words.
//! @^data structure assumptions@>
//!
//! @<Get ready to start...@>=
//! link(temp_head):=link(head);
//! if is_char_node(tail) then tail_append(new_penalty(inf_penalty))
//! else if type(tail)<>glue_node then tail_append(new_penalty(inf_penalty))
//! else  begin type(tail):=penalty_node; delete_glue_ref(glue_ptr(tail));
//!   flush_node_list(leader_ptr(tail)); penalty(tail):=inf_penalty;
//!   end;
//! link(tail):=new_param_glue(par_fill_skip_code);
//! init_cur_lang:=prev_graf mod @'200000;
//! init_l_hyf:=prev_graf div @'20000000;
//! init_r_hyf:=(prev_graf div @'200000) mod @'100;
//! pop_nest;
//!
//! @ When looking for optimal line breaks, \TeX\ creates a ``break node'' for
//! each break that is {\sl feasible}, in the sense that there is a way to end
//! a line at the given place without requiring any line to stretch more than
//! a given tolerance. A break node is characterized by three things: the position
//! of the break (which is a pointer to a |glue_node|, |math_node|, |penalty_node|,
//! or |disc_node|); the ordinal number of the line that will follow this
//! breakpoint; and the fitness classification of the line that has just
//! ended, i.e., |tight_fit|, |decent_fit|, |loose_fit|, or |very_loose_fit|.
//!
//! @d tight_fit=3 {fitness classification for lines shrinking 0.5 to 1.0 of their
//!   shrinkability}
//! @d loose_fit=1 {fitness classification for lines stretching 0.5 to 1.0 of their
//!   stretchability}
//! @d very_loose_fit=0 {fitness classification for lines stretching more than
//!   their stretchability}
//! @d decent_fit=2 {fitness classification for all other lines}
//!
//! @ The algorithm essentially determines the best possible way to achieve
//! each feasible combination of position, line, and fitness. Thus, it answers
//! questions like, ``What is the best way to break the opening part of the
//! paragraph so that the fourth line is a tight line ending at such-and-such
//! a place?'' However, the fact that all lines are to be the same length
//! after a certain point makes it possible to regard all sufficiently large
//! line numbers as equivalent, when the looseness parameter is zero, and this
//! makes it possible for the algorithm to save space and time.
//!
//! An ``active node'' and a ``passive node'' are created in |mem| for each
//! feasible breakpoint that needs to be considered. Active nodes are three
//! words long and passive nodes are two words long. We need active nodes only
//! for breakpoints near the place in the paragraph that is currently being
//! examined, so they are recycled within a comparatively short time after
//! they are created.
//!
