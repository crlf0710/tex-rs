//! @* \[47] Building boxes and lists.
//! The most important parts of |main_control| are concerned with \TeX's
//! chief mission of box-making. We need to control the activities that put
//! entries on vlists and hlists, as well as the activities that convert
//! those lists into boxes. All of the necessary machinery has already been
//! developed; it remains for us to ``push the buttons'' at the right times.
//!
//! @ As an introduction to these routines, let's consider one of the simplest
//! cases: What happens when `\.{\\hrule}' occurs in vertical mode, or
//! `\.{\\vrule}' in horizontal mode or math mode? The code in |main_control|
//! is short, since the |scan_rule_spec| routine already does most of what is
//! required; thus, there is no need for a special action procedure.
//!
//! Note that baselineskip calculations are disabled after a rule in vertical
//! mode, by setting |prev_depth:=ignore_depth|.
//!
//! @<Cases of |main_control| that build...@>=
//! vmode+hrule,hmode+vrule,mmode+vrule: begin tail_append(scan_rule_spec);
//!   if abs(mode)=vmode then prev_depth:=ignore_depth
//!   else if abs(mode)=hmode then space_factor:=1000;
//!   end;
//!
//! @ The processing of things like \.{\\hskip} and \.{\\vskip} is slightly
//! more complicated. But the code in |main_control| is very short, since
//! it simply calls on the action routine |append_glue|. Similarly, \.{\\kern}
//! activates |append_kern|.
//!
//! @<Cases of |main_control| that build...@>=
//! vmode+vskip,hmode+hskip,mmode+hskip,mmode+mskip: append_glue;
//! any_mode(kern),mmode+mkern: append_kern;
//!
//! @ The |hskip| and |vskip| command codes are used for control sequences
//! like \.{\\hss} and \.{\\vfil} as well as for \.{\\hskip} and \.{\\vskip}.
//! The difference is in the value of |cur_chr|.
//!
//! @d fil_code=0 {identifies \.{\\hfil} and \.{\\vfil}}
//! @d fill_code=1 {identifies \.{\\hfill} and \.{\\vfill}}
//! @d ss_code=2 {identifies \.{\\hss} and \.{\\vss}}
//! @d fil_neg_code=3 {identifies \.{\\hfilneg} and \.{\\vfilneg}}
//! @d skip_code=4 {identifies \.{\\hskip} and \.{\\vskip}}
//! @d mskip_code=5 {identifies \.{\\mskip}}
//!
//! @<Put each...@>=
//! primitive("hskip",hskip,skip_code);@/
//! @!@:hskip_}{\.{\\hskip} primitive@>
//! primitive("hfil",hskip,fil_code);
//! @!@:hfil_}{\.{\\hfil} primitive@>
//! primitive("hfill",hskip,fill_code);@/
//! @!@:hfill_}{\.{\\hfill} primitive@>
//! primitive("hss",hskip,ss_code);
//! @!@:hss_}{\.{\\hss} primitive@>
//! primitive("hfilneg",hskip,fil_neg_code);@/
//! @!@:hfil_neg_}{\.{\\hfilneg} primitive@>
//! primitive("vskip",vskip,skip_code);@/
//! @!@:vskip_}{\.{\\vskip} primitive@>
//! primitive("vfil",vskip,fil_code);
//! @!@:vfil_}{\.{\\vfil} primitive@>
//! primitive("vfill",vskip,fill_code);@/
//! @!@:vfill_}{\.{\\vfill} primitive@>
//! primitive("vss",vskip,ss_code);
//! @!@:vss_}{\.{\\vss} primitive@>
//! primitive("vfilneg",vskip,fil_neg_code);@/
//! @!@:vfil_neg_}{\.{\\vfilneg} primitive@>
//! primitive("mskip",mskip,mskip_code);@/
//! @!@:mskip_}{\.{\\mskip} primitive@>
//! primitive("kern",kern,explicit);
//! @!@:kern_}{\.{\\kern} primitive@>
//! primitive("mkern",mkern,mu_glue);@/
//! @!@:mkern_}{\.{\\mkern} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! hskip: case chr_code of
//!   skip_code:print_esc("hskip");
//!   fil_code:print_esc("hfil");
//!   fill_code:print_esc("hfill");
//!   ss_code:print_esc("hss");
//!   othercases print_esc("hfilneg")
//!   endcases;
//! vskip: case chr_code of
//!   skip_code:print_esc("vskip");
//!   fil_code:print_esc("vfil");
//!   fill_code:print_esc("vfill");
//!   ss_code:print_esc("vss");
//!   othercases print_esc("vfilneg")
//!   endcases;
//! mskip: print_esc("mskip");
//! kern: print_esc("kern");
//! mkern: print_esc("mkern");
//!
//! @ All the work relating to glue creation has been relegated to the
//! following subroutine. It does not call |build_page|, because it is
//! used in at least one place where that would be a mistake.
//!
//! @<Declare action...@>=
//! procedure append_glue;
//! var s:small_number; {modifier of skip command}
//! begin s:=cur_chr;
//! case s of
//! fil_code: cur_val:=fil_glue;
//! fill_code: cur_val:=fill_glue;
//! ss_code: cur_val:=ss_glue;
//! fil_neg_code: cur_val:=fil_neg_glue;
//! skip_code: scan_glue(glue_val);
//! mskip_code: scan_glue(mu_val);
//! end; {now |cur_val| points to the glue specification}
//! tail_append(new_glue(cur_val));
//! if s>=skip_code then
//!   begin decr(glue_ref_count(cur_val));
//!   if s>skip_code then subtype(tail):=mu_glue;
//!   end;
//! end;
//!
//! @ @<Declare act...@>=
//! procedure append_kern;
//! var s:quarterword; {|subtype| of the kern node}
//! begin s:=cur_chr; scan_dimen(s=mu_glue,false,false);
//! tail_append(new_kern(cur_val)); subtype(tail):=s;
//! end;
//!
//! @ Many of the actions related to box-making are triggered by the appearance
//! of braces in the input. For example, when the user says `\.{\\hbox}
//! \.{to} \.{100pt\{$\langle\,\hbox{hlist}\,\rangle$\}}' in vertical mode,
//! the information about the box size (100pt, |exactly|) is put onto |save_stack|
//! with a level boundary word just above it, and |cur_group:=adjusted_hbox_group|;
//! \TeX\ enters restricted horizontal mode to process the hlist. The right
//! brace eventually causes |save_stack| to be restored to its former state,
//! at which time the information about the box size (100pt, |exactly|) is
//! available once again; a box is packaged and we leave restricted horizontal
//! mode, appending the new box to the current list of the enclosing mode
//! (in this case to the current list of vertical mode), followed by any
//! vertical adjustments that were removed from the box by |hpack|.
//!
//! The next few sections of the program are therefore concerned with the
//! treatment of left and right curly braces.
//!
//! @ If a left brace occurs in the middle of a page or paragraph, it simply
//! introduces a new level of grouping, and the matching right brace will not have
//! such a drastic effect. Such grouping affects neither the mode nor the
//! current list.
//!
//! @<Cases of |main_control| that build...@>=
//! non_math(left_brace): new_save_level(simple_group);
//! any_mode(begin_group): new_save_level(semi_simple_group);
//! any_mode(end_group): if cur_group=semi_simple_group then unsave
//!   else off_save;
//!
//! @ We have to deal with errors in which braces and such things are not
//! properly nested. Sometimes the user makes an error of commission by
//! inserting an extra symbol, but sometimes the user makes an error of omission.
//! \TeX\ can't always tell one from the other, so it makes a guess and tries
//! to avoid getting into a loop.
//!
//! The |off_save| routine is called when the current group code is wrong. It tries
//! to insert something into the user's input that will help clean off
//! the top level.
//!
//! @<Declare act...@>=
//! procedure off_save;
//! var p:pointer; {inserted token}
//! begin if cur_group=bottom_level then
//!   @<Drop current token and complain that it was unmatched@>
//! else  begin back_input; p:=get_avail; link(temp_head):=p;
//!   print_err("Missing ");
//!   @<Prepare to insert a token that matches |cur_group|,
//!     and print what it is@>;
//!   print(" inserted"); ins_list(link(temp_head));
//!   help5("I've inserted something that you may have forgotten.")@/
//!   ("(See the <inserted text> above.)")@/
//!   ("With luck, this will get me unwedged. But if you")@/
//!   ("really didn't forget anything, try typing `2' now; then")@/
//!   ("my insertion and my current dilemma will both disappear.");
//!   error;
//!   end;
//! end;
//!
//! @ At this point, |link(temp_head)=p|, a pointer to an empty one-word node.
//!
//! @<Prepare to insert a token that matches |cur_group|...@>=
//! case cur_group of
//! semi_simple_group: begin info(p):=cs_token_flag+frozen_end_group;
//!   print_esc("endgroup");
//! @.Missing \\endgroup inserted@>
//!   end;
//! math_shift_group: begin info(p):=math_shift_token+"$"; print_char("$");
//! @.Missing \$ inserted@>
//!   end;
//! math_left_group: begin info(p):=cs_token_flag+frozen_right; link(p):=get_avail;
//!   p:=link(p); info(p):=other_token+"."; print_esc("right.");
//! @.Missing \\right\hbox{.} inserted@>
//! @^null delimiter@>
//!   end;
//! othercases begin info(p):=right_brace_token+"}"; print_char("}");
//! @.Missing \} inserted@>
//!   end
//! endcases
//!
//! @ @<Drop current token and complain that it was unmatched@>=
//! begin print_err("Extra "); print_cmd_chr(cur_cmd,cur_chr);
//! @.Extra x@>
//! help1("Things are pretty mixed up, but I think the worst is over.");@/
//! error;
//! end
//!
//! @ The routine for a |right_brace| character branches into many subcases,
//! since a variety of things may happen, depending on |cur_group|. Some
//! types of groups are not supposed to be ended by a right brace; error
//! messages are given in hopes of pinpointing the problem. Most branches
//! of this routine will be filled in later, when we are ready to understand
//! them; meanwhile, we must prepare ourselves to deal with such errors.
//!
//! @<Cases of |main_control| that build...@>=
//! any_mode(right_brace): handle_right_brace;
//!
//! @ @<Declare the procedure called |handle_right_brace|@>=
//! procedure handle_right_brace;
//! var p,@!q:pointer; {for short-term use}
//! @!d:scaled; {holds |split_max_depth| in |insert_group|}
//! @!f:integer; {holds |floating_penalty| in |insert_group|}
//! begin case cur_group of
//! simple_group: unsave;
//! bottom_level: begin print_err("Too many }'s");
//! @.Too many \}'s@>
//!   help2("You've closed more groups than you opened.")@/
//!   ("Such booboos are generally harmless, so keep going."); error;
//!   end;
//! semi_simple_group,math_shift_group,math_left_group: extra_right_brace;
//! @t\4@>@<Cases of |handle_right_brace| where a |right_brace| triggers
//!   a delayed action@>@;
//! othercases confusion("rightbrace")
//! @:this can't happen rightbrace}{\quad rightbrace@>
//! endcases;
//! end;
//!
//! @ @<Declare act...@>=
//! procedure extra_right_brace;
//! begin print_err("Extra }, or forgotten ");
//! @.Extra \}, or forgotten x@>
//! case cur_group of
//! semi_simple_group: print_esc("endgroup");
//! math_shift_group: print_char("$");
//! math_left_group: print_esc("right");
//! end;@/
//! help5("I've deleted a group-closing symbol because it seems to be")@/
//! ("spurious, as in `$x}$'. But perhaps the } is legitimate and")@/
//! ("you forgot something else, as in `\hbox{$x}'. In such cases")@/
//! ("the way to recover is to insert both the forgotten and the")@/
//! ("deleted material, e.g., by typing `I$}'."); error;
//! incr(align_state);
//! end;
//!
//! @ Here is where we clear the parameters that are supposed to revert to their
//! default values after every paragraph and when internal vertical mode is entered.
//!
//! @<Declare act...@>=
//! procedure normal_paragraph;
//! begin if looseness<>0 then eq_word_define(int_base+looseness_code,0);
//! if hang_indent<>0 then eq_word_define(dimen_base+hang_indent_code,0);
//! if hang_after<>1 then eq_word_define(int_base+hang_after_code,1);
//! if par_shape_ptr<>null then eq_define(par_shape_loc,shape_ref,null);
//! end;
//!
//! @ Now let's turn to the question of how \.{\\hbox} is treated. We actually
//! need to consider also a slightly larger context, since constructions like
//! `\.{\\setbox3=}\penalty0\.{\\hbox...}' and
//! `\.{\\leaders}\penalty0\.{\\hbox...}' and
//! `\.{\\lower3.8pt\\hbox...}'
//! are supposed to invoke quite
//! different actions after the box has been packaged. Conversely,
//! constructions like `\.{\\setbox3=}' can be followed by a variety of
//! different kinds of boxes, and we would like to encode such things in an
//! efficient way.
//!
//! In other words, there are two problems: to represent the context of a box,
//! and to represent its type.
//!
//! The first problem is solved by putting a ``context code'' on the |save_stack|,
//! just below the two entries that give the dimensions produced by |scan_spec|.
//! The context code is either a (signed) shift amount, or it is a large
//! integer |>=box_flag|, where |box_flag=@t$2^{30}$@>|. Codes |box_flag| through
//! |box_flag+255| represent `\.{\\setbox0}' through `\.{\\setbox255}';
//! codes |box_flag+256| through |box_flag+511| represent `\.{\\global\\setbox0}'
//! through `\.{\\global\\setbox255}';
//! code |box_flag+512| represents `\.{\\shipout}'; and codes |box_flag+513|
//! through |box_flag+515| represent `\.{\\leaders}', `\.{\\cleaders}',
//! and `\.{\\xleaders}'.
//!
//! The second problem is solved by giving the command code |make_box| to all
//! control sequences that produce a box, and by using the following |chr_code|
//! values to distinguish between them: |box_code|, |copy_code|, |last_box_code|,
//! |vsplit_code|, |vtop_code|, |vtop_code+vmode|, and |vtop_code+hmode|, where
//! the latter two are used to denote \.{\\vbox} and \.{\\hbox}, respectively.
//!
//! @d box_flag==@'10000000000 {context code for `\.{\\setbox0}'}
//! @d ship_out_flag==box_flag+512 {context code for `\.{\\shipout}'}
//! @d leader_flag==box_flag+513 {context code for `\.{\\leaders}'}
//! @d box_code=0 {|chr_code| for `\.{\\box}'}
//! @d copy_code=1 {|chr_code| for `\.{\\copy}'}
//! @d last_box_code=2 {|chr_code| for `\.{\\lastbox}'}
//! @d vsplit_code=3 {|chr_code| for `\.{\\vsplit}'}
//! @d vtop_code=4 {|chr_code| for `\.{\\vtop}'}
//!
//! @<Put each...@>=
//! primitive("moveleft",hmove,1);
//! @!@:move_left_}{\.{\\moveleft} primitive@>
//! primitive("moveright",hmove,0);@/
//! @!@:move_right_}{\.{\\moveright} primitive@>
//! primitive("raise",vmove,1);
//! @!@:raise_}{\.{\\raise} primitive@>
//! primitive("lower",vmove,0);
//! @!@:lower_}{\.{\\lower} primitive@>
//! @#
//! primitive("box",make_box,box_code);
//! @!@:box_}{\.{\\box} primitive@>
//! primitive("copy",make_box,copy_code);
//! @!@:copy_}{\.{\\copy} primitive@>
//! primitive("lastbox",make_box,last_box_code);
//! @!@:last_box_}{\.{\\lastbox} primitive@>
//! primitive("vsplit",make_box,vsplit_code);
//! @!@:vsplit_}{\.{\\vsplit} primitive@>
//! primitive("vtop",make_box,vtop_code);@/
//! @!@:vtop_}{\.{\\vtop} primitive@>
//! primitive("vbox",make_box,vtop_code+vmode);
//! @!@:vbox_}{\.{\\vbox} primitive@>
//! primitive("hbox",make_box,vtop_code+hmode);@/
//! @!@:hbox_}{\.{\\hbox} primitive@>
//! primitive("shipout",leader_ship,a_leaders-1); {|ship_out_flag=leader_flag-1|}
//! @!@:ship_out_}{\.{\\shipout} primitive@>
//! primitive("leaders",leader_ship,a_leaders);
//! @!@:leaders_}{\.{\\leaders} primitive@>
//! primitive("cleaders",leader_ship,c_leaders);
//! @!@:c_leaders_}{\.{\\cleaders} primitive@>
//! primitive("xleaders",leader_ship,x_leaders);
//! @!@:x_leaders_}{\.{\\xleaders} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! hmove: if chr_code=1 then print_esc("moveleft")@+else print_esc("moveright");
//! vmove: if chr_code=1 then print_esc("raise")@+else print_esc("lower");
//! make_box: case chr_code of
//!   box_code: print_esc("box");
//!   copy_code: print_esc("copy");
//!   last_box_code: print_esc("lastbox");
//!   vsplit_code: print_esc("vsplit");
//!   vtop_code: print_esc("vtop");
//!   vtop_code+vmode: print_esc("vbox");
//!   othercases print_esc("hbox")
//!   endcases;
//! leader_ship: if chr_code=a_leaders then print_esc("leaders")
//!   else if chr_code=c_leaders then print_esc("cleaders")
//!   else if chr_code=x_leaders then print_esc("xleaders")
//!   else print_esc("shipout");
//!
//! @ Constructions that require a box are started by calling |scan_box| with
//! a specified context code. The |scan_box| routine verifies
//! that a |make_box| command comes next and then it calls |begin_box|.
//!
//! @<Cases of |main_control| that build...@>=
//! vmode+hmove,hmode+vmove,mmode+vmove: begin t:=cur_chr;
//!   scan_normal_dimen;
//!   if t=0 then scan_box(cur_val)@+else scan_box(-cur_val);
//!   end;
//! any_mode(leader_ship): scan_box(leader_flag-a_leaders+cur_chr);
//! any_mode(make_box): begin_box(0);
//!
//! @ The global variable |cur_box| will point to a newly made box. If the box
//! is void, we will have |cur_box=null|. Otherwise we will have
//! |type(cur_box)=hlist_node| or |vlist_node| or |rule_node|; the |rule_node|
//! case can occur only with leaders.
//!
//! @<Glob...@>=
//! @!cur_box:pointer; {box to be placed into its context}
//!
//! @ The |box_end| procedure does the right thing with |cur_box|, if
//! |box_context| represents the context as explained above.
//!
//! @<Declare act...@>=
//! procedure box_end(@!box_context:integer);
//! var p:pointer; {|ord_noad| for new box in math mode}
//! begin if box_context<box_flag then @<Append box |cur_box| to the current list,
//!     shifted by |box_context|@>
//! else if box_context<ship_out_flag then @<Store \(c)|cur_box| in a box register@>
//! else if cur_box<>null then
//!   if box_context>ship_out_flag then @<Append a new leader node that
//!       uses |cur_box|@>
//!   else ship_out(cur_box);
//! end;
//!
//! @ The global variable |adjust_tail| will be non-null if and only if the
//! current box might include adjustments that should be appended to the
//! current vertical list.
//!
//! @<Append box |cur_box| to the current...@>=
//! begin if cur_box<>null then
//!   begin shift_amount(cur_box):=box_context;
//!   if abs(mode)=vmode then
//!     begin append_to_vlist(cur_box);
//!     if adjust_tail<>null then
//!       begin if adjust_head<>adjust_tail then
//!         begin link(tail):=link(adjust_head); tail:=adjust_tail;
//!         end;
//!       adjust_tail:=null;
//!       end;
//!     if mode>0 then build_page;
//!     end
//!   else  begin if abs(mode)=hmode then space_factor:=1000
//!     else  begin p:=new_noad;
//!       math_type(nucleus(p)):=sub_box;
//!       info(nucleus(p)):=cur_box; cur_box:=p;
//!       end;
//!     link(tail):=cur_box; tail:=cur_box;
//!     end;
//!   end;
//! end
//!
//! @ @<Store \(c)|cur_box| in a box register@>=
//! if box_context<box_flag+256 then
//!   eq_define(box_base-box_flag+box_context,box_ref,cur_box)
//! else geq_define(box_base-box_flag-256+box_context,box_ref,cur_box)
//!
//! @ @<Append a new leader node ...@>=
//! begin @<Get the next non-blank non-relax...@>;
//! if ((cur_cmd=hskip)and(abs(mode)<>vmode))or@|
//!    ((cur_cmd=vskip)and(abs(mode)=vmode)) then
//!   begin append_glue; subtype(tail):=box_context-(leader_flag-a_leaders);
//!   leader_ptr(tail):=cur_box;
//!   end
//! else  begin print_err("Leaders not followed by proper glue");
//! @.Leaders not followed by...@>
//!   help3("You should say `\leaders <box or rule><hskip or vskip>'.")@/
//!   ("I found the <box or rule>, but there's no suitable")@/
//!   ("<hskip or vskip>, so I'm ignoring these leaders."); back_error;
//!   flush_node_list(cur_box);
//!   end;
//! end
//!
//! @ Now that we can see what eventually happens to boxes, we can consider
//! the first steps in their creation. The |begin_box| routine is called when
//! |box_context| is a context specification, |cur_chr| specifies the type of
//! box desired, and |cur_cmd=make_box|.
//!
//! @<Declare act...@>=
//! procedure begin_box(@!box_context:integer);
//! label exit, done;
//! var @!p,@!q:pointer; {run through the current list}
//! @!m:quarterword; {the length of a replacement list}
//! @!k:halfword; {0 or |vmode| or |hmode|}
//! @!n:eight_bits; {a box number}
//! begin case cur_chr of
//! box_code: begin scan_eight_bit_int; cur_box:=box(cur_val);
//!   box(cur_val):=null; {the box becomes void, at the same level}
//!   end;
//! copy_code: begin scan_eight_bit_int; cur_box:=copy_node_list(box(cur_val));
//!   end;
//! last_box_code: @<If the current list ends with a box node, delete it from
//!   the list and make |cur_box| point to it; otherwise set |cur_box:=null|@>;
//! vsplit_code: @<Split off part of a vertical box, make |cur_box| point to it@>;
//! othercases @<Initiate the construction of an hbox or vbox, then |return|@>
//! endcases;@/
//! box_end(box_context); {in simple cases, we use the box immediately}
//! exit:end;
//!
//! @ Note that the condition |not is_char_node(tail)| implies that |head<>tail|,
//! since |head| is a one-word node.
//!
//! @<If the current list ends with a box node, delete it...@>=
//! begin cur_box:=null;
//! if abs(mode)=mmode then
//!   begin you_cant; help1("Sorry; this \lastbox will be void."); error;
//!   end
//! else if (mode=vmode)and(head=tail) then
//!   begin you_cant;
//!   help2("Sorry...I usually can't take things from the current page.")@/
//!     ("This \lastbox will therefore be void."); error;
//!   end
//! else  begin if not is_char_node(tail) then
//!     if (type(tail)=hlist_node)or(type(tail)=vlist_node) then
//!       @<Remove the last box, unless it's part of a discretionary@>;
//!   end;
//! end
//!
//! @ @<Remove the last box...@>=
//! begin q:=head;
//! repeat p:=q;
//! if not is_char_node(q) then if type(q)=disc_node then
//!   begin for m:=1 to replace_count(q) do p:=link(p);
//!   if p=tail then goto done;
//!   end;
//! q:=link(p);
//! until q=tail;
//! cur_box:=tail; shift_amount(cur_box):=0;
//! tail:=p; link(p):=null;
//! done:end
//!
//! @ Here we deal with things like `\.{\\vsplit 13 to 100pt}'.
//!
//! @<Split off part of a vertical box, make |cur_box| point to it@>=
//! begin scan_eight_bit_int; n:=cur_val;
//! if not scan_keyword("to") then
//! @.to@>
//!   begin print_err("Missing `to' inserted");
//! @.Missing `to' inserted@>
//!   help2("I'm working on `\vsplit<box number> to <dimen>';")@/
//!   ("will look for the <dimen> next."); error;
//!   end;
//! scan_normal_dimen;
//! cur_box:=vsplit(n,cur_val);
//! end
//!
//! @ Here is where we enter restricted horizontal mode or internal vertical
//! mode, in order to make a box.
//!
//! @<Initiate the construction of an hbox or vbox, then |return|@>=
//! begin k:=cur_chr-vtop_code; saved(0):=box_context;
//! if k=hmode then
//!   if (box_context<box_flag)and(abs(mode)=vmode) then
//!     scan_spec(adjusted_hbox_group,true)
//!   else scan_spec(hbox_group,true)
//! else  begin if k=vmode then scan_spec(vbox_group,true)
//!   else  begin scan_spec(vtop_group,true); k:=vmode;
//!     end;
//!   normal_paragraph;
//!   end;
//! push_nest; mode:=-k;
//! if k=vmode then
//!   begin prev_depth:=ignore_depth;
//!   if every_vbox<>null then begin_token_list(every_vbox,every_vbox_text);
//!   end
//! else  begin space_factor:=1000;
//!   if every_hbox<>null then begin_token_list(every_hbox,every_hbox_text);
//!   end;
//! return;
//! end
//!
//! @ @<Declare act...@>=
//! procedure scan_box(@!box_context:integer);
//!   {the next input should specify a box or perhaps a rule}
//! begin @<Get the next non-blank non-relax...@>;
//! if cur_cmd=make_box then begin_box(box_context)
//! else if (box_context>=leader_flag)and((cur_cmd=hrule)or(cur_cmd=vrule)) then
//!   begin cur_box:=scan_rule_spec; box_end(box_context);
//!   end
//! else  begin@t@>@;@/
//!   print_err("A <box> was supposed to be here");@/
//! @.A <box> was supposed to...@>
//!   help3("I was expecting to see \hbox or \vbox or \copy or \box or")@/
//!   ("something like that. So you might find something missing in")@/
//!   ("your output. But keep trying; you can fix this later."); back_error;
//!   end;
//! end;
//!
//! @ When the right brace occurs at the end of an \.{\\hbox} or \.{\\vbox} or
//! \.{\\vtop} construction, the |package| routine comes into action. We might
//! also have to finish a paragraph that hasn't ended.
//!
//! @<Cases of |handle...@>=
//! hbox_group: package(0);
//! adjusted_hbox_group: begin adjust_tail:=adjust_head; package(0);
//!   end;
//! vbox_group: begin end_graf; package(0);
//!   end;
//! vtop_group: begin end_graf; package(vtop_code);
//!   end;
//!
//! @ @<Declare action...@>=
//! procedure package(@!c:small_number);
//! var h:scaled; {height of box}
//! @!p:pointer; {first node in a box}
//! @!d:scaled; {max depth}
//! begin d:=box_max_depth; unsave; save_ptr:=save_ptr-3;
//! if mode=-hmode then cur_box:=hpack(link(head),saved(2),saved(1))
//! else  begin cur_box:=vpackage(link(head),saved(2),saved(1),d);
//!   if c=vtop_code then @<Readjust the height and depth of |cur_box|,
//!     for \.{\\vtop}@>;
//!   end;
//! pop_nest; box_end(saved(0));
//! end;
//!
//! @ The height of a `\.{\\vtop}' box is inherited from the first item on its list,
//! if that item is an |hlist_node|, |vlist_node|, or |rule_node|; otherwise
//! the \.{\\vtop} height is zero.
//!
//!
//! @<Readjust the height...@>=
//! begin h:=0; p:=list_ptr(cur_box);
//! if p<>null then if type(p)<=rule_node then h:=height(p);
//! depth(cur_box):=depth(cur_box)-h+height(cur_box); height(cur_box):=h;
//! end
//!
//! @ A paragraph begins when horizontal-mode material occurs in vertical mode,
//! or when the paragraph is explicitly started by `\.{\\indent}' or
//! `\.{\\noindent}'.
//!
//! @<Put each...@>=
//! primitive("indent",start_par,1);
//! @!@:indent_}{\.{\\indent} primitive@>
//! primitive("noindent",start_par,0);
//! @!@:no_indent_}{\.{\\noindent} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! start_par: if chr_code=0 then print_esc("noindent")@+ else print_esc("indent");
//!
//! @ @<Cases of |main_control| that build...@>=
//! vmode+start_par: new_graf(cur_chr>0);
//! vmode+letter,vmode+other_char,vmode+char_num,vmode+char_given,
//!    vmode+math_shift,vmode+un_hbox,vmode+vrule,
//!    vmode+accent,vmode+discretionary,vmode+hskip,vmode+valign,
//!    vmode+ex_space,vmode+no_boundary:@t@>@;@/
//!   begin back_input; new_graf(true);
//!   end;
//!
//! @ @<Declare act...@>=
//! function norm_min(@!h:integer):small_number;
//! begin if h<=0 then norm_min:=1@+else if h>=63 then norm_min:=63@+
//! else norm_min:=h;
//! end;
//! @#
//! procedure new_graf(@!indented:boolean);
//! begin prev_graf:=0;
//! if (mode=vmode)or(head<>tail) then
//!   tail_append(new_param_glue(par_skip_code));
//! push_nest; mode:=hmode; space_factor:=1000; set_cur_lang; clang:=cur_lang;
//! prev_graf:=(norm_min(left_hyphen_min)*@'100+norm_min(right_hyphen_min))
//!              *@'200000+cur_lang;
//! if indented then
//!   begin tail:=new_null_box; link(head):=tail; width(tail):=par_indent;@+
//!   end;
//! if every_par<>null then begin_token_list(every_par,every_par_text);
//! if nest_ptr=1 then build_page; {put |par_skip| glue on current page}
//! end;
//!
//! @ @<Cases of |main_control| that build...@>=
//! hmode+start_par,mmode+start_par: indent_in_hmode;
//!
//! @ @<Declare act...@>=
//! procedure indent_in_hmode;
//! var p,@!q:pointer;
//! begin if cur_chr>0 then {\.{\\indent}}
//!   begin p:=new_null_box; width(p):=par_indent;
//!   if abs(mode)=hmode then space_factor:=1000
//!   else  begin q:=new_noad; math_type(nucleus(q)):=sub_box;
//!     info(nucleus(q)):=p; p:=q;
//!     end;
//!   tail_append(p);
//!   end;
//! end;
//!
//! @ A paragraph ends when a |par_end| command is sensed, or when we are in
//! horizontal mode when reaching the right brace of vertical-mode routines
//! like \.{\\vbox}, \.{\\insert}, or \.{\\output}.
//!
//! @<Cases of |main_control| that build...@>=
//! vmode+par_end: begin normal_paragraph;
//!   if mode>0 then build_page;
//!   end;
//! hmode+par_end: begin if align_state<0 then off_save; {this tries to
//!     recover from an alignment that didn't end properly}
//!   end_graf; {this takes us to the enclosing mode, if |mode>0|}
//!   if mode=vmode then build_page;
//!   end;
//! hmode+stop,hmode+vskip,hmode+hrule,hmode+un_vbox,hmode+halign: head_for_vmode;
//!
//! @ @<Declare act...@>=
//! procedure head_for_vmode;
//! begin if mode<0 then
//!   if cur_cmd<>hrule then off_save
//!   else  begin print_err("You can't use `");
//!     print_esc("hrule"); print("' here except with leaders");
//! @.You can't use \\hrule...@>
//!     help2("To put a horizontal rule in an hbox or an alignment,")@/
//!       ("you should use \leaders or \hrulefill (see The TeXbook).");
//!     error;
//!     end
//! else  begin back_input; cur_tok:=par_token; back_input; token_type:=inserted;
//!   end;
//! end;
//!
//! @ @<Declare act...@>=
//! procedure end_graf;
//! begin if mode=hmode then
//!   begin if head=tail then pop_nest {null paragraphs are ignored}
//!   else line_break(widow_penalty);
//!   normal_paragraph;
//!   error_count:=0;
//!   end;
//! end;
//!
//! @ Insertion and adjustment and mark nodes are constructed by the following
//! pieces of the program.
//!
//! @<Cases of |main_control| that build...@>=
//! any_mode(insert),hmode+vadjust,mmode+vadjust: begin_insert_or_adjust;
//! any_mode(mark): make_mark;
//!
//! @ @<Forbidden...@>=
//! vmode+vadjust,
//!
//! @ @<Declare act...@>=
//! procedure begin_insert_or_adjust;
//! begin if cur_cmd=vadjust then cur_val:=255
//! else  begin scan_eight_bit_int;
//!   if cur_val=255 then
//!     begin print_err("You can't "); print_esc("insert"); print_int(255);
//! @.You can't \\insert255@>
//!     help1("I'm changing to \insert0; box 255 is special.");
//!     error; cur_val:=0;
//!     end;
//!   end;
//! saved(0):=cur_val; incr(save_ptr);
//! new_save_level(insert_group); scan_left_brace; normal_paragraph;
//! push_nest; mode:=-vmode; prev_depth:=ignore_depth;
//! end;
//!
//! @ @<Cases of |handle...@>=
//! insert_group: begin end_graf; q:=split_top_skip; add_glue_ref(q);
//!   d:=split_max_depth; f:=floating_penalty; unsave; decr(save_ptr);
//!   {now |saved(0)| is the insertion number, or 255 for |vadjust|}
//!   p:=vpack(link(head),natural); pop_nest;
//!   if saved(0)<255 then
//!     begin tail_append(get_node(ins_node_size));
//!     type(tail):=ins_node; subtype(tail):=qi(saved(0));
//!     height(tail):=height(p)+depth(p); ins_ptr(tail):=list_ptr(p);
//!     split_top_ptr(tail):=q; depth(tail):=d; float_cost(tail):=f;
//!     end
//!   else  begin tail_append(get_node(small_node_size));
//!     type(tail):=adjust_node;@/
//!     subtype(tail):=0; {the |subtype| is not used}
//!     adjust_ptr(tail):=list_ptr(p); delete_glue_ref(q);
//!     end;
//!   free_node(p,box_node_size);
//!   if nest_ptr=0 then build_page;
//!   end;
//! output_group: @<Resume the page builder...@>;
//!
//! @ @<Declare act...@>=
//! procedure make_mark;
//! var p:pointer; {new node}
//! begin p:=scan_toks(false,true); p:=get_node(small_node_size);
//! type(p):=mark_node; subtype(p):=0; {the |subtype| is not used}
//! mark_ptr(p):=def_ref; link(tail):=p; tail:=p;
//! end;
//!
//! @ Penalty nodes get into a list via the |break_penalty| command.
//! @^penalties@>
//!
//! @<Cases of |main_control| that build...@>=
//! any_mode(break_penalty): append_penalty;
//!
//! @ @<Declare action...@>=
//! procedure append_penalty;
//! begin scan_int; tail_append(new_penalty(cur_val));
//! if mode=vmode then build_page;
//! end;
//!
//! @ The |remove_item| command removes a penalty, kern, or glue node if it
//! appears at the tail of the current list, using a brute-force linear scan.
//! Like \.{\\lastbox}, this command is not allowed in vertical mode (except
//! internal vertical mode), since the current list in vertical mode is sent
//! to the page builder.  But if we happen to be able to implement it in
//! vertical mode, we do.
//!
//! @<Cases of |main_control| that build...@>=
//! any_mode(remove_item): delete_last;
//!
//! @ When |delete_last| is called, |cur_chr| is the |type| of node that
//! will be deleted, if present.
//!
//! @<Declare action...@>=
//! procedure delete_last;
//! label exit;
//! var @!p,@!q:pointer; {run through the current list}
//! @!m:quarterword; {the length of a replacement list}
//! begin if (mode=vmode)and(tail=head) then
//!   @<Apologize for inability to do the operation now,
//!     unless \.{\\unskip} follows non-glue@>
//! else  begin if not is_char_node(tail) then if type(tail)=cur_chr then
//!     begin q:=head;
//!     repeat p:=q;
//!     if not is_char_node(q) then if type(q)=disc_node then
//!       begin for m:=1 to replace_count(q) do p:=link(p);
//!       if p=tail then return;
//!       end;
//!     q:=link(p);
//!     until q=tail;
//!     link(p):=null; flush_node_list(tail); tail:=p;
//!     end;
//!   end;
//! exit:end;
//!
//! @ @<Apologize for inability to do the operation...@>=
//! begin if (cur_chr<>glue_node)or(last_glue<>max_halfword) then
//!   begin you_cant;
//!   help2("Sorry...I usually can't take things from the current page.")@/
//!     ("Try `I\vskip-\lastskip' instead.");
//!   if cur_chr=kern_node then help_line[0]:=
//!     ("Try `I\kern-\lastkern' instead.")
//!   else if cur_chr<>glue_node then help_line[0]:=@|
//!     ("Perhaps you can make the output routine do it.");
//!   error;
//!   end;
//! end
//!
//! @ @<Put each...@>=
//! primitive("unpenalty",remove_item,penalty_node);@/
//! @!@:un_penalty_}{\.{\\unpenalty} primitive@>
//! primitive("unkern",remove_item,kern_node);@/
//! @!@:un_kern_}{\.{\\unkern} primitive@>
//! primitive("unskip",remove_item,glue_node);@/
//! @!@:un_skip_}{\.{\\unskip} primitive@>
//! primitive("unhbox",un_hbox,box_code);@/
//! @!@:un_hbox_}{\.{\\unhbox} primitive@>
//! primitive("unhcopy",un_hbox,copy_code);@/
//! @!@:un_hcopy_}{\.{\\unhcopy} primitive@>
//! primitive("unvbox",un_vbox,box_code);@/
//! @!@:un_vbox_}{\.{\\unvbox} primitive@>
//! primitive("unvcopy",un_vbox,copy_code);@/
//! @!@:un_vcopy_}{\.{\\unvcopy} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! remove_item: if chr_code=glue_node then print_esc("unskip")
//!   else if chr_code=kern_node then print_esc("unkern")
//!   else print_esc("unpenalty");
//! un_hbox: if chr_code=copy_code then print_esc("unhcopy")
//!   else print_esc("unhbox");
//! un_vbox: if chr_code=copy_code then print_esc("unvcopy")
//!   else print_esc("unvbox");
//!
//! @ The |un_hbox| and |un_vbox| commands unwrap one of the 256 current boxes.
//!
//! @<Cases of |main_control| that build...@>=
//! vmode+un_vbox,hmode+un_hbox,mmode+un_hbox: unpackage;
//!
//! @ @<Declare act...@>=
//! procedure unpackage;
//! label exit;
//! var p:pointer; {the box}
//! @!c:box_code..copy_code; {should we copy?}
//! begin c:=cur_chr; scan_eight_bit_int; p:=box(cur_val);
//! if p=null then return;
//! if (abs(mode)=mmode)or((abs(mode)=vmode)and(type(p)<>vlist_node))or@|
//!    ((abs(mode)=hmode)and(type(p)<>hlist_node)) then
//!   begin print_err("Incompatible list can't be unboxed");
//! @.Incompatible list...@>
//!   help3("Sorry, Pandora. (You sneaky devil.)")@/
//!   ("I refuse to unbox an \hbox in vertical mode or vice versa.")@/
//!   ("And I can't open any boxes in math mode.");@/
//!   error; return;
//!   end;
//! if c=copy_code then link(tail):=copy_node_list(list_ptr(p))
//! else  begin link(tail):=list_ptr(p); box(cur_val):=null;
//!   free_node(p,box_node_size);
//!   end;
//! while link(tail)<>null do tail:=link(tail);
//! exit:end;
//!
//! @ @<Forbidden...@>=vmode+ital_corr,
//!
//! @ Italic corrections are converted to kern nodes when the |ital_corr| command
//! follows a character. In math mode the same effect is achieved by appending
//! a kern of zero here, since italic corrections are supplied later.
//!
//! @<Cases of |main_control| that build...@>=
//! hmode+ital_corr: append_italic_correction;
//! mmode+ital_corr: tail_append(new_kern(0));
//!
//! @ @<Declare act...@>=
//! procedure append_italic_correction;
//! label exit;
//! var p:pointer; {|char_node| at the tail of the current list}
//! @!f:internal_font_number; {the font in the |char_node|}
//! begin if tail<>head then
//!   begin if is_char_node(tail) then p:=tail
//!   else if type(tail)=ligature_node then p:=lig_char(tail)
//!   else return;
//!   f:=font(p);
//!   tail_append(new_kern(char_italic(f)(char_info(f)(character(p)))));
//!   subtype(tail):=explicit;
//!   end;
//! exit:end;
//!
//! @ Discretionary nodes are easy in the common case `\.{\\-}', but in the
//! general case we must process three braces full of items.
//!
//! @<Put each...@>=
//! primitive("-",discretionary,1);
//! @!@:Single-character primitives -}{\quad\.{\\-}@>
//! primitive("discretionary",discretionary,0);
//! @!@:discretionary_}{\.{\\discretionary} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! discretionary: if chr_code=1 then
//!   print_esc("-")@+else print_esc("discretionary");
//!
//! @ @<Cases of |main_control| that build...@>=
//! hmode+discretionary,mmode+discretionary: append_discretionary;
//!
//! @ The space factor does not change when we append a discretionary node,
//! but it starts out as 1000 in the subsidiary lists.
//!
//! @<Declare act...@>=
//! procedure append_discretionary;
//! var c:integer; {hyphen character}
//! begin tail_append(new_disc);
//! if cur_chr=1 then
//!   begin c:=hyphen_char[cur_font];
//!   if c>=0 then if c<256 then pre_break(tail):=new_character(cur_font,c);
//!   end
//! else  begin incr(save_ptr); saved(-1):=0; new_save_level(disc_group);
//!   scan_left_brace; push_nest; mode:=-hmode; space_factor:=1000;
//!   end;
//! end;
//!
//! @ The three discretionary lists are constructed somewhat as if they were
//! hboxes. A~subroutine called |build_discretionary| handles the transitions.
//! (This is sort of fun.)
//!
//! @<Cases of |handle...@>=
//! disc_group: build_discretionary;
//!
//! @ @<Declare act...@>=
//! procedure build_discretionary;
//! label done,exit;
//! var p,@!q:pointer; {for link manipulation}
//! @!n:integer; {length of discretionary list}
//! begin unsave;
//! @<Prune the current list, if necessary, until it contains only
//!   |char_node|, |kern_node|, |hlist_node|, |vlist_node|, |rule_node|,
//!   and |ligature_node| items; set |n| to the length of the list,
//!   and set |q| to the list's tail@>;
//! p:=link(head); pop_nest;
//! case saved(-1) of
//! 0:pre_break(tail):=p;
//! 1:post_break(tail):=p;
//! 2:@<Attach list |p| to the current list, and record its length;
//!   then finish up and |return|@>;
//! end; {there are no other cases}
//! incr(saved(-1)); new_save_level(disc_group); scan_left_brace;
//! push_nest; mode:=-hmode; space_factor:=1000;
//! exit:end;
//!
//! @ @<Attach list |p| to the current...@>=
//! begin if (n>0)and(abs(mode)=mmode) then
//!   begin print_err("Illegal math "); print_esc("discretionary");
//! @.Illegal math \\disc...@>
//!   help2("Sorry: The third part of a discretionary break must be")@/
//!   ("empty, in math formulas. I had to delete your third part.");
//!   flush_node_list(p); n:=0; error;
//!   end
//! else link(tail):=p;
//! if n<=max_quarterword then replace_count(tail):=n
//! else  begin print_err("Discretionary list is too long");
//! @.Discretionary list is too long@>
//!   help2("Wow---I never thought anybody would tweak me here.")@/
//!   ("You can't seriously need such a huge discretionary list?");
//!   error;
//!   end;
//! if n>0 then tail:=q;
//! decr(save_ptr); return;
//! end
//!
//! @ During this loop, |p=link(q)| and there are |n| items preceding |p|.
//!
//! @<Prune the current list, if necessary...@>=
//! q:=head; p:=link(q); n:=0;
//! while p<>null do
//!   begin if not is_char_node(p) then if type(p)>rule_node then
//!     if type(p)<>kern_node then if type(p)<>ligature_node then
//!       begin print_err("Improper discretionary list");
//! @.Improper discretionary list@>
//!       help1("Discretionary lists must contain only boxes and kerns.");@/
//!       error;
//!       begin_diagnostic;
//!       print_nl("The following discretionary sublist has been deleted:");
//! @.The following...deleted@>
//!       show_box(p);
//!       end_diagnostic(true);
//!       flush_node_list(p); link(q):=null; goto done;
//!       end;
//!   q:=p; p:=link(q); incr(n);
//!   end;
//! done:
//!
//! @ We need only one more thing to complete the horizontal mode routines, namely
//! the \.{\\accent} primitive.
//!
//! @<Cases of |main_control| that build...@>=
//! hmode+accent: make_accent;
//!
//! @ The positioning of accents is straightforward but tedious. Given an accent
//! of width |a|, designed for characters of height |x| and slant |s|;
//! and given a character of width |w|, height |h|, and slant |t|: We will shift
//! the accent down by |x-h|, and we will insert kern nodes that have the effect of
//! centering the accent over the character and shifting the accent to the
//! right by $\delta={1\over2}(w-a)+h\cdot t-x\cdot s$.  If either character is
//! absent from the font, we will simply use the other, without shifting.
//!
//! @<Declare act...@>=
//! procedure make_accent;
//! var s,@!t: real; {amount of slant}
//! @!p,@!q,@!r:pointer; {character, box, and kern nodes}
//! @!f:internal_font_number; {relevant font}
//! @!a,@!h,@!x,@!w,@!delta:scaled; {heights and widths, as explained above}
//! @!i:four_quarters; {character information}
//! begin scan_char_num; f:=cur_font; p:=new_character(f,cur_val);
//! if p<>null then
//!   begin x:=x_height(f); s:=slant(f)/float_constant(65536);
//! @^real division@>
//!   a:=char_width(f)(char_info(f)(character(p)));@/
//!   do_assignments;@/
//!   @<Create a character node |q| for the next character,
//!     but set |q:=null| if problems arise@>;
//!   if q<>null then @<Append the accent with appropriate kerns,
//!       then set |p:=q|@>;
//!   link(tail):=p; tail:=p; space_factor:=1000;
//!   end;
//! end;
//!
//! @ @<Create a character node |q| for the next...@>=
//! q:=null; f:=cur_font;
//! if (cur_cmd=letter)or(cur_cmd=other_char)or(cur_cmd=char_given) then
//!   q:=new_character(f,cur_chr)
//! else if cur_cmd=char_num then
//!   begin scan_char_num; q:=new_character(f,cur_val);
//!   end
//! else back_input
//!
//! @ The kern nodes appended here must be distinguished from other kerns, lest
//! they be wiped away by the hyphenation algorithm or by a previous line break.
//!
//! The two kerns are computed with (machine-dependent) |real| arithmetic, but
//! their sum is machine-independent; the net effect is machine-independent,
//! because the user cannot remove these nodes nor access them via \.{\\lastkern}.
//!
//! @<Append the accent with appropriate kerns...@>=
//! begin t:=slant(f)/float_constant(65536);
//! @^real division@>
//! i:=char_info(f)(character(q));
//! w:=char_width(f)(i); h:=char_height(f)(height_depth(i));
//! if h<>x then {the accent must be shifted up or down}
//!   begin p:=hpack(p,natural); shift_amount(p):=x-h;
//!   end;
//! delta:=round((w-a)/float_constant(2)+h*t-x*s);
//! @^real multiplication@>
//! @^real addition@>
//! r:=new_kern(delta); subtype(r):=acc_kern; link(tail):=r; link(r):=p;
//! tail:=new_kern(-a-delta); subtype(tail):=acc_kern; link(p):=tail; p:=q;
//! end
//!
//! @ When `\.{\\cr}' or `\.{\\span}' or a tab mark comes through the scanner
//! into |main_control|, it might be that the user has foolishly inserted
//! one of them into something that has nothing to do with alignment. But it is
//! far more likely that a left brace or right brace has been omitted, since
//! |get_next| takes actions appropriate to alignment only when `\.{\\cr}'
//! or `\.{\\span}' or tab marks occur with |align_state=0|. The following
//! program attempts to make an appropriate recovery.
//!
//! @<Cases of |main_control| that build...@>=
//! any_mode(car_ret), any_mode(tab_mark): align_error;
//! any_mode(no_align): no_align_error;
//! any_mode(omit): omit_error;
//!
//! @ @<Declare act...@>=
//! procedure align_error;
//! begin if abs(align_state)>2 then
//!   @<Express consternation over the fact that no alignment is in progress@>
//! else  begin back_input;
//!   if align_state<0 then
//!     begin print_err("Missing { inserted");
//! @.Missing \{ inserted@>
//!     incr(align_state); cur_tok:=left_brace_token+"{";
//!     end
//!   else  begin print_err("Missing } inserted");
//! @.Missing \} inserted@>
//!     decr(align_state); cur_tok:=right_brace_token+"}";
//!     end;
//!   help3("I've put in what seems to be necessary to fix")@/
//!     ("the current column of the current alignment.")@/
//!     ("Try to go on, since this might almost work."); ins_error;
//!   end;
//! end;
//!
//! @ @<Express consternation...@>=
//! begin print_err("Misplaced "); print_cmd_chr(cur_cmd,cur_chr);
//! @.Misplaced \&@>
//! @.Misplaced \\span@>
//! @.Misplaced \\cr@>
//! if cur_tok=tab_token+"&" then
//!   begin help6("I can't figure out why you would want to use a tab mark")@/
//!   ("here. If you just want an ampersand, the remedy is")@/
//!   ("simple: Just type `I\&' now. But if some right brace")@/
//!   ("up above has ended a previous alignment prematurely,")@/
//!   ("you're probably due for more error messages, and you")@/
//!   ("might try typing `S' now just to see what is salvageable.");
//!   end
//! else  begin help5("I can't figure out why you would want to use a tab mark")@/
//!   ("or \cr or \span just now. If something like a right brace")@/
//!   ("up above has ended a previous alignment prematurely,")@/
//!   ("you're probably due for more error messages, and you")@/
//!   ("might try typing `S' now just to see what is salvageable.");
//!   end;
//! error;
//! end
//!
//! @ The help messages here contain a little white lie, since \.{\\noalign}
//! and \.{\\omit} are allowed also after `\.{\\noalign\{...\}}'.
//!
//! @<Declare act...@>=
//! procedure no_align_error;
//! begin print_err("Misplaced "); print_esc("noalign");
//! @.Misplaced \\noalign@>
//! help2("I expect to see \noalign only after the \cr of")@/
//!   ("an alignment. Proceed, and I'll ignore this case."); error;
//! end;
//! procedure omit_error;
//! begin print_err("Misplaced "); print_esc("omit");
//! @.Misplaced \\omit@>
//! help2("I expect to see \omit only after tab marks or the \cr of")@/
//!   ("an alignment. Proceed, and I'll ignore this case."); error;
//! end;
//!
//! @ We've now covered most of the abuses of \.{\\halign} and \.{\\valign}.
//! Let's take a look at what happens when they are used correctly.
//!
//! @<Cases of |main_control| that build...@>=
//! vmode+halign,hmode+valign:init_align;
//! mmode+halign: if privileged then
//!   if cur_group=math_shift_group then init_align
//!   else off_save;
//! vmode+endv,hmode+endv: do_endv;
//!
//! @ An |align_group| code is supposed to remain on the |save_stack|
//! during an entire alignment, until |fin_align| removes it.
//!
//! A devious user might force an |endv| command to occur just about anywhere;
//! we must defeat such hacks.
//!
//! @<Declare act...@>=
//! procedure do_endv;
//! begin base_ptr:=input_ptr; input_stack[base_ptr]:=cur_input;
//! while (input_stack[base_ptr].index_field<>v_template) and
//!       (input_stack[base_ptr].loc_field=null) and
//!       (input_stack[base_ptr].state_field=token_list) do decr(base_ptr);
//! if (input_stack[base_ptr].index_field<>v_template) or
//!       (input_stack[base_ptr].loc_field<>null) or
//!       (input_stack[base_ptr].state_field<>token_list) then
//!   fatal_error("(interwoven alignment preambles are not allowed)");
//! @.interwoven alignment preambles...@>
//!  if cur_group=align_group then
//!   begin end_graf;
//!   if fin_col then fin_row;
//!   end
//! else off_save;
//! end;
//!
//! @ @<Cases of |handle_right_brace|...@>=
//! align_group: begin back_input; cur_tok:=cs_token_flag+frozen_cr;
//!   print_err("Missing "); print_esc("cr"); print(" inserted");
//! @.Missing \\cr inserted@>
//!   help1("I'm guessing that you meant to end an alignment here.");
//!   ins_error;
//!   end;
//!
//! @ @<Cases of |handle_right_brace|...@>=
//! no_align_group: begin end_graf; unsave; align_peek;
//!   end;
//!
//! @ Finally, \.{\\endcsname} is not supposed to get through to |main_control|.
//!
//! @<Cases of |main_control| that build...@>=
//! any_mode(end_cs_name): cs_error;
//!
//! @ @<Declare act...@>=
//! procedure cs_error;
//! begin print_err("Extra "); print_esc("endcsname");
//! @.Extra \\endcsname@>
//! help1("I'm ignoring this, since I wasn't doing a \csname.");
//! error;
//! end;
//!
