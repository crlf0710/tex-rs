//! @ Here we save memory space in a common case.
//!
//! @<Simplify a trivial box@>=
//! q:=list_ptr(x);
//! if is_char_node(q) then
//!   begin r:=link(q);
//!   if r<>null then if link(r)=null then if not is_char_node(r) then
//!    if type(r)=kern_node then {unneeded italic correction}
//!     begin free_node(r,small_node_size); link(q):=null;
//!     end;
//!   end
//!
//! @ It is convenient to have a procedure that converts a |math_char|
//! field to an ``unpacked'' form. The |fetch| routine sets |cur_f|, |cur_c|,
//! and |cur_i| to the font code, character code, and character information bytes of
//! a given noad field. It also takes care of issuing error messages for
//! nonexistent characters; in such cases, |char_exists(cur_i)| will be |false|
//! after |fetch| has acted, and the field will also have been reset to |empty|.
//!
//! @p procedure fetch(@!a:pointer); {unpack the |math_char| field |a|}
//! begin cur_c:=character(a); cur_f:=fam_fnt(fam(a)+cur_size);
//! if cur_f=null_font then
//!   @<Complain about an undefined family and set |cur_i| null@>
//! else  begin if (qo(cur_c)>=font_bc[cur_f])and(qo(cur_c)<=font_ec[cur_f]) then
//!     cur_i:=char_info(cur_f)(cur_c)
//!   else cur_i:=null_character;
//!   if not(char_exists(cur_i)) then
//!     begin char_warning(cur_f,qo(cur_c));
//!     math_type(a):=empty; cur_i:=null_character;
//!     end;
//!   end;
//! end;
//!
//! @ @<Complain about an undefined family...@>=
//! begin print_err(""); print_size(cur_size); print_char(" ");
//! print_int(fam(a)); print(" is undefined (character ");
//! print_ASCII(qo(cur_c)); print_char(")");
//! help4("Somewhere in the math formula just ended, you used the")@/
//! ("stated character from an undefined font family. For example,")@/
//! ("plain TeX doesn't allow \it or \sl in subscripts. Proceed,")@/
//! ("and I'll try to forget that I needed that character.");
//! error; cur_i:=null_character; math_type(a):=empty;
//! end
//!
//! @ The outputs of |fetch| are placed in global variables.
//!
//! @<Glob...@>=
//! @!cur_f:internal_font_number; {the |font| field of a |math_char|}
//! @!cur_c:quarterword; {the |character| field of a |math_char|}
//! @!cur_i:four_quarters; {the |char_info| of a |math_char|,
//!   or a lig/kern instruction}
//!
//! @ We need to do a lot of different things, so |mlist_to_hlist| makes two
//! passes over the given mlist.
//!
//! The first pass does most of the processing: It removes ``mu'' spacing from
//! glue, it recursively evaluates all subsidiary mlists so that only the
//! top-level mlist remains to be handled, it puts fractions and square roots
//! and such things into boxes, it attaches subscripts and superscripts, and
//! it computes the overall height and depth of the top-level mlist so that
//! the size of delimiters for a |left_noad| and a |right_noad| will be known.
//! The hlist resulting from each noad is recorded in that noad's |new_hlist|
//! field, an integer field that replaces the |nucleus| or |thickness|.
//! @^recursion@>
//!
//! The second pass eliminates all noads and inserts the correct glue and
//! penalties between nodes.
//!
//! @d new_hlist(#)==mem[nucleus(#)].int {the translation of an mlist}
//!
