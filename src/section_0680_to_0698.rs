//! @* \[34] Data structures for math mode.
//! When \TeX\ reads a formula that is enclosed between \.\$'s, it constructs an
//! {\sl mlist}, which is essentially a tree structure representing that
//! formula.  An mlist is a linear sequence of items, but we can regard it as
//! a tree structure because mlists can appear within mlists. For example, many
//! of the entries can be subscripted or superscripted, and such ``scripts''
//! are mlists in their own right.
//!
//! An entire formula is parsed into such a tree before any of the actual
//! typesetting is done, because the current style of type is usually not
//! known until the formula has been fully scanned. For example, when the
//! formula `\.{\$a+b \\over c+d\$}' is being read, there is no way to tell
//! that `\.{a+b}' will be in script size until `\.{\\over}' has appeared.
//!
//! During the scanning process, each element of the mlist being built is
//! classified as a relation, a binary operator, an open parenthesis, etc.,
//! or as a construct like `\.{\\sqrt}' that must be built up. This classification
//! appears in the mlist data structure.
//!
//! After a formula has been fully scanned, the mlist is converted to an hlist
//! so that it can be incorporated into the surrounding text. This conversion is
//! controlled by a recursive procedure that decides all of the appropriate
//! styles by a ``top-down'' process starting at the outermost level and working
//! in towards the subformulas. The formula is ultimately pasted together using
//! combinations of horizontal and vertical boxes, with glue and penalty nodes
//! inserted as necessary.
//!
//! An mlist is represented internally as a linked list consisting chiefly
//! of ``noads'' (pronounced ``no-adds''), to distinguish them from the somewhat
//! similar ``nodes'' in hlists and vlists. Certain kinds of ordinary nodes are
//! allowed to appear in mlists together with the noads; \TeX\ tells the difference
//! by means of the |type| field, since a noad's |type| is always greater than
//! that of a node. An mlist does not contain character nodes, hlist nodes, vlist
//! nodes, math nodes, ligature nodes,
//! or unset nodes; in particular, each mlist item appears in the
//! variable-size part of |mem|, so the |type| field is always present.
//!
//! @ Each noad is four or more words long. The first word contains the |type|
//! and |subtype| and |link| fields that are already so familiar to us; the
//! second, third, and fourth words are called the noad's |nucleus|, |subscr|,
//! and |supscr| fields.
//!
//! Consider, for example, the simple formula `\.{\$x\^2\$}', which would be
//! parsed into an mlist containing a single element called an |ord_noad|.
//! The |nucleus| of this noad is a representation of `\.x', the |subscr| is
//! empty, and the |supscr| is a representation of `\.2'.
//!
//! The |nucleus|, |subscr|, and |supscr| fields are further broken into
//! subfields. If |p| points to a noad, and if |q| is one of its principal
//! fields (e.g., |q=subscr(p)|), there are several possibilities for the
//! subfields, depending on the |math_type| of |q|.
//!
//! \yskip\hang|math_type(q)=math_char| means that |fam(q)| refers to one of
//! the sixteen font families, and |character(q)| is the number of a character
//! within a font of that family, as in a character node.
//!
//! \yskip\hang|math_type(q)=math_text_char| is similar, but the character is
//! unsubscripted and unsuperscripted and it is followed immediately by another
//! character from the same font. (This |math_type| setting appears only
//! briefly during the processing; it is used to suppress unwanted italic
//! corrections.)
//!
//! \yskip\hang|math_type(q)=empty| indicates a field with no value (the
//! corresponding attribute of noad |p| is not present).
//!
//! \yskip\hang|math_type(q)=sub_box| means that |info(q)| points to a box
//! node (either an |hlist_node| or a |vlist_node|) that should be used as the
//! value of the field.  The |shift_amount| in the subsidiary box node is the
//! amount by which that box will be shifted downward.
//!
//! \yskip\hang|math_type(q)=sub_mlist| means that |info(q)| points to
//! an mlist; the mlist must be converted to an hlist in order to obtain
//! the value of this field.
//!
//! \yskip\noindent In the latter case, we might have |info(q)=null|. This
//! is not the same as |math_type(q)=empty|; for example, `\.{\$P\_\{\}\$}'
//! and `\.{\$P\$}' produce different results (the former will not have the
//! ``italic correction'' added to the width of |P|, but the ``script skip''
//! will be added).
//!
//! The definitions of subfields given here are evidently wasteful of space,
//! since a halfword is being used for the |math_type| although only three
//! bits would be needed. However, there are hardly ever many noads present at
//! once, since they are soon converted to nodes that take up even more space,
//! so we can afford to represent them in whatever way simplifies the
//! programming.
//!
//! @d noad_size=4 {number of words in a normal noad}
//! @d nucleus(#)==#+1 {the |nucleus| field of a noad}
//! @d supscr(#)==#+2 {the |supscr| field of a noad}
//! @d subscr(#)==#+3 {the |subscr| field of a noad}
//! @d math_type==link {a |halfword| in |mem|}
//! @d fam==font {a |quarterword| in |mem|}
//! @d math_char=1 {|math_type| when the attribute is simple}
//! @d sub_box=2 {|math_type| when the attribute is a box}
//! @d sub_mlist=3 {|math_type| when the attribute is a formula}
//! @d math_text_char=4 {|math_type| when italic correction is dubious}
//!
//! @ Each portion of a formula is classified as Ord, Op, Bin, Rel, Open,
//! Close, Punct, or Inner, for purposes of spacing and line breaking. An
//! |ord_noad|, |op_noad|, |bin_noad|, |rel_noad|, |open_noad|, |close_noad|,
//! |punct_noad|, or |inner_noad| is used to represent portions of the various
//! types. For example, an `\.=' sign in a formula leads to the creation of a
//! |rel_noad| whose |nucleus| field is a representation of an equals sign
//! (usually |fam=0|, |character=@'75|).  A formula preceded by \.{\\mathrel}
//! also results in a |rel_noad|.  When a |rel_noad| is followed by an
//! |op_noad|, say, and possibly separated by one or more ordinary nodes (not
//! noads), \TeX\ will insert a penalty node (with the current |rel_penalty|)
//! just after the formula that corresponds to the |rel_noad|, unless there
//! already was a penalty immediately following; and a ``thick space'' will be
//! inserted just before the formula that corresponds to the |op_noad|.
//!
//! A noad of type |ord_noad|, |op_noad|, \dots, |inner_noad| usually
//! has a |subtype=normal|. The only exception is that an |op_noad| might
//! have |subtype=limits| or |no_limits|, if the normal positioning of
//! limits has been overridden for this operator.
//!
//! @d ord_noad=unset_node+3 {|type| of a noad classified Ord}
//! @d op_noad=ord_noad+1 {|type| of a noad classified Op}
//! @d bin_noad=ord_noad+2 {|type| of a noad classified Bin}
//! @d rel_noad=ord_noad+3 {|type| of a noad classified Rel}
//! @d open_noad=ord_noad+4 {|type| of a noad classified Open}
//! @d close_noad=ord_noad+5 {|type| of a noad classified Close}
//! @d punct_noad=ord_noad+6 {|type| of a noad classified Punct}
//! @d inner_noad=ord_noad+7 {|type| of a noad classified Inner}
//! @d limits=1 {|subtype| of |op_noad| whose scripts are to be above, below}
//! @d no_limits=2 {|subtype| of |op_noad| whose scripts are to be normal}
//!
//! @ A |radical_noad| is five words long; the fifth word is the |left_delimiter|
//! field, which usually represents a square root sign.
//!
//! A |fraction_noad| is six words long; it has a |right_delimiter| field
//! as well as a |left_delimiter|.
//!
//! Delimiter fields are of type |four_quarters|, and they have four subfields
//! called |small_fam|, |small_char|, |large_fam|, |large_char|. These subfields
//! represent variable-size delimiters by giving the ``small'' and ``large''
//! starting characters, as explained in Chapter~17 of {\sl The \TeX book}.
//! @:TeXbook}{\sl The \TeX book@>
//!
//! A |fraction_noad| is actually quite different from all other noads. Not
//! only does it have six words, it has |thickness|, |denominator|, and
//! |numerator| fields instead of |nucleus|, |subscr|, and |supscr|. The
//! |thickness| is a scaled value that tells how thick to make a fraction
//! rule; however, the special value |default_code| is used to stand for the
//! |default_rule_thickness| of the current size. The |numerator| and
//! |denominator| point to mlists that define a fraction; we always have
//! $$\hbox{|math_type(numerator)=math_type(denominator)=sub_mlist|}.$$ The
//! |left_delimiter| and |right_delimiter| fields specify delimiters that will
//! be placed at the left and right of the fraction. In this way, a
//! |fraction_noad| is able to represent all of \TeX's operators \.{\\over},
//! \.{\\atop}, \.{\\above}, \.{\\overwithdelims}, \.{\\atopwithdelims}, and
//!  \.{\\abovewithdelims}.
//!
//! @d left_delimiter(#)==#+4 {first delimiter field of a noad}
//! @d right_delimiter(#)==#+5 {second delimiter field of a fraction noad}
//! @d radical_noad=inner_noad+1 {|type| of a noad for square roots}
//! @d radical_noad_size=5 {number of |mem| words in a radical noad}
//! @d fraction_noad=radical_noad+1 {|type| of a noad for generalized fractions}
//! @d fraction_noad_size=6 {number of |mem| words in a fraction noad}
//! @d small_fam(#)==mem[#].qqqq.b0 {|fam| for ``small'' delimiter}
//! @d small_char(#)==mem[#].qqqq.b1 {|character| for ``small'' delimiter}
//! @d large_fam(#)==mem[#].qqqq.b2 {|fam| for ``large'' delimiter}
//! @d large_char(#)==mem[#].qqqq.b3 {|character| for ``large'' delimiter}
//! @d thickness==width {|thickness| field in a fraction noad}
//! @d default_code==@'10000000000 {denotes |default_rule_thickness|}
//! @d numerator==supscr {|numerator| field in a fraction noad}
//! @d denominator==subscr {|denominator| field in a fraction noad}
//!
//! @ The global variable |empty_field| is set up for initialization of empty
//! fields in new noads. Similarly, |null_delimiter| is for the initialization
//! of delimiter fields.
//!
//! @<Glob...@>=
//! @!empty_field:two_halves;
//! @!null_delimiter:four_quarters;
//!
//! @ @<Set init...@>=
//! empty_field.rh:=empty; empty_field.lh:=null;@/
//! null_delimiter.b0:=0; null_delimiter.b1:=min_quarterword;@/
//! null_delimiter.b2:=0; null_delimiter.b3:=min_quarterword;
//!
//! @ The |new_noad| function creates an |ord_noad| that is completely null.
//!
//! @p function new_noad:pointer;
//! var p:pointer;
//! begin p:=get_node(noad_size);
//! type(p):=ord_noad; subtype(p):=normal;
//! mem[nucleus(p)].hh:=empty_field;
//! mem[subscr(p)].hh:=empty_field;
//! mem[supscr(p)].hh:=empty_field;
//! new_noad:=p;
//! end;
//!
//! @ A few more kinds of noads will complete the set: An |under_noad| has its
//! nucleus underlined; an |over_noad| has it overlined. An |accent_noad| places
//! an accent over its nucleus; the accent character appears as
//! |fam(accent_chr(p))| and |character(accent_chr(p))|. A |vcenter_noad|
//! centers its nucleus vertically with respect to the axis of the formula;
//! in such noads we always have |math_type(nucleus(p))=sub_box|.
//!
//! And finally, we have |left_noad| and |right_noad| types, to implement
//! \TeX's \.{\\left} and \.{\\right}. The |nucleus| of such noads is
//! replaced by a |delimiter| field; thus, for example, `\.{\\left(}' produces
//! a |left_noad| such that |delimiter(p)| holds the family and character
//! codes for all left parentheses. A |left_noad| never appears in an mlist
//! except as the first element, and a |right_noad| never appears in an mlist
//! except as the last element; furthermore, we either have both a |left_noad|
//! and a |right_noad|, or neither one is present. The |subscr| and |supscr|
//! fields are always |empty| in a |left_noad| and a |right_noad|.
//!
//! @d under_noad=fraction_noad+1 {|type| of a noad for underlining}
//! @d over_noad=under_noad+1 {|type| of a noad for overlining}
//! @d accent_noad=over_noad+1 {|type| of a noad for accented subformulas}
//! @d accent_noad_size=5 {number of |mem| words in an accent noad}
//! @d accent_chr(#)==#+4 {the |accent_chr| field of an accent noad}
//! @d vcenter_noad=accent_noad+1 {|type| of a noad for \.{\\vcenter}}
//! @d left_noad=vcenter_noad+1 {|type| of a noad for \.{\\left}}
//! @d right_noad=left_noad+1 {|type| of a noad for \.{\\right}}
//! @d delimiter==nucleus {|delimiter| field in left and right noads}
//! @d scripts_allowed(#)==(type(#)>=ord_noad)and(type(#)<left_noad)
//!
//! @ Math formulas can also contain instructions like \.{\\textstyle} that
//! override \TeX's normal style rules. A |style_node| is inserted into the
//! data structure to record such instructions; it is three words long, so it
//! is considered a node instead of a noad. The |subtype| is either |display_style|
//! or |text_style| or |script_style| or |script_script_style|. The
//! second and third words of a |style_node| are not used, but they are
//! present because a |choice_node| is converted to a |style_node|.
//!
//! \TeX\ uses even numbers 0, 2, 4, 6 to encode the basic styles
//! |display_style|, \dots, |script_script_style|, and adds~1 to get the
//! ``cramped'' versions of these styles. This gives a numerical order that
//! is backwards from the convention of Appendix~G in {\sl The \TeX book\/};
//! i.e., a smaller style has a larger numerical value.
//! @:TeXbook}{\sl The \TeX book@>
//!
//! @d style_node=unset_node+1 {|type| of a style node}
//! @d style_node_size=3 {number of words in a style node}
//! @d display_style=0 {|subtype| for \.{\\displaystyle}}
//! @d text_style=2 {|subtype| for \.{\\textstyle}}
//! @d script_style=4 {|subtype| for \.{\\scriptstyle}}
//! @d script_script_style=6 {|subtype| for \.{\\scriptscriptstyle}}
//! @d cramped=1 {add this to an uncramped style if you want to cramp it}
//!
//! @p function new_style(@!s:small_number):pointer; {create a style node}
//! var p:pointer; {the new node}
//! begin p:=get_node(style_node_size); type(p):=style_node;
//! subtype(p):=s; width(p):=0; depth(p):=0; {the |width| and |depth| are not used}
//! new_style:=p;
//! end;
//!
//! @ Finally, the \.{\\mathchoice} primitive creates a |choice_node|, which
//! has special subfields |display_mlist|, |text_mlist|, |script_mlist|,
//! and |script_script_mlist| pointing to the mlists for each style.
//!
//! @d choice_node=unset_node+2 {|type| of a choice node}
//! @d display_mlist(#)==info(#+1) {mlist to be used in display style}
//! @d text_mlist(#)==link(#+1) {mlist to be used in text style}
//! @d script_mlist(#)==info(#+2) {mlist to be used in script style}
//! @d script_script_mlist(#)==link(#+2) {mlist to be used in scriptscript style}
//!
//! @p function new_choice:pointer; {create a choice node}
//! var p:pointer; {the new node}
//! begin p:=get_node(style_node_size); type(p):=choice_node;
//! subtype(p):=0; {the |subtype| is not used}
//! display_mlist(p):=null; text_mlist(p):=null; script_mlist(p):=null;
//! script_script_mlist(p):=null;
//! new_choice:=p;
//! end;
//!
//! @ Let's consider now the previously unwritten part of |show_node_list|
//! that displays the things that can only be present in mlists; this
//! program illustrates how to access the data structures just defined.
//!
//! In the context of the following program, |p| points to a node or noad that
//! should be displayed, and the current string contains the ``recursion history''
//! that leads to this point. The recursion history consists of a dot for each
//! outer level in which |p| is subsidiary to some node, or in which |p| is
//! subsidiary to the |nucleus| field of some noad; the dot is replaced by
//! `\.\_' or `\.\^' or `\./' or `\.\\' if |p| is descended from the |subscr|
//! or |supscr| or |denominator| or |numerator| fields of noads. For example,
//! the current string would be `\.{.\^.\_/}' if |p| points to the |ord_noad| for
//! |x| in the (ridiculous) formula
//! `\.{\$\\sqrt\{a\^\{\\mathinner\{b\_\{c\\over x+y\}\}\}\}\$}'.
//!
//! @<Cases of |show_node_list| that arise...@>=
//! style_node:print_style(subtype(p));
//! choice_node:@<Display choice node |p|@>;
//! ord_noad,op_noad,bin_noad,rel_noad,open_noad,close_noad,punct_noad,inner_noad,
//!   radical_noad,over_noad,under_noad,vcenter_noad,accent_noad,
//!   left_noad,right_noad:@<Display normal noad |p|@>;
//! fraction_noad:@<Display fraction noad |p|@>;
//!
//! @ Here are some simple routines used in the display of noads.
//!
//! @<Declare procedures needed for displaying the elements of mlists@>=
//! procedure print_fam_and_char(@!p:pointer); {prints family and character}
//! begin print_esc("fam"); print_int(fam(p)); print_char(" ");
//! print_ASCII(qo(character(p)));
//! end;
//! @#
//! procedure print_delimiter(@!p:pointer); {prints a delimiter as 24-bit hex value}
//! var a:integer; {accumulator}
//! begin a:=small_fam(p)*256+qo(small_char(p));
//! a:=a*@"1000+large_fam(p)*256+qo(large_char(p));
//! if a<0 then print_int(a) {this should never happen}
//! else print_hex(a);
//! end;
//!
//! @ The next subroutine will descend to another level of recursion when a
//! subsidiary mlist needs to be displayed. The parameter |c| indicates what
//! character is to become part of the recursion history. An empty mlist is
//! distinguished from a field with |math_type(p)=empty|, because these are
//! not equivalent (as explained above).
//! @^recursion@>
//!
//! @<Declare procedures needed for displaying...@>=
//! procedure@?show_info; forward;@t\2@>@?{|show_node_list(info(temp_ptr))|}
//! procedure print_subsidiary_data(@!p:pointer;@!c:ASCII_code);
//!   {display a noad field}
//! begin if cur_length>=depth_threshold then
//!   begin if math_type(p)<>empty then print(" []");
//!   end
//! else  begin append_char(c); {include |c| in the recursion history}
//!   temp_ptr:=p; {prepare for |show_info| if recursion is needed}
//!   case math_type(p) of
//!   math_char: begin print_ln; print_current_string; print_fam_and_char(p);
//!     end;
//!   sub_box: show_info; {recursive call}
//!   sub_mlist: if info(p)=null then
//!       begin print_ln; print_current_string; print("{}");
//!       end
//!     else show_info; {recursive call}
//!   othercases do_nothing {|empty|}
//!   endcases;@/
//!   flush_char; {remove |c| from the recursion history}
//!   end;
//! end;
//!
//! @ The inelegant introduction of |show_info| in the code above seems better
//! than the alternative of using \PASCAL's strange |forward| declaration for a
//! procedure with parameters. The \PASCAL\ convention about dropping parameters
//! from a post-|forward| procedure is, frankly, so intolerable to the author
//! of \TeX\ that he would rather stoop to communication via a global temporary
//! variable. (A similar stoopidity occurred with respect to |hlist_out| and
//! |vlist_out| above, and it will occur with respect to |mlist_to_hlist| below.)
//! @^Knuth, Donald Ervin@>
//! @:PASCAL}{\PASCAL@>
//!
//! @p procedure show_info; {the reader will kindly forgive this}
//! begin show_node_list(info(temp_ptr));
//! end;
//!
//! @ @<Declare procedures needed for displaying...@>=
//! procedure print_style(@!c:integer);
//! begin case c div 2 of
//! 0: print_esc("displaystyle"); {|display_style=0|}
//! 1: print_esc("textstyle"); {|text_style=2|}
//! 2: print_esc("scriptstyle"); {|script_style=4|}
//! 3: print_esc("scriptscriptstyle"); {|script_script_style=6|}
//! othercases print("Unknown style!")
//! endcases;
//! end;
//!
//! @ @<Display choice node |p|@>=
//! begin print_esc("mathchoice");
//! append_char("D"); show_node_list(display_mlist(p)); flush_char;
//! append_char("T"); show_node_list(text_mlist(p)); flush_char;
//! append_char("S"); show_node_list(script_mlist(p)); flush_char;
//! append_char("s"); show_node_list(script_script_mlist(p)); flush_char;
//! end
//!
//! @ @<Display normal noad |p|@>=
//! begin case type(p) of
//! ord_noad: print_esc("mathord");
//! op_noad: print_esc("mathop");
//! bin_noad: print_esc("mathbin");
//! rel_noad: print_esc("mathrel");
//! open_noad: print_esc("mathopen");
//! close_noad: print_esc("mathclose");
//! punct_noad: print_esc("mathpunct");
//! inner_noad: print_esc("mathinner");
//! over_noad: print_esc("overline");
//! under_noad: print_esc("underline");
//! vcenter_noad: print_esc("vcenter");
//! radical_noad: begin print_esc("radical"); print_delimiter(left_delimiter(p));
//!   end;
//! accent_noad: begin print_esc("accent"); print_fam_and_char(accent_chr(p));
//!   end;
//! left_noad: begin print_esc("left"); print_delimiter(delimiter(p));
//!   end;
//! right_noad: begin print_esc("right"); print_delimiter(delimiter(p));
//!   end;
//! end;
//! if subtype(p)<>normal then
//!   if subtype(p)=limits then print_esc("limits")
//!   else print_esc("nolimits");
//! if type(p)<left_noad then print_subsidiary_data(nucleus(p),".");
//! print_subsidiary_data(supscr(p),"^");
//! print_subsidiary_data(subscr(p),"_");
//! end
//!
//! @ @<Display fraction noad |p|@>=
//! begin print_esc("fraction, thickness ");
//! if thickness(p)=default_code then print("= default")
//! else print_scaled(thickness(p));
//! if (small_fam(left_delimiter(p))<>0)or@+
//!   (small_char(left_delimiter(p))<>min_quarterword)or@|
//!   (large_fam(left_delimiter(p))<>0)or@|
//!   (large_char(left_delimiter(p))<>min_quarterword) then
//!   begin print(", left-delimiter "); print_delimiter(left_delimiter(p));
//!   end;
//! if (small_fam(right_delimiter(p))<>0)or@|
//!   (small_char(right_delimiter(p))<>min_quarterword)or@|
//!   (large_fam(right_delimiter(p))<>0)or@|
//!   (large_char(right_delimiter(p))<>min_quarterword) then
//!   begin print(", right-delimiter "); print_delimiter(right_delimiter(p));
//!   end;
//! print_subsidiary_data(numerator(p),"\");
//! print_subsidiary_data(denominator(p),"/");
//! end
//!
//! @ That which can be displayed can also be destroyed.
//!
//! @<Cases of |flush_node_list| that arise...@>=
//! style_node: begin free_node(p,style_node_size); goto done;
//!   end;
//! choice_node:begin flush_node_list(display_mlist(p));
//!   flush_node_list(text_mlist(p));
//!   flush_node_list(script_mlist(p));
//!   flush_node_list(script_script_mlist(p));
//!   free_node(p,style_node_size); goto done;
//!   end;
//! ord_noad,op_noad,bin_noad,rel_noad,open_noad,close_noad,punct_noad,inner_noad,
//!   radical_noad,over_noad,under_noad,vcenter_noad,accent_noad:@t@>@;@/
//!   begin if math_type(nucleus(p))>=sub_box then
//!     flush_node_list(info(nucleus(p)));
//!   if math_type(supscr(p))>=sub_box then
//!     flush_node_list(info(supscr(p)));
//!   if math_type(subscr(p))>=sub_box then
//!     flush_node_list(info(subscr(p)));
//!   if type(p)=radical_noad then free_node(p,radical_noad_size)
//!   else if type(p)=accent_noad then free_node(p,accent_noad_size)
//!   else free_node(p,noad_size);
//!   goto done;
//!   end;
//! left_noad,right_noad: begin free_node(p,noad_size); goto done;
//!   end;
//! fraction_noad: begin flush_node_list(info(numerator(p)));
//!   flush_node_list(info(denominator(p)));
//!   free_node(p,fraction_noad_size); goto done;
//!   end;
//!
