//! @ @<Set init...@>=
//! format_ident:=0;
//!
//! @ @<Initialize table entries...@>=
//! format_ident:=" (INITEX)";
//!
//! @ @<Declare act...@>=
//! @!init procedure store_fmt_file;
//! label found1,found2,done1,done2;
//! var j,@!k,@!l:integer; {all-purpose indices}
//! @!p,@!q: pointer; {all-purpose pointers}
//! @!x: integer; {something to dump}
//! @!w: four_quarters; {four ASCII codes}
//! begin @<If dumping is not allowed, abort@>;
//! @<Create the |format_ident|, open the format file,
//!   and inform the user that dumping has begun@>;
//! @<Dump constants for consistency check@>;
//! @<Dump the string pool@>;
//! @<Dump the dynamic memory@>;
//! @<Dump the table of equivalents@>;
//! @<Dump the font information@>;
//! @<Dump the hyphenation tables@>;
//! @<Dump a couple more things and the closing check word@>;
//! @<Close the format file@>;
//! end;
//! tini
//!
//! @ Corresponding to the procedure that dumps a format file, we have a function
//! that reads one in. The function returns |false| if the dumped format is
//! incompatible with the present \TeX\ table sizes, etc.
//!
//! @d bad_fmt=6666 {go here if the format file is unacceptable}
//! @d too_small(#)==begin wake_up_terminal;
//!   wterm_ln('---! Must increase the ',#);
//! @.Must increase the x@>
//!   goto bad_fmt;
//!   end
//!
//! @p @t\4@>@<Declare the function called |open_fmt_file|@>@;
//! function load_fmt_file:boolean;
//! label bad_fmt,exit;
//! var j,@!k:integer; {all-purpose indices}
//! @!p,@!q: pointer; {all-purpose pointers}
//! @!x: integer; {something undumped}
//! @!w: four_quarters; {four ASCII codes}
//! begin @<Undump constants for consistency check@>;
//! @<Undump the string pool@>;
//! @<Undump the dynamic memory@>;
//! @<Undump the table of equivalents@>;
//! @<Undump the font information@>;
//! @<Undump the hyphenation tables@>;
//! @<Undump a couple more things and the closing check word@>;
//! load_fmt_file:=true; return; {it worked!}
//! bad_fmt: wake_up_terminal;
//!   wterm_ln('(Fatal format file error; I''m stymied)');
//! @.Fatal format file error@>
//! load_fmt_file:=false;
//! exit:end;
//!
//! @ The user is not allowed to dump a format file unless |save_ptr=0|.
//! This condition implies that |cur_level=level_one|, hence
//! the |xeq_level| array is constant and it need not be dumped.
//!
//! @<If dumping is not allowed, abort@>=
//! if save_ptr<>0 then
//!   begin print_err("You can't dump inside a group");
//! @.You can't dump...@>
//!   help1("`{...\dump}' is a no-no."); succumb;
//!   end
//!
//! @ Format files consist of |memory_word| items, and we use the following
//! macros to dump words of different types:
//!
//! @d dump_wd(#)==begin fmt_file^:=#; put(fmt_file);@+end
//! @d dump_int(#)==begin fmt_file^.int:=#; put(fmt_file);@+end
//! @d dump_hh(#)==begin fmt_file^.hh:=#; put(fmt_file);@+end
//! @d dump_qqqq(#)==begin fmt_file^.qqqq:=#; put(fmt_file);@+end
//!
//! @<Glob...@>=
//! @!fmt_file:word_file; {for input or output of format information}
//!
//! @ The inverse macros are slightly more complicated, since we need to check
//! the range of the values we are reading in. We say `|undump(a)(b)(x)|' to
//! read an integer value |x| that is supposed to be in the range |a<=x<=b|.
//!
//! @d undump_wd(#)==begin get(fmt_file); #:=fmt_file^;@+end
//! @d undump_int(#)==begin get(fmt_file); #:=fmt_file^.int;@+end
//! @d undump_hh(#)==begin get(fmt_file); #:=fmt_file^.hh;@+end
//! @d undump_qqqq(#)==begin get(fmt_file); #:=fmt_file^.qqqq;@+end
//! @d undump_end_end(#)==#:=x;@+end
//! @d undump_end(#)==(x>#) then goto bad_fmt@+else undump_end_end
//! @d undump(#)==begin undump_int(x); if (x<#) or undump_end
//! @d undump_size_end_end(#)==too_small(#)@+else undump_end_end
//! @d undump_size_end(#)==if x># then undump_size_end_end
//! @d undump_size(#)==begin undump_int(x);
//!   if x<# then goto bad_fmt; undump_size_end
//!
//! @ The next few sections of the program should make it clear how we use the
//! dump/undump macros.
//!
//! @<Dump constants for consistency check@>=
//! dump_int(@$);@/
//! dump_int(mem_bot);@/
//! dump_int(mem_top);@/
//! dump_int(eqtb_size);@/
//! dump_int(hash_prime);@/
//! dump_int(hyph_size)
//!
//! @ Sections of a \.{WEB} program that are ``commented out'' still contribute
//! strings to the string pool; therefore \.{INITEX} and \TeX\ will have
//! the same strings. (And it is, of course, a good thing that they do.)
//! @.WEB@>
//! @^string pool@>
//!
//! @<Undump constants for consistency check@>=
//! x:=fmt_file^.int;
//! if x<>@$ then goto bad_fmt; {check that strings are the same}
//! undump_int(x);
//! if x<>mem_bot then goto bad_fmt;
//! undump_int(x);
//! if x<>mem_top then goto bad_fmt;
//! undump_int(x);
//! if x<>eqtb_size then goto bad_fmt;
//! undump_int(x);
//! if x<>hash_prime then goto bad_fmt;
//! undump_int(x);
//! if x<>hyph_size then goto bad_fmt
//!
//! @ @d dump_four_ASCII==
//!   w.b0:=qi(so(str_pool[k])); w.b1:=qi(so(str_pool[k+1]));
//!   w.b2:=qi(so(str_pool[k+2])); w.b3:=qi(so(str_pool[k+3]));
//!   dump_qqqq(w)
//!
//! @<Dump the string pool@>=
//! dump_int(pool_ptr);
//! dump_int(str_ptr);
//! for k:=0 to str_ptr do dump_int(str_start[k]);
//! k:=0;
//! while k+4<pool_ptr do
//!   begin dump_four_ASCII; k:=k+4;
//!   end;
//! k:=pool_ptr-4; dump_four_ASCII;
//! print_ln; print_int(str_ptr); print(" strings of total length ");
//! print_int(pool_ptr)
//!
//! @ @d undump_four_ASCII==
//!   undump_qqqq(w);
//!   str_pool[k]:=si(qo(w.b0)); str_pool[k+1]:=si(qo(w.b1));
//!   str_pool[k+2]:=si(qo(w.b2)); str_pool[k+3]:=si(qo(w.b3))
//!
//! @<Undump the string pool@>=
//! undump_size(0)(pool_size)('string pool size')(pool_ptr);
//! undump_size(0)(max_strings)('max strings')(str_ptr);
//! for k:=0 to str_ptr do undump(0)(pool_ptr)(str_start[k]);
//! k:=0;
//! while k+4<pool_ptr do
//!   begin undump_four_ASCII; k:=k+4;
//!   end;
//! k:=pool_ptr-4; undump_four_ASCII;
//! init_str_ptr:=str_ptr; init_pool_ptr:=pool_ptr
//!
//! @ By sorting the list of available spaces in the variable-size portion of
//! |mem|, we are usually able to get by without having to dump very much
//! of the dynamic memory.
//!
//! We recompute |var_used| and |dyn_used|, so that \.{INITEX} dumps valid
//! information even when it has not been gathering statistics.
//!
//! @<Dump the dynamic memory@>=
//! sort_avail; var_used:=0;
//! dump_int(lo_mem_max); dump_int(rover);
//! p:=mem_bot; q:=rover; x:=0;
//! repeat for k:=p to q+1 do dump_wd(mem[k]);
//! x:=x+q+2-p; var_used:=var_used+q-p;
//! p:=q+node_size(q); q:=rlink(q);
//! until q=rover;
//! var_used:=var_used+lo_mem_max-p; dyn_used:=mem_end+1-hi_mem_min;@/
//! for k:=p to lo_mem_max do dump_wd(mem[k]);
//! x:=x+lo_mem_max+1-p;
//! dump_int(hi_mem_min); dump_int(avail);
//! for k:=hi_mem_min to mem_end do dump_wd(mem[k]);
//! x:=x+mem_end+1-hi_mem_min;
//! p:=avail;
//! while p<>null do
//!   begin decr(dyn_used); p:=link(p);
//!   end;
//! dump_int(var_used); dump_int(dyn_used);
//! print_ln; print_int(x);
//! print(" memory locations dumped; current usage is ");
//! print_int(var_used); print_char("&"); print_int(dyn_used)
//!
//! @ @<Undump the dynamic memory@>=
//! undump(lo_mem_stat_max+1000)(hi_mem_stat_min-1)(lo_mem_max);
//! undump(lo_mem_stat_max+1)(lo_mem_max)(rover);
//! p:=mem_bot; q:=rover;
//! repeat for k:=p to q+1 do undump_wd(mem[k]);
//! p:=q+node_size(q);
//! if (p>lo_mem_max)or((q>=rlink(q))and(rlink(q)<>rover)) then goto bad_fmt;
//! q:=rlink(q);
//! until q=rover;
//! for k:=p to lo_mem_max do undump_wd(mem[k]);
//! if mem_min<mem_bot-2 then {make more low memory available}
//!   begin p:=llink(rover); q:=mem_min+1;
//!   link(mem_min):=null; info(mem_min):=null; {we don't use the bottom word}
//!   rlink(p):=q; llink(rover):=q;@/
//!   rlink(q):=rover; llink(q):=p; link(q):=empty_flag;
//!   node_size(q):=mem_bot-q;
//!   end;
//! undump(lo_mem_max+1)(hi_mem_stat_min)(hi_mem_min);
//! undump(null)(mem_top)(avail); mem_end:=mem_top;
//! for k:=hi_mem_min to mem_end do undump_wd(mem[k]);
//! undump_int(var_used); undump_int(dyn_used)
//!
//! @ @<Dump the table of equivalents@>=
//! @<Dump regions 1 to 4 of |eqtb|@>;
//! @<Dump regions 5 and 6 of |eqtb|@>;
//! dump_int(par_loc); dump_int(write_loc);@/
//! @<Dump the hash table@>
//!
//! @ @<Undump the table of equivalents@>=
//! @<Undump regions 1 to 6 of |eqtb|@>;
//! undump(hash_base)(frozen_control_sequence)(par_loc);
//! par_token:=cs_token_flag+par_loc;@/
//! undump(hash_base)(frozen_control_sequence)(write_loc);@/
//! @<Undump the hash table@>
//!
//! @ The table of equivalents usually contains repeated information, so we dump it
//! in compressed form: The sequence of $n+2$ values $(n,x_1,\ldots,x_n,m)$ in the
//! format file represents $n+m$ consecutive entries of |eqtb|, with |m| extra
//! copies of $x_n$, namely $(x_1,\ldots,x_n,x_n,\ldots,x_n)$.
//!
//! @<Dump regions 1 to 4 of |eqtb|@>=
//! k:=active_base;
//! repeat j:=k;
//! while j<int_base-1 do
//!   begin if (equiv(j)=equiv(j+1))and(eq_type(j)=eq_type(j+1))and@|
//!     (eq_level(j)=eq_level(j+1)) then goto found1;
//!   incr(j);
//!   end;
//! l:=int_base; goto done1; {|j=int_base-1|}
//! found1: incr(j); l:=j;
//! while j<int_base-1 do
//!   begin if (equiv(j)<>equiv(j+1))or(eq_type(j)<>eq_type(j+1))or@|
//!     (eq_level(j)<>eq_level(j+1)) then goto done1;
//!   incr(j);
//!   end;
//! done1:dump_int(l-k);
//! while k<l do
//!   begin dump_wd(eqtb[k]); incr(k);
//!   end;
//! k:=j+1; dump_int(k-l);
//! until k=int_base
//!
//! @ @<Dump regions 5 and 6 of |eqtb|@>=
//! repeat j:=k;
//! while j<eqtb_size do
//!   begin if eqtb[j].int=eqtb[j+1].int then goto found2;
//!   incr(j);
//!   end;
//! l:=eqtb_size+1; goto done2; {|j=eqtb_size|}
//! found2: incr(j); l:=j;
//! while j<eqtb_size do
//!   begin if eqtb[j].int<>eqtb[j+1].int then goto done2;
//!   incr(j);
//!   end;
//! done2:dump_int(l-k);
//! while k<l do
//!   begin dump_wd(eqtb[k]); incr(k);
//!   end;
//! k:=j+1; dump_int(k-l);
//! until k>eqtb_size
//!
//! @ @<Undump regions 1 to 6 of |eqtb|@>=
//! k:=active_base;
//! repeat undump_int(x);
//! if (x<1)or(k+x>eqtb_size+1) then goto bad_fmt;
//! for j:=k to k+x-1 do undump_wd(eqtb[j]);
//! k:=k+x;
//! undump_int(x);
//! if (x<0)or(k+x>eqtb_size+1) then goto bad_fmt;
//! for j:=k to k+x-1 do eqtb[j]:=eqtb[k-1];
//! k:=k+x;
//! until k>eqtb_size
//!
//! @ A different scheme is used to compress the hash table, since its lower
//! region is usually sparse. When |text(p)<>0| for |p<=hash_used|, we output
//! two words, |p| and |hash[p]|. The hash table is, of course, densely packed
//! for |p>=hash_used|, so the remaining entries are output in a~block.
//!
//! @<Dump the hash table@>=
//! dump_int(hash_used); cs_count:=frozen_control_sequence-1-hash_used;
//! for p:=hash_base to hash_used do if text(p)<>0 then
//!   begin dump_int(p); dump_hh(hash[p]); incr(cs_count);
//!   end;
//! for p:=hash_used+1 to undefined_control_sequence-1 do dump_hh(hash[p]);
//! dump_int(cs_count);@/
//! print_ln; print_int(cs_count); print(" multiletter control sequences")
//!
//! @ @<Undump the hash table@>=
//! undump(hash_base)(frozen_control_sequence)(hash_used); p:=hash_base-1;
//! repeat undump(p+1)(hash_used)(p); undump_hh(hash[p]);
//! until p=hash_used;
//! for p:=hash_used+1 to undefined_control_sequence-1 do undump_hh(hash[p]);
//! undump_int(cs_count)
//!
//! @ @<Dump the font information@>=
//! dump_int(fmem_ptr);
//! for k:=0 to fmem_ptr-1 do dump_wd(font_info[k]);
//! dump_int(font_ptr);
//! for k:=null_font to font_ptr do
//!   @<Dump the array info for internal font number |k|@>;
//! print_ln; print_int(fmem_ptr-7); print(" words of font info for ");
//! print_int(font_ptr-font_base); print(" preloaded font");
//! if font_ptr<>font_base+1 then print_char("s")
//!
//! @ @<Undump the font information@>=
//! undump_size(7)(font_mem_size)('font mem size')(fmem_ptr);
//! for k:=0 to fmem_ptr-1 do undump_wd(font_info[k]);
//! undump_size(font_base)(font_max)('font max')(font_ptr);
//! for k:=null_font to font_ptr do
//!   @<Undump the array info for internal font number |k|@>
//!
//! @ @<Dump the array info for internal font number |k|@>=
//! begin dump_qqqq(font_check[k]);
//! dump_int(font_size[k]);
//! dump_int(font_dsize[k]);
//! dump_int(font_params[k]);@/
//! dump_int(hyphen_char[k]);
//! dump_int(skew_char[k]);@/
//! dump_int(font_name[k]);
//! dump_int(font_area[k]);@/
//! dump_int(font_bc[k]);
//! dump_int(font_ec[k]);@/
//! dump_int(char_base[k]);
//! dump_int(width_base[k]);
//! dump_int(height_base[k]);@/
//! dump_int(depth_base[k]);
//! dump_int(italic_base[k]);
//! dump_int(lig_kern_base[k]);@/
//! dump_int(kern_base[k]);
//! dump_int(exten_base[k]);
//! dump_int(param_base[k]);@/
//! dump_int(font_glue[k]);@/
//! dump_int(bchar_label[k]);
//! dump_int(font_bchar[k]);
//! dump_int(font_false_bchar[k]);@/
//! print_nl("\font"); print_esc(font_id_text(k)); print_char("=");
//! print_file_name(font_name[k],font_area[k],"");
//! if font_size[k]<>font_dsize[k] then
//!   begin print(" at "); print_scaled(font_size[k]); print("pt");
//!   end;
//! end
//!
//! @ @<Undump the array info for internal font number |k|@>=
//! begin undump_qqqq(font_check[k]);@/
//! undump_int(font_size[k]);
//! undump_int(font_dsize[k]);
//! undump(min_halfword)(max_halfword)(font_params[k]);@/
//! undump_int(hyphen_char[k]);
//! undump_int(skew_char[k]);@/
//! undump(0)(str_ptr)(font_name[k]);
//! undump(0)(str_ptr)(font_area[k]);@/
//! undump(0)(255)(font_bc[k]);
//! undump(0)(255)(font_ec[k]);@/
//! undump_int(char_base[k]);
//! undump_int(width_base[k]);
//! undump_int(height_base[k]);@/
//! undump_int(depth_base[k]);
//! undump_int(italic_base[k]);
//! undump_int(lig_kern_base[k]);@/
//! undump_int(kern_base[k]);
//! undump_int(exten_base[k]);
//! undump_int(param_base[k]);@/
//! undump(min_halfword)(lo_mem_max)(font_glue[k]);@/
//! undump(0)(fmem_ptr-1)(bchar_label[k]);
//! undump(min_quarterword)(non_char)(font_bchar[k]);
//! undump(min_quarterword)(non_char)(font_false_bchar[k]);
//! end
//!
//! @ @<Dump the hyphenation tables@>=
//! dump_int(hyph_count);
//! for k:=0 to hyph_size do if hyph_word[k]<>0 then
//!   begin dump_int(k); dump_int(hyph_word[k]); dump_int(hyph_list[k]);
//!   end;
//! print_ln; print_int(hyph_count); print(" hyphenation exception");
//! if hyph_count<>1 then print_char("s");
//! if trie_not_ready then init_trie;
//! dump_int(trie_max);
//! for k:=0 to trie_max do dump_hh(trie[k]);
//! dump_int(trie_op_ptr);
//! for k:=1 to trie_op_ptr do
//!   begin dump_int(hyf_distance[k]);
//!   dump_int(hyf_num[k]);
//!   dump_int(hyf_next[k]);
//!   end;
//! print_nl("Hyphenation trie of length "); print_int(trie_max);
//! @.Hyphenation trie...@>
//! print(" has "); print_int(trie_op_ptr); print(" op");
//! if trie_op_ptr<>1 then print_char("s");
//! print(" out of "); print_int(trie_op_size);
//! for k:=255 downto 0 do if trie_used[k]>min_quarterword then
//!   begin print_nl("  "); print_int(qo(trie_used[k]));
//!   print(" for language "); print_int(k);
//!   dump_int(k); dump_int(qo(trie_used[k]));
//!   end
//!
//! @ Only ``nonempty'' parts of |op_start| need to be restored.
//!
//! @<Undump the hyphenation tables@>=
//! undump(0)(hyph_size)(hyph_count);
//! for k:=1 to hyph_count do
//!   begin undump(0)(hyph_size)(j);
//!   undump(0)(str_ptr)(hyph_word[j]);
//!   undump(min_halfword)(max_halfword)(hyph_list[j]);
//!   end;
//! undump_size(0)(trie_size)('trie size')(j); @+init trie_max:=j;@+tini
//! for k:=0 to j do undump_hh(trie[k]);
//! undump_size(0)(trie_op_size)('trie op size')(j); @+init trie_op_ptr:=j;@+tini
//! for k:=1 to j do
//!   begin undump(0)(63)(hyf_distance[k]); {a |small_number|}
//!   undump(0)(63)(hyf_num[k]);
//!   undump(min_quarterword)(max_quarterword)(hyf_next[k]);
//!   end;
//! init for k:=0 to 255 do trie_used[k]:=min_quarterword;@+tini@;@/
//! k:=256;
//! while j>0 do
//!   begin undump(0)(k-1)(k); undump(1)(j)(x);@+init trie_used[k]:=qi(x);@+tini@;@/
//!   j:=j-x; op_start[k]:=qo(j);
//!   end;
//! @!init trie_not_ready:=false @+tini
//!
//! @ We have already printed a lot of statistics, so we set |tracing_stats:=0|
//! to prevent them from appearing again.
//!
//! @<Dump a couple more things and the closing check word@>=
//! dump_int(interaction); dump_int(format_ident); dump_int(69069);
//! tracing_stats:=0
//!
//! @ @<Undump a couple more things and the closing check word@>=
//! undump(batch_mode)(error_stop_mode)(interaction);
//! undump(0)(str_ptr)(format_ident);
//! undump_int(x);
//! if (x<>69069)or eof(fmt_file) then goto bad_fmt
//!
//! @ @<Create the |format_ident|...@>=
//! selector:=new_string;
//! print(" (preloaded format="); print(job_name); print_char(" ");
//! print_int(year); print_char(".");
//! print_int(month); print_char("."); print_int(day); print_char(")");
//! if interaction=batch_mode then selector:=log_only
//! else selector:=term_and_log;
//! str_room(1);
//! format_ident:=make_string;
//! pack_job_name(format_extension);
//! while not w_open_out(fmt_file) do
//!   prompt_file_name("format file name",format_extension);
//! print_nl("Beginning to dump on file ");
//! @.Beginning to dump...@>
//! slow_print(w_make_name_string(fmt_file)); flush_string;
//! print_nl(""); slow_print(format_ident)
//!
//! @ @<Close the format file@>=
//! w_close(fmt_file)
//!
