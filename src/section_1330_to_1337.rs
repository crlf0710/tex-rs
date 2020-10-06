//! @* \[51] The main program.
//! This is it: the part of \TeX\ that executes all those procedures we have
//! written.
//!
//! Well---almost. Let's leave space for a few more routines that we may
//! have forgotten.
//!
//! @p @<Last-minute procedures@>
//!
//! @ We have noted that there are two versions of \TeX82. One, called \.{INITEX},
//! @.INITEX@>
//! has to be run first; it initializes everything from scratch, without
//! reading a format file, and it has the capability of dumping a format file.
//! The other one is called `\.{VIRTEX}'; it is a ``virgin'' program that needs
//! @.VIRTEX@>
//! to input a format file in order to get started. \.{VIRTEX} typically has
//! more memory capacity than \.{INITEX}, because it does not need the space
//! consumed by the auxiliary hyphenation tables and the numerous calls on
//! |primitive|, etc.
//!
//! The \.{VIRTEX} program cannot read a format file instantaneously, of course;
//! the best implementations therefore allow for production versions of \TeX\ that
//! not only avoid the loading routine for \PASCAL\ object code, they also have
//! a format file pre-loaded. This is impossible to do if we stick to standard
//! \PASCAL; but there is a simple way to fool many systems into avoiding the
//! initialization, as follows:\quad(1)~We declare a global integer variable
//! called |ready_already|. The probability is negligible that this
//! variable holds any particular value like 314159 when \.{VIRTEX} is first
//! loaded.\quad(2)~After we have read in a format file and initialized
//! everything, we set |ready_already:=314159|.\quad(3)~Soon \.{VIRTEX}
//! will print `\.*', waiting for more input; and at this point we
//! interrupt the program and save its core image in some form that the
//! operating system can reload speedily.\quad(4)~When that core image is
//! activated, the program starts again at the beginning; but now
//! |ready_already=314159| and all the other global variables have
//! their initial values too. The former chastity has vanished!
//!
//! In other words, if we allow ourselves to test the condition
//! |ready_already=314159|, before |ready_already| has been
//! assigned a value, we can avoid the lengthy initialization. Dirty tricks
//! rarely pay off so handsomely.
//! @^dirty \PASCAL@>
//! @^system dependencies@>
//!
//! On systems that allow such preloading, the standard program called \.{TeX}
//! should be the one that has \.{plain} format preloaded, since that agrees
//! with {\sl The \TeX book}. Other versions, e.g., \.{AmSTeX}, should also
//! @:TeXbook}{\sl The \TeX book@>
//! @.AmSTeX@>
//! @.plain@>
//! be provided for commonly used formats.
//!
//! @<Glob...@>=
//! @!ready_already:integer; {a sacrifice of purity for economy}
//!
//! @ Now this is really it: \TeX\ starts and ends here.
//!
//! The initial test involving |ready_already| should be deleted if the
//! \PASCAL\ runtime system is smart enough to detect such a ``mistake.''
//! @^system dependencies@>
//!
//! @p begin @!{|start_here|}
//! history:=fatal_error_stop; {in case we quit during initialization}
//! t_open_out; {open the terminal for output}
//! if ready_already=314159 then goto start_of_TEX;
//! @<Check the ``constant'' values...@>@;
//! if bad>0 then
//!   begin wterm_ln('Ouch---my internal constants have been clobbered!',
//!     '---case ',bad:1);
//! @.Ouch...clobbered@>
//!   goto final_end;
//!   end;
//! initialize; {set global variables to their starting values}
//! @!init if not get_strings_started then goto final_end;
//! init_prim; {call |primitive| for each primitive}
//! init_str_ptr:=str_ptr; init_pool_ptr:=pool_ptr; fix_date_and_time;
//! tini@/
//! ready_already:=314159;
//! start_of_TEX: @<Initialize the output routines@>;
//! @<Get the first line of input and prepare to start@>;
//! history:=spotless; {ready to go!}
//! main_control; {come to life}
//! final_cleanup; {prepare for death}
//! end_of_TEX: close_files_and_terminate;
//! final_end: ready_already:=0;
//! end.
//!
//! @ Here we do whatever is needed to complete \TeX's job gracefully on the
//! local operating system. The code here might come into play after a fatal
//! error; it must therefore consist entirely of ``safe'' operations that
//! cannot produce error messages. For example, it would be a mistake to call
//! |str_room| or |make_string| at this time, because a call on |overflow|
//! might lead to an infinite loop.
//! @^system dependencies@>
//!
//! Actually there's one way to get error messages, via |prepare_mag|;
//! but that can't cause infinite recursion.
//! @^recursion@>
//!
//! This program doesn't bother to close the input files that may still be open.
//!
//! @<Last-minute...@>=
//! procedure close_files_and_terminate;
//! var k:integer; {all-purpose index}
//! begin @<Finish the extensions@>;
//! @!stat if tracing_stats>0 then @<Output statistics about this job@>;@;@+tats@/
//! wake_up_terminal; @<Finish the \.{DVI} file@>;
//! if log_opened then
//!   begin wlog_cr; a_close(log_file); selector:=selector-2;
//!   if selector=term_only then
//!     begin print_nl("Transcript written on ");
//! @.Transcript written...@>
//!     slow_print(log_name); print_char(".");
//!     end;
//!   end;
//! end;
//!
//! @ The present section goes directly to the log file instead of using
//! |print| commands, because there's no need for these strings to take
//! up |str_pool| memory when a non-{\bf stat} version of \TeX\ is being used.
//!
//! @<Output statistics...@>=
//! if log_opened then
//!   begin wlog_ln(' ');
//!   wlog_ln('Here is how much of TeX''s memory',' you used:');
//! @.Here is how much...@>
//!   wlog(' ',str_ptr-init_str_ptr:1,' string');
//!   if str_ptr<>init_str_ptr+1 then wlog('s');
//!   wlog_ln(' out of ', max_strings-init_str_ptr:1);@/
//!   wlog_ln(' ',pool_ptr-init_pool_ptr:1,' string characters out of ',
//!     pool_size-init_pool_ptr:1);@/
//!   wlog_ln(' ',lo_mem_max-mem_min+mem_end-hi_mem_min+2:1,@|
//!     ' words of memory out of ',mem_end+1-mem_min:1);@/
//!   wlog_ln(' ',cs_count:1,' multiletter control sequences out of ',
//!     hash_size:1);@/
//!   wlog(' ',fmem_ptr:1,' words of font info for ',
//!     font_ptr-font_base:1,' font');
//!   if font_ptr<>font_base+1 then wlog('s');
//!   wlog_ln(', out of ',font_mem_size:1,' for ',font_max-font_base:1);@/
//!   wlog(' ',hyph_count:1,' hyphenation exception');
//!   if hyph_count<>1 then wlog('s');
//!   wlog_ln(' out of ',hyph_size:1);@/
//!   wlog_ln(' ',max_in_stack:1,'i,',max_nest_stack:1,'n,',@|
//!     max_param_stack:1,'p,',@|
//!     max_buf_stack+1:1,'b,',@|
//!     max_save_stack+6:1,'s stack positions out of ',@|
//!     stack_size:1,'i,',
//!     nest_size:1,'n,',
//!     param_size:1,'p,',
//!     buf_size:1,'b,',
//!     save_size:1,'s');
//!   end
//!
//! @ We get to the |final_cleanup| routine when \.{\\end} or \.{\\dump} has
//! been scanned and |its_all_over|\kern-2pt.
//!
//! @<Last-minute...@>=
//! procedure final_cleanup;
//! label exit;
//! var c:small_number; {0 for \.{\\end}, 1 for \.{\\dump}}
//! begin c:=cur_chr;
//! if job_name=0 then open_log_file;
//! while input_ptr>0 do
//!   if state=token_list then end_token_list@+else end_file_reading;
//! while open_parens>0 do
//!   begin print(" )"); decr(open_parens);
//!   end;
//! if cur_level>level_one then
//!   begin print_nl("("); print_esc("end occurred ");
//!   print("inside a group at level ");
//! @:end_}{\.{(\\end occurred...)}@>
//!   print_int(cur_level-level_one); print_char(")");
//!   end;
//! while cond_ptr<>null do
//!   begin print_nl("("); print_esc("end occurred ");
//!   print("when "); print_cmd_chr(if_test,cur_if);
//!   if if_line<>0 then
//!     begin print(" on line "); print_int(if_line);
//!     end;
//!   print(" was incomplete)");
//!   if_line:=if_line_field(cond_ptr);
//!   cur_if:=subtype(cond_ptr); temp_ptr:=cond_ptr;
//!   cond_ptr:=link(cond_ptr); free_node(temp_ptr,if_node_size);
//!   end;
//! if history<>spotless then
//!  if ((history=warning_issued)or(interaction<error_stop_mode)) then
//!   if selector=term_and_log then
//!   begin selector:=term_only;
//!   print_nl("(see the transcript file for additional information)");
//! @.see the transcript file...@>
//!   selector:=term_and_log;
//!   end;
//! if c=1 then
//!   begin @!init for c:=top_mark_code to split_bot_mark_code do
//!     if cur_mark[c]<>null then delete_token_ref(cur_mark[c]);
//!   if last_glue<>max_halfword then delete_glue_ref(last_glue);
//!   store_fmt_file; return;@+tini@/
//!   print_nl("(\dump is performed only by INITEX)"); return;
//! @:dump_}{\.{\\dump...only by INITEX}@>
//!   end;
//! exit:end;
//!
//! @ @<Last-minute...@>=
//! @!init procedure init_prim; {initialize all the primitives}
//! begin no_new_control_sequence:=false;
//! @<Put each...@>;
//! no_new_control_sequence:=true;
//! end;
//! tini
//!
//! @ When we begin the following code, \TeX's tables may still contain garbage;
//! the strings might not even be present. Thus we must proceed cautiously to get
//! bootstrapped in.
//!
//! But when we finish this part of the program, \TeX\ is ready to call on the
//! |main_control| routine to do its work.
//!
//! @<Get the first line...@>=
//! begin @<Initialize the input routines@>;
//! if (format_ident=0)or(buffer[loc]="&") then
//!   begin if format_ident<>0 then initialize; {erase preloaded format}
//!   if not open_fmt_file then goto final_end;
//!   if not load_fmt_file then
//!     begin w_close(fmt_file); goto final_end;
//!     end;
//!   w_close(fmt_file);
//!   while (loc<limit)and(buffer[loc]=" ") do incr(loc);
//!   end;
//! if end_line_char_inactive then decr(limit)
//! else  buffer[limit]:=end_line_char;
//! fix_date_and_time;@/
//! @<Compute the magic offset@>;
//! @<Initialize the print |selector|...@>;
//! if (loc<limit)and(cat_code(buffer[loc])<>escape) then start_input;
//!   {\.{\\input} assumed}
//! end
//!
