//! @ When the user's output routine finishes, it has constructed a vlist
//! in internal vertical mode, and \TeX\ will do the following:
//!
//! @<Resume the page builder after an output routine has come to an end@>=
//! begin if (loc<>null) or 
//!  ((token_type<>output_text)and(token_type<>backed_up)) then
//!   @<Recover from an unbalanced output routine@>;
//! end_token_list; {conserve stack space in case more outputs are triggered}
//! end_graf; unsave; output_active:=false; insert_penalties:=0;@/
//! @<Ensure that box 255 is empty after output@>;
//! if tail<>head then {current list goes after heldover insertions}
//!   begin link(page_tail):=link(head);
//!   page_tail:=tail;
//!   end;
//! if link(page_head)<>null then {and both go before heldover contributions}
//!   begin if link(contrib_head)=null then contrib_tail:=page_tail;
//!   link(page_tail):=link(contrib_head);
//!   link(contrib_head):=link(page_head);
//!   link(page_head):=null; page_tail:=page_head;
//!   end;
//! pop_nest; build_page;
//! end
//!
//! @ @<Recover from an unbalanced output routine@>=
//! begin print_err("Unbalanced output routine");
//! @.Unbalanced output routine@>
//! help2("Your sneaky output routine has problematic {'s and/or }'s.")@/
//! ("I can't handle that very well; good luck."); error;
//! repeat get_token;
//! until loc=null;
//! end {loops forever if reading from a file, since |null=min_halfword<=0|}
//!
//! @ @<Ensure that box 255 is empty after output@>=
//! if box(255)<>null then
//!   begin print_err("Output routine didn't use all of ");
//!   print_esc("box"); print_int(255);
//! @.Output routine didn't use...@>
//!   help3("Your \output commands should empty \box255,")@/
//!     ("e.g., by saying `\shipout\box255'.")@/
//!     ("Proceed; I'll discard its present contents.");
//!   box_error(255);
//!   end
//!
