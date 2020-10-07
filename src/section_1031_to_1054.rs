//! @ When a new token has just been fetched at |big_switch|, we have an
//! ideal place to monitor \TeX's activity.
//! @^debugging@>
//!
//! @<Give diagnostic information, if requested@>=
//! if interrupt<>0 then if OK_to_interrupt then
//!   begin back_input; check_interrupt; goto big_switch;
//!   end;
//! @!debug if panicking then check_mem(false);@+@;@+gubed
//! if tracing_commands>0 then show_cur_cmd_chr
//!
//! @ The following part of the program was first written in a structured
//! manner, according to the philosophy that ``premature optimization is
//! the root of all evil.'' Then it was rearranged into pieces of
//! spaghetti so that the most common actions could proceed with little or
//! no redundancy.
//!
//! The original unoptimized form of this algorithm resembles the
//! |reconstitute| procedure, which was described earlier in connection with
//! hyphenation. Again we have an implied ``cursor'' between characters
//! |cur_l| and |cur_r|. The main difference is that the |lig_stack| can now
//! contain a charnode as well as pseudo-ligatures; that stack is now
//! usually nonempty, because the next character of input (if any) has been
//! appended to it. In |main_control| we have
//! $$|cur_r|=\cases{|character(lig_stack)|,&if |lig_stack>null|;\cr
//!   |font_bchar[cur_font]|,&otherwise;\cr}$$
//! except when |character(lig_stack)=font_false_bchar[cur_font]|.
//! Several additional global variables are needed.
//!
//! @<Glob...@>=
//! @!main_f:internal_font_number; {the current font}
//! @!main_i:four_quarters; {character information bytes for |cur_l|}
//! @!main_j:four_quarters; {ligature/kern command}
//! @!main_k:font_index; {index into |font_info|}
//! @!main_p:pointer; {temporary register for list manipulation}
//! @!main_s:integer; {space factor value}
//! @!bchar:halfword; {right boundary character of current font, or |non_char|}
//! @!false_bchar:halfword; {nonexistent character matching |bchar|, or |non_char|}
//! @!cancel_boundary:boolean; {should the left boundary be ignored?}
//! @!ins_disc:boolean; {should we insert a discretionary node?}
//!
//! @ The boolean variables of the main loop are normally false, and always reset
//! to false before the loop is left. That saves us the extra work of initializing
//! each time.
//!
//! @<Set init...@>=
//! ligature_present:=false; cancel_boundary:=false; lft_hit:=false; rt_hit:=false;
//! ins_disc:=false;
//!
//! @ We leave the |space_factor| unchanged if |sf_code(cur_chr)=0|; otherwise we
//! set it equal to |sf_code(cur_chr)|, except that it should never change
//! from a value less than 1000 to a value exceeding 1000. The most common
//! case is |sf_code(cur_chr)=1000|, so we want that case to be fast.
//!
//! The overall structure of the main loop is presented here. Some program labels
//! are inside the individual sections.
//! @^inner loop@>
//!
//! @d adjust_space_factor==@t@>@;@/
//!   main_s:=sf_code(cur_chr);
//!   if main_s=1000 then space_factor:=1000
//!   else if main_s<1000 then
//!     begin if main_s>0 then space_factor:=main_s;
//!     end
//!   else if space_factor<1000 then space_factor:=1000
//!   else space_factor:=main_s
//!
//! @<Append character |cur_chr|...@>=
//! adjust_space_factor;@/
//! main_f:=cur_font;
//! bchar:=font_bchar[main_f]; false_bchar:=font_false_bchar[main_f];
//! if mode>0 then if language<>clang then fix_language;
//! fast_get_avail(lig_stack); font(lig_stack):=main_f; cur_l:=qi(cur_chr);
//! character(lig_stack):=cur_l;@/
//! cur_q:=tail;
//! if cancel_boundary then
//!   begin cancel_boundary:=false; main_k:=non_address;
//!   end
//! else main_k:=bchar_label[main_f];
//! if main_k=non_address then goto main_loop_move+2; {no left boundary processing}
//! cur_r:=cur_l; cur_l:=non_char;
//! goto main_lig_loop+1; {begin with cursor after left boundary}
//! @#
//! main_loop_wrapup:@<Make a ligature node, if |ligature_present|;
//!   insert a null discretionary, if appropriate@>;
//! main_loop_move:@<If the cursor is immediately followed by the right boundary,
//!   |goto reswitch|; if it's followed by an invalid character, |goto big_switch|;
//!   otherwise move the cursor one step to the right and |goto main_lig_loop|@>;
//! main_loop_lookahead:@<Look ahead for another character, or leave |lig_stack|
//!   empty if there's none there@>;
//! main_lig_loop:@<If there's a ligature/kern command relevant to |cur_l| and
//!   |cur_r|, adjust the text appropriately; exit to |main_loop_wrapup|@>;
//! main_loop_move_lig:@<Move the cursor past a pseudo-ligature, then
//!   |goto main_loop_lookahead| or |main_lig_loop|@>
//!
//! @ If |link(cur_q)| is nonnull when |wrapup| is invoked, |cur_q| points to
//! the list of characters that were consumed while building the ligature
//! character~|cur_l|.
//!
//! A discretionary break is not inserted for an explicit hyphen when we are in
//! restricted horizontal mode. In particular, this avoids putting discretionary
//! nodes inside of other discretionaries.
//! @^inner loop@>
//!
//! @d pack_lig(#)== {the parameter is either |rt_hit| or |false|}
//!   begin main_p:=new_ligature(main_f,cur_l,link(cur_q));
//!   if lft_hit then
//!     begin subtype(main_p):=2; lft_hit:=false;
//!     end;
//!   if # then if lig_stack=null then
//!     begin incr(subtype(main_p)); rt_hit:=false;
//!     end;
//!   link(cur_q):=main_p; tail:=main_p; ligature_present:=false;
//!   end
//!
//! @d wrapup(#)==if cur_l<non_char then
//!   begin if link(cur_q)>null then
//!     if character(tail)=qi(hyphen_char[main_f]) then ins_disc:=true;
//!   if ligature_present then pack_lig(#);
//!   if ins_disc then
//!     begin ins_disc:=false;
//!     if mode>0 then tail_append(new_disc);
//!     end;
//!   end
//!
//! @<Make a ligature node, if |ligature_present|;...@>=
//! wrapup(rt_hit)
//!
//! @ @<If the cursor is immediately followed by the right boundary...@>=
//! @^inner loop@>
//! if lig_stack=null then goto reswitch;
//! cur_q:=tail; cur_l:=character(lig_stack);
//! main_loop_move+1:if not is_char_node(lig_stack) then goto main_loop_move_lig;
//! main_loop_move+2:if(cur_chr<font_bc[main_f])or(cur_chr>font_ec[main_f]) then
//!   begin char_warning(main_f,cur_chr); free_avail(lig_stack); goto big_switch;
//!   end;
//! main_i:=char_info(main_f)(cur_l);
//! if not char_exists(main_i) then
//!   begin char_warning(main_f,cur_chr); free_avail(lig_stack); goto big_switch;
//!   end;
//! link(tail):=lig_stack; tail:=lig_stack {|main_loop_lookahead| is next}
//!
//! @ Here we are at |main_loop_move_lig|.
//! When we begin this code we have |cur_q=tail| and |cur_l=character(lig_stack)|.
//!
//! @<Move the cursor past a pseudo-ligature...@>=
//! main_p:=lig_ptr(lig_stack);
//! if main_p>null then tail_append(main_p); {append a single character}
//! temp_ptr:=lig_stack; lig_stack:=link(temp_ptr);
//! free_node(temp_ptr,small_node_size);
//! main_i:=char_info(main_f)(cur_l); ligature_present:=true;
//! if lig_stack=null then
//!   if main_p>null then goto main_loop_lookahead
//!   else cur_r:=bchar
//! else cur_r:=character(lig_stack);
//! goto main_lig_loop
//!
//! @ The result of \.{\\char} can participate in a ligature or kern, so we must
//! look ahead for it.
//!
//! @<Look ahead for another character...@>=
//! get_next; {set only |cur_cmd| and |cur_chr|, for speed}
//! if cur_cmd=letter then goto main_loop_lookahead+1;
//! if cur_cmd=other_char then goto main_loop_lookahead+1;
//! if cur_cmd=char_given then goto main_loop_lookahead+1;
//! x_token; {now expand and set |cur_cmd|, |cur_chr|, |cur_tok|}
//! if cur_cmd=letter then goto main_loop_lookahead+1;
//! if cur_cmd=other_char then goto main_loop_lookahead+1;
//! if cur_cmd=char_given then goto main_loop_lookahead+1;
//! if cur_cmd=char_num then
//!   begin scan_char_num; cur_chr:=cur_val; goto main_loop_lookahead+1;
//!   end;
//! if cur_cmd=no_boundary then bchar:=non_char;
//! cur_r:=bchar; lig_stack:=null; goto main_lig_loop;
//! main_loop_lookahead+1: adjust_space_factor;
//! fast_get_avail(lig_stack); font(lig_stack):=main_f;
//! cur_r:=qi(cur_chr); character(lig_stack):=cur_r;
//! if cur_r=false_bchar then cur_r:=non_char {this prevents spurious ligatures}
//!
//! @ Even though comparatively few characters have a lig/kern program, several
//! of the instructions here count as part of \TeX's inner loop, since a
//! @^inner loop@>
//! potentially long sequential search must be performed. For example, tests with
//! Computer Modern Roman showed that about 40 per cent of all characters
//! actually encountered in practice had a lig/kern program, and that about four
//! lig/kern commands were investigated for every such character.
//!
//! At the beginning of this code we have |main_i=char_info(main_f)(cur_l)|.
//!
//! @<If there's a ligature/kern command...@>=
//! if char_tag(main_i)<>lig_tag then goto main_loop_wrapup;
//! if cur_r=non_char then goto main_loop_wrapup;
//! main_k:=lig_kern_start(main_f)(main_i); main_j:=font_info[main_k].qqqq;
//! if skip_byte(main_j)<=stop_flag then goto main_lig_loop+2;
//! main_k:=lig_kern_restart(main_f)(main_j);
//! main_lig_loop+1:main_j:=font_info[main_k].qqqq;
//! main_lig_loop+2:if next_char(main_j)=cur_r then
//!  if skip_byte(main_j)<=stop_flag then
//!   @<Do ligature or kern command, returning to |main_lig_loop|
//!   or |main_loop_wrapup| or |main_loop_move|@>;
//! if skip_byte(main_j)=qi(0) then incr(main_k)
//! else begin if skip_byte(main_j)>=stop_flag then goto main_loop_wrapup;
//!   main_k:=main_k+qo(skip_byte(main_j))+1;
//!   end;
//! goto main_lig_loop+1
//!
//! @ When a ligature or kern instruction matches a character, we know from
//! |read_font_info| that the character exists in the font, even though we
//! haven't verified its existence in the normal way.
//!
//! This section could be made into a subroutine, if the code inside
//! |main_control| needs to be shortened.
//!
//! \chardef\?='174 % vertical line to indicate character retention
//!
//! @<Do ligature or kern command...@>=
//! begin if op_byte(main_j)>=kern_flag then
//!   begin wrapup(rt_hit);
//!   tail_append(new_kern(char_kern(main_f)(main_j))); goto main_loop_move;
//!   end;
//! if cur_l=non_char then lft_hit:=true
//! else if lig_stack=null then rt_hit:=true;
//! check_interrupt; {allow a way out in case there's an infinite ligature loop}
//! case op_byte(main_j) of
//! qi(1),qi(5):begin cur_l:=rem_byte(main_j); {\.{=:\?}, \.{=:\?>}}
//!   main_i:=char_info(main_f)(cur_l); ligature_present:=true;
//!   end;
//! qi(2),qi(6):begin cur_r:=rem_byte(main_j); {\.{\?=:}, \.{\?=:>}}
//!   if lig_stack=null then {right boundary character is being consumed}
//!     begin lig_stack:=new_lig_item(cur_r); bchar:=non_char;
//!     end
//!   else if is_char_node(lig_stack) then {|link(lig_stack)=null|}
//!     begin main_p:=lig_stack; lig_stack:=new_lig_item(cur_r);
//!     lig_ptr(lig_stack):=main_p;
//!     end
//!   else character(lig_stack):=cur_r;
//!   end;
//! qi(3):begin cur_r:=rem_byte(main_j); {\.{\?=:\?}}
//!   main_p:=lig_stack; lig_stack:=new_lig_item(cur_r);
//!   link(lig_stack):=main_p;
//!   end;
//! qi(7),qi(11):begin wrapup(false); {\.{\?=:\?>}, \.{\?=:\?>>}}
//!   cur_q:=tail; cur_l:=rem_byte(main_j);
//!   main_i:=char_info(main_f)(cur_l); ligature_present:=true;
//!   end;
//! othercases begin cur_l:=rem_byte(main_j); ligature_present:=true; {\.{=:}}
//!   if lig_stack=null then goto main_loop_wrapup
//!   else goto main_loop_move+1;
//!   end
//! endcases;
//! if op_byte(main_j)>qi(4) then
//!   if op_byte(main_j)<>qi(7) then goto main_loop_wrapup;
//! if cur_l<non_char then goto main_lig_loop;
//! main_k:=bchar_label[main_f]; goto main_lig_loop+1;
//! end
//!
//! @ The occurrence of blank spaces is almost part of \TeX's inner loop,
//! @^inner loop@>
//! since we usually encounter about one space for every five non-blank characters.
//! Therefore |main_control| gives second-highest priority to ordinary spaces.
//!
//! When a glue parameter like \.{\\spaceskip} is set to `\.{0pt}', we will
//! see to it later that the corresponding glue specification is precisely
//! |zero_glue|, not merely a pointer to some specification that happens
//! to be full of zeroes. Therefore it is simple to test whether a glue parameter
//! is zero or~not.
//!
//! @<Append a normal inter-word space...@>=
//! if space_skip=zero_glue then
//!   begin @<Find the glue specification, |main_p|, for
//!     text spaces in the current font@>;
//!   temp_ptr:=new_glue(main_p);
//!   end
//! else temp_ptr:=new_param_glue(space_skip_code);
//! link(tail):=temp_ptr; tail:=temp_ptr;
//! goto big_switch
//!
//! @ Having |font_glue| allocated for each text font saves both time and memory.
//! If any of the three spacing parameters are subsequently changed by the
//! use of \.{\\fontdimen}, the |find_font_dimen| procedure deallocates the
//! |font_glue| specification allocated here.
//!
//! @<Find the glue specification...@>=
//! begin main_p:=font_glue[cur_font];
//! if main_p=null then
//!   begin main_p:=new_spec(zero_glue); main_k:=param_base[cur_font]+space_code;
//!   width(main_p):=font_info[main_k].sc; {that's |space(cur_font)|}
//!   stretch(main_p):=font_info[main_k+1].sc; {and |space_stretch(cur_font)|}
//!   shrink(main_p):=font_info[main_k+2].sc; {and |space_shrink(cur_font)|}
//!   font_glue[cur_font]:=main_p;
//!   end;
//! end
//!
//! @ @<Declare act...@>=
//! procedure app_space; {handle spaces when |space_factor<>1000|}
//! var@!q:pointer; {glue node}
//! begin if (space_factor>=2000)and(xspace_skip<>zero_glue) then
//!   q:=new_param_glue(xspace_skip_code)
//! else  begin if space_skip<>zero_glue then main_p:=space_skip
//!   else @<Find the glue specification...@>;
//!   main_p:=new_spec(main_p);
//!   @<Modify the glue specification in |main_p| according to the space factor@>;
//!   q:=new_glue(main_p); glue_ref_count(main_p):=null;
//!   end;
//! link(tail):=q; tail:=q;
//! end;
//!
//! @ @<Modify the glue specification in |main_p| according to the space factor@>=
//! if space_factor>=2000 then width(main_p):=width(main_p)+extra_space(cur_font);
//! stretch(main_p):=xn_over_d(stretch(main_p),space_factor,1000);
//! shrink(main_p):=xn_over_d(shrink(main_p),1000,space_factor)
//!
//! @ Whew---that covers the main loop. We can now proceed at a leisurely
//! pace through the other combinations of possibilities.
//!
//! @d any_mode(#)==vmode+#,hmode+#,mmode+# {for mode-independent commands}
//!
//! @<Cases of |main_control| that are not part of the inner loop@>=
//! any_mode(relax),vmode+spacer,mmode+spacer,mmode+no_boundary:do_nothing;
//! any_mode(ignore_spaces): begin @<Get the next non-blank non-call...@>;
//!   goto reswitch;
//!   end;
//! vmode+stop: if its_all_over then return; {this is the only way out}
//! @t\4@>@<Forbidden cases detected in |main_control|@>@+@,any_mode(mac_param):
//!   report_illegal_case;
//! @<Math-only cases in non-math modes, or vice versa@>: insert_dollar_sign;
//! @t\4@>@<Cases of |main_control| that build boxes and lists@>@;
//! @t\4@>@<Cases of |main_control| that don't depend on |mode|@>@;
//! @t\4@>@<Cases of |main_control| that are for extensions to \TeX@>@;
//!
//! @ Here is a list of cases where the user has probably gotten into or out of math
//! mode by mistake. \TeX\ will insert a dollar sign and rescan the current token.
//!
//! @d non_math(#)==vmode+#,hmode+#
//!
//! @<Math-only cases in non-math modes...@>=
//! non_math(sup_mark), non_math(sub_mark), non_math(math_char_num),
//! non_math(math_given), non_math(math_comp), non_math(delim_num),
//! non_math(left_right), non_math(above), non_math(radical),
//! non_math(math_style), non_math(math_choice), non_math(vcenter),
//! non_math(non_script), non_math(mkern), non_math(limit_switch),
//! non_math(mskip), non_math(math_accent),
//! mmode+endv, mmode+par_end, mmode+stop, mmode+vskip, mmode+un_vbox,
//! mmode+valign, mmode+hrule
//!
//! @ @<Declare action...@>=
//! procedure insert_dollar_sign;
//! begin back_input; cur_tok:=math_shift_token+"$";
//! print_err("Missing $ inserted");
//! @.Missing \$ inserted@>
//! help2("I've inserted a begin-math/end-math symbol since I think")@/
//! ("you left one out. Proceed, with fingers crossed."); ins_error;
//! end;
//!
//! @ When erroneous situations arise, \TeX\ usually issues an error message
//! specific to the particular error. For example, `\.{\\noalign}' should
//! not appear in any mode, since it is recognized by the |align_peek| routine
//! in all of its legitimate appearances; a special error message is given
//! when `\.{\\noalign}' occurs elsewhere. But sometimes the most appropriate
//! error message is simply that the user is not allowed to do what he or she
//! has attempted. For example, `\.{\\moveleft}' is allowed only in vertical mode,
//! and `\.{\\lower}' only in non-vertical modes.  Such cases are enumerated
//! here and in the other sections referred to under `See also \dots.'
//!
//! @<Forbidden cases...@>=
//! vmode+vmove,hmode+hmove,mmode+hmove,any_mode(last_item),
//!
//! @ The `|you_cant|' procedure prints a line saying that the current command
//! is illegal in the current mode; it identifies these things symbolically.
//!
//! @<Declare action...@>=
//! procedure you_cant;
//! begin print_err("You can't use `");
//! @.You can't use x in y mode@>
//! print_cmd_chr(cur_cmd,cur_chr);
//! print("' in "); print_mode(mode);
//! end;
//!
//! @ @<Declare act...@>=
//! procedure report_illegal_case;
//! begin you_cant;
//! help4("Sorry, but I'm not programmed to handle this case;")@/
//! ("I'll just pretend that you didn't ask for it.")@/
//! ("If you're in the wrong mode, you might be able to")@/
//! ("return to the right one by typing `I}' or `I$' or `I\par'.");@/
//! error;
//! end;
//!
//! @ Some operations are allowed only in privileged modes, i.e., in cases
//! that |mode>0|. The |privileged| function is used to detect violations
//! of this rule; it issues an error message and returns |false| if the
//! current |mode| is negative.
//!
//! @<Declare act...@>=
//! function privileged:boolean;
//! begin if mode>0 then privileged:=true
//! else  begin report_illegal_case; privileged:=false;
//!   end;
//! end;
//!
//! @ Either \.{\\dump} or \.{\\end} will cause |main_control| to enter the
//! endgame, since both of them have `|stop|' as their command code.
//!
//! @<Put each...@>=
//! primitive("end",stop,0);@/
//! @!@:end_}{\.{\\end} primitive@>
//! primitive("dump",stop,1);@/
//! @!@:dump_}{\.{\\dump} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! stop:if chr_code=1 then print_esc("dump")@+else print_esc("end");
//!
//! @ We don't want to leave |main_control| immediately when a |stop| command
//! is sensed, because it may be necessary to invoke an \.{\\output} routine
//! several times before things really grind to a halt. (The output routine
//! might even say `\.{\\gdef\\end\{...\}}', to prolong the life of the job.)
//! Therefore |its_all_over| is |true| only when the current page
//! and contribution list are empty, and when the last output was not a
//! ``dead cycle.''
//!
//! @<Declare act...@>=
//! function its_all_over:boolean; {do this when \.{\\end} or \.{\\dump} occurs}
//! label exit;
//! begin if privileged then
//!   begin if (page_head=page_tail)and(head=tail)and(dead_cycles=0) then
//!     begin its_all_over:=true; return;
//!     end;
//!   back_input; {we will try to end again after ejecting residual material}
//!   tail_append(new_null_box);
//!   width(tail):=hsize;
//!   tail_append(new_glue(fill_glue));
//!   tail_append(new_penalty(-@'10000000000));@/
//!   build_page; {append \.{\\hbox to \\hsize\{\}\\vfill\\penalty-'10000000000}}
//!   end;
//! its_all_over:=false;
//! exit:end;
//!
