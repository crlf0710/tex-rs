//! @* \[18] The hash table.
//! Control sequences are stored and retrieved by means of a fairly standard hash
//! table algorithm called the method of ``coalescing lists'' (cf.\ Algorithm 6.4C
//! in {\sl The Art of Computer Programming\/}). Once a control sequence enters the
//! table, it is never removed, because there are complicated situations
//! involving \.{\\gdef} where the removal of a control sequence at the end of
//! a group would be a mistake preventable only by the introduction of a
//! complicated reference-count mechanism.
//!
//! The actual sequence of letters forming a control sequence identifier is
//! stored in the |str_pool| array together with all the other strings. An
//! auxiliary array |hash| consists of items with two halfword fields per
//! word. The first of these, called |next(p)|, points to the next identifier
//! belonging to the same coalesced list as the identifier corresponding to~|p|;
//! and the other, called |text(p)|, points to the |str_start| entry for
//! |p|'s identifier. If position~|p| of the hash table is empty, we have
//! |text(p)=0|; if position |p| is either empty or the end of a coalesced
//! hash list, we have |next(p)=0|. An auxiliary pointer variable called
//! |hash_used| is maintained in such a way that all locations |p>=hash_used|
//! are nonempty. The global variable |cs_count| tells how many multiletter
//! control sequences have been defined, if statistics are being kept.
//!
//! A global boolean variable called |no_new_control_sequence| is set to
//! |true| during the time that new hash table entries are forbidden.
//!
//! @d next(#) == hash[#].lh {link for coalesced lists}
//! @d text(#) == hash[#].rh {string number for control sequence name}
//! @d hash_is_full == (hash_used=hash_base) {test if all positions are occupied}
//! @d font_id_text(#) == text(font_id_base+#) {a frozen font identifier's name}
//!
//! @<Glob...@>=
//! @!hash: array[hash_base..undefined_control_sequence-1] of two_halves;
//!   {the hash table}
//! @!hash_used:pointer; {allocation pointer for |hash|}
//! @!no_new_control_sequence:boolean; {are new identifiers legal?}
//! @!cs_count:integer; {total number of known identifiers}
//!
//! @ @<Set init...@>=
//! no_new_control_sequence:=true; {new identifiers are usually forbidden}
//! next(hash_base):=0; text(hash_base):=0;
//! for k:=hash_base+1 to undefined_control_sequence-1 do hash[k]:=hash[hash_base];
//!
//! @ @<Initialize table entries...@>=
//! hash_used:=frozen_control_sequence; {nothing is used}
//! cs_count:=0;
//! eq_type(frozen_dont_expand):=dont_expand;
//! text(frozen_dont_expand):="notexpanded:";
//! @.notexpanded:@>
//!
//! @ Here is the subroutine that searches the hash table for an identifier
//! that matches a given string of length |l>1| appearing in |buffer[j..
//! (j+l-1)]|. If the identifier is found, the corresponding hash table address
//! is returned. Otherwise, if the global variable |no_new_control_sequence|
//! is |true|, the dummy address |undefined_control_sequence| is returned.
//! Otherwise the identifier is inserted into the hash table and its location
//! is returned.
//!
//! @p function id_lookup(@!j,@!l:integer):pointer; {search the hash table}
//! label found; {go here if you found it}
//! var h:integer; {hash code}
//! @!d:integer; {number of characters in incomplete current string}
//! @!p:pointer; {index in |hash| array}
//! @!k:pointer; {index in |buffer| array}
//! begin @<Compute the hash code |h|@>;
//! p:=h+hash_base; {we start searching here; note that |0<=h<hash_prime|}
//! loop@+begin if text(p)>0 then if length(text(p))=l then
//!     if str_eq_buf(text(p),j) then goto found;
//!   if next(p)=0 then
//!     begin if no_new_control_sequence then
//!       p:=undefined_control_sequence
//!     else @<Insert a new control sequence after |p|, then make
//!       |p| point to it@>;
//!     goto found;
//!     end;
//!   p:=next(p);
//!   end;
//! found: id_lookup:=p;
//! end;
//!
//! @ @<Insert a new control...@>=
//! begin if text(p)>0 then
//!   begin repeat if hash_is_full then overflow("hash size",hash_size);
//! @:TeX capacity exceeded hash size}{\quad hash size@>
//!   decr(hash_used);
//!   until text(hash_used)=0; {search for an empty location in |hash|}
//!   next(p):=hash_used; p:=hash_used;
//!   end;
//! str_room(l); d:=cur_length;
//! while pool_ptr>str_start[str_ptr] do
//!   begin decr(pool_ptr); str_pool[pool_ptr+l]:=str_pool[pool_ptr];
//!   end; {move current string up to make room for another}
//! for k:=j to j+l-1 do append_char(buffer[k]);
//! text(p):=make_string; pool_ptr:=pool_ptr+d;
//! @!stat incr(cs_count);@+tats@;@/
//! end
//!
//! @ The value of |hash_prime| should be roughly 85\pct! of |hash_size|, and it
//! should be a prime number.  The theory of hashing tells us to expect fewer
//! than two table probes, on the average, when the search is successful.
//! [See J.~S. Vitter, {\sl Journal of the ACM\/ \bf30} (1983), 231--258.]
//! @^Vitter, Jeffrey Scott@>
//!
//! @<Compute the hash code |h|@>=
//! h:=buffer[j];
//! for k:=j+1 to j+l-1 do
//!   begin h:=h+h+buffer[k];
//!   while h>=hash_prime do h:=h-hash_prime;
//!   end
//!
//! @ Single-character control sequences do not need to be looked up in a hash
//! table, since we can use the character code itself as a direct address.
//! The procedure |print_cs| prints the name of a control sequence, given
//! a pointer to its address in |eqtb|. A space is printed after the name
//! unless it is a single nonletter or an active character. This procedure
//! might be invoked with invalid data, so it is ``extra robust.'' The
//! individual characters must be printed one at a time using |print|, since
//! they may be unprintable.
//!
//! @<Basic printing...@>=
//! procedure print_cs(@!p:integer); {prints a purported control sequence}
//! begin if p<hash_base then {single character}
//!   if p>=single_base then
//!     if p=null_cs then
//!       begin print_esc("csname"); print_esc("endcsname"); print_char(" ");
//!       end
//!     else  begin print_esc(p-single_base);
//!       if cat_code(p-single_base)=letter then print_char(" ");
//!       end
//!   else if p<active_base then print_esc("IMPOSSIBLE.")
//! @.IMPOSSIBLE@>
//!   else print(p-active_base)
//! else if p>=undefined_control_sequence then print_esc("IMPOSSIBLE.")
//! else if (text(p)<0)or(text(p)>=str_ptr) then print_esc("NONEXISTENT.")
//! @.NONEXISTENT@>
//! else  begin print_esc(text(p));
//!   print_char(" ");
//!   end;
//! end;
//!
//! @ Here is a similar procedure; it avoids the error checks, and it never
//! prints a space after the control sequence.
//!
//! @<Basic printing procedures@>=
//! procedure sprint_cs(@!p:pointer); {prints a control sequence}
//! begin if p<hash_base then
//!   if p<single_base then print(p-active_base)
//!   else  if p<null_cs then print_esc(p-single_base)
//!     else  begin print_esc("csname"); print_esc("endcsname");
//!       end
//! else print_esc(text(p));
//! end;
//!
//! @ We need to put \TeX's ``primitive'' control sequences into the hash
//! table, together with their command code (which will be the |eq_type|)
//! and an operand (which will be the |equiv|). The |primitive| procedure
//! does this, in a way that no \TeX\ user can. The global value |cur_val|
//! contains the new |eqtb| pointer after |primitive| has acted.
//!
//! @p @!init procedure primitive(@!s:str_number;@!c:quarterword;@!o:halfword);
//! var k:pool_pointer; {index into |str_pool|}
//! @!j:small_number; {index into |buffer|}
//! @!l:small_number; {length of the string}
//! begin if s<256 then cur_val:=s+single_base
//! else  begin k:=str_start[s]; l:=str_start[s+1]-k;
//!     {we will move |s| into the (empty) |buffer|}
//!   for j:=0 to l-1 do buffer[j]:=so(str_pool[k+j]);
//!   cur_val:=id_lookup(0,l); {|no_new_control_sequence| is |false|}
//!   flush_string; text(cur_val):=s; {we don't want to have the string twice}
//!   end;
//! eq_level(cur_val):=level_one; eq_type(cur_val):=c; equiv(cur_val):=o;
//! end;
//! tini
//!
//! @ Many of \TeX's primitives need no |equiv|, since they are identifiable
//! by their |eq_type| alone. These primitives are loaded into the hash table
//! as follows:
//!
//! @<Put each of \TeX's primitives into the hash table@>=
//! primitive(" ",ex_space,0);@/
//! @!@:Single-character primitives /}{\quad\.{\\\ }@>
//! primitive("/",ital_corr,0);@/
//! @!@:Single-character primitives /}{\quad\.{\\/}@>
//! primitive("accent",accent,0);@/
//! @!@:accent_}{\.{\\accent} primitive@>
//! primitive("advance",advance,0);@/
//! @!@:advance_}{\.{\\advance} primitive@>
//! primitive("afterassignment",after_assignment,0);@/
//! @!@:after_assignment_}{\.{\\afterassignment} primitive@>
//! primitive("aftergroup",after_group,0);@/
//! @!@:after_group_}{\.{\\aftergroup} primitive@>
//! primitive("begingroup",begin_group,0);@/
//! @!@:begin_group_}{\.{\\begingroup} primitive@>
//! primitive("char",char_num,0);@/
//! @!@:char_}{\.{\\char} primitive@>
//! primitive("csname",cs_name,0);@/
//! @!@:cs_name_}{\.{\\csname} primitive@>
//! primitive("delimiter",delim_num,0);@/
//! @!@:delimiter_}{\.{\\delimiter} primitive@>
//! primitive("divide",divide,0);@/
//! @!@:divide_}{\.{\\divide} primitive@>
//! primitive("endcsname",end_cs_name,0);@/
//! @!@:end_cs_name_}{\.{\\endcsname} primitive@>
//! primitive("endgroup",end_group,0);
//! @!@:end_group_}{\.{\\endgroup} primitive@>
//! text(frozen_end_group):="endgroup"; eqtb[frozen_end_group]:=eqtb[cur_val];@/
//! primitive("expandafter",expand_after,0);@/
//! @!@:expand_after_}{\.{\\expandafter} primitive@>
//! primitive("font",def_font,0);@/
//! @!@:font_}{\.{\\font} primitive@>
//! primitive("fontdimen",assign_font_dimen,0);@/
//! @!@:font_dimen_}{\.{\\fontdimen} primitive@>
//! primitive("halign",halign,0);@/
//! @!@:halign_}{\.{\\halign} primitive@>
//! primitive("hrule",hrule,0);@/
//! @!@:hrule_}{\.{\\hrule} primitive@>
//! primitive("ignorespaces",ignore_spaces,0);@/
//! @!@:ignore_spaces_}{\.{\\ignorespaces} primitive@>
//! primitive("insert",insert,0);@/
//! @!@:insert_}{\.{\\insert} primitive@>
//! primitive("mark",mark,0);@/
//! @!@:mark_}{\.{\\mark} primitive@>
//! primitive("mathaccent",math_accent,0);@/
//! @!@:math_accent_}{\.{\\mathaccent} primitive@>
//! primitive("mathchar",math_char_num,0);@/
//! @!@:math_char_}{\.{\\mathchar} primitive@>
//! primitive("mathchoice",math_choice,0);@/
//! @!@:math_choice_}{\.{\\mathchoice} primitive@>
//! primitive("multiply",multiply,0);@/
//! @!@:multiply_}{\.{\\multiply} primitive@>
//! primitive("noalign",no_align,0);@/
//! @!@:no_align_}{\.{\\noalign} primitive@>
//! primitive("noboundary",no_boundary,0);@/
//! @!@:no_boundary_}{\.{\\noboundary} primitive@>
//! primitive("noexpand",no_expand,0);@/
//! @!@:no_expand_}{\.{\\noexpand} primitive@>
//! primitive("nonscript",non_script,0);@/
//! @!@:non_script_}{\.{\\nonscript} primitive@>
//! primitive("omit",omit,0);@/
//! @!@:omit_}{\.{\\omit} primitive@>
//! primitive("parshape",set_shape,0);@/
//! @!@:par_shape_}{\.{\\parshape} primitive@>
//! primitive("penalty",break_penalty,0);@/
//! @!@:penalty_}{\.{\\penalty} primitive@>
//! primitive("prevgraf",set_prev_graf,0);@/
//! @!@:prev_graf_}{\.{\\prevgraf} primitive@>
//! primitive("radical",radical,0);@/
//! @!@:radical_}{\.{\\radical} primitive@>
//! primitive("read",read_to_cs,0);@/
//! @!@:read_}{\.{\\read} primitive@>
//! primitive("relax",relax,256); {cf.\ |scan_file_name|}
//! @!@:relax_}{\.{\\relax} primitive@>
//! text(frozen_relax):="relax"; eqtb[frozen_relax]:=eqtb[cur_val];@/
//! primitive("setbox",set_box,0);@/
//! @!@:set_box_}{\.{\\setbox} primitive@>
//! primitive("the",the,0);@/
//! @!@:the_}{\.{\\the} primitive@>
//! primitive("toks",toks_register,0);@/
//! @!@:toks_}{\.{\\toks} primitive@>
//! primitive("vadjust",vadjust,0);@/
//! @!@:vadjust_}{\.{\\vadjust} primitive@>
//! primitive("valign",valign,0);@/
//! @!@:valign_}{\.{\\valign} primitive@>
//! primitive("vcenter",vcenter,0);@/
//! @!@:vcenter_}{\.{\\vcenter} primitive@>
//! primitive("vrule",vrule,0);@/
//! @!@:vrule_}{\.{\\vrule} primitive@>
//!
//! @ Each primitive has a corresponding inverse, so that it is possible to
//! display the cryptic numeric contents of |eqtb| in symbolic form.
//! Every call of |primitive| in this program is therefore accompanied by some
//! straightforward code that forms part of the |print_cmd_chr| routine
//! below.
//!
//! @<Cases of |print_cmd_chr|...@>=
//! accent: print_esc("accent");
//! advance: print_esc("advance");
//! after_assignment: print_esc("afterassignment");
//! after_group: print_esc("aftergroup");
//! assign_font_dimen: print_esc("fontdimen");
//! begin_group: print_esc("begingroup");
//! break_penalty: print_esc("penalty");
//! char_num: print_esc("char");
//! cs_name: print_esc("csname");
//! def_font: print_esc("font");
//! delim_num: print_esc("delimiter");
//! divide: print_esc("divide");
//! end_cs_name: print_esc("endcsname");
//! end_group: print_esc("endgroup");
//! ex_space: print_esc(" ");
//! expand_after: print_esc("expandafter");
//! halign: print_esc("halign");
//! hrule: print_esc("hrule");
//! ignore_spaces: print_esc("ignorespaces");
//! insert: print_esc("insert");
//! ital_corr: print_esc("/");
//! mark: print_esc("mark");
//! math_accent: print_esc("mathaccent");
//! math_char_num: print_esc("mathchar");
//! math_choice: print_esc("mathchoice");
//! multiply: print_esc("multiply");
//! no_align: print_esc("noalign");
//! no_boundary:print_esc("noboundary");
//! no_expand: print_esc("noexpand");
//! non_script: print_esc("nonscript");
//! omit: print_esc("omit");
//! radical: print_esc("radical");
//! read_to_cs: print_esc("read");
//! relax: print_esc("relax");
//! set_box: print_esc("setbox");
//! set_prev_graf: print_esc("prevgraf");
//! set_shape: print_esc("parshape");
//! the: print_esc("the");
//! toks_register: print_esc("toks");
//! vadjust: print_esc("vadjust");
//! valign: print_esc("valign");
//! vcenter: print_esc("vcenter");
//! vrule: print_esc("vrule");
//!
//! @ We will deal with the other primitives later, at some point in the program
//! where their |eq_type| and |equiv| values are more meaningful.  For example,
//! the primitives for math mode will be loaded when we consider the routines
//! that deal with formulas. It is easy to find where each particular
//! primitive was treated by looking in the index at the end; for example, the
//! section where |"radical"| entered |eqtb| is listed under `\.{\\radical}
//! primitive'. (Primitives consisting of a single nonalphabetic character,
//! @!like `\.{\\/}', are listed under `Single-character primitives'.)
//! @!@^Single-character primitives@>
//!
//! Meanwhile, this is a convenient place to catch up on something we were unable
//! to do before the hash table was defined:
//!
//! @<Print the font identifier for |font(p)|@>=
//! print_esc(font_id_text(font(p)))
//!
