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
