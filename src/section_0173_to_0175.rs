//! @* \[12] Displaying boxes.
//! We can reinforce our knowledge of the data structures just introduced
//! by considering two procedures that display a list in symbolic form.
//! The first of these, called |short_display|, is used in ``overfull box''
//! messages to give the top-level description of a list. The other one,
//! called |show_node_list|, prints a detailed description of exactly what
//! is in the data structure.
//!
//! The philosophy of |short_display| is to ignore the fine points about exactly
//! what is inside boxes, except that ligatures and discretionary breaks are
//! expanded. As a result, |short_display| is a recursive procedure, but the
//! recursion is never more than one level deep.
//! @^recursion@>
//!
//! A global variable |font_in_short_display| keeps track of the font code that
//! is assumed to be present when |short_display| begins; deviations from this
//! font will be printed.
//!
//! @<Glob...@>=
//! @!font_in_short_display:integer; {an internal font number}
//!
//! @ Boxes, rules, inserts, whatsits, marks, and things in general that are
//! sort of ``complicated'' are indicated only by printing `\.{[]}'.
//!
//! @p procedure short_display(@!p:integer); {prints highlights of list |p|}
//! var n:integer; {for replacement counts}
//! begin while p>mem_min do
//!   begin if is_char_node(p) then
//!     begin if p<=mem_end then
//!       begin if font(p)<>font_in_short_display then
//!         begin if (font(p)<font_base)or(font(p)>font_max) then
//!           print_char("*")
//! @.*\relax@>
//!         else @<Print the font identifier for |font(p)|@>;
//!         print_char(" "); font_in_short_display:=font(p);
//!         end;
//!       print_ASCII(qo(character(p)));
//!       end;
//!     end
//!   else @<Print a short indication of the contents of node |p|@>;
//!   p:=link(p);
//!   end;
//! end;
//!
//! @ @<Print a short indication of the contents of node |p|@>=
//! case type(p) of
//! hlist_node,vlist_node,ins_node,whatsit_node,mark_node,adjust_node,
//!   unset_node: print("[]");
//! rule_node: print_char("|");
//! glue_node: if glue_ptr(p)<>zero_glue then print_char(" ");
//! math_node: print_char("$");
//! ligature_node: short_display(lig_ptr(p));
//! disc_node: begin short_display(pre_break(p));
//!   short_display(post_break(p));@/
//!   n:=replace_count(p);
//!   while n>0 do
//!     begin if link(p)<>null then p:=link(p);
//!     decr(n);
//!     end;
//!   end;
//! othercases do_nothing
//! endcases
//!
