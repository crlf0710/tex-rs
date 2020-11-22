//! @ The global variable |null_character| is set up to be a word of
//! |char_info| for a character that doesn't exist. Such a word provides a
//! convenient way to deal with erroneous situations.
//!
//! @<Glob...@>=
//! @!null_character:four_quarters; {nonexistent character information}
//!
//! @ @<Set init...@>=
//! null_character.b0:=min_quarterword; null_character.b1:=min_quarterword;
//! null_character.b2:=min_quarterword; null_character.b3:=min_quarterword;
//!
//! @ Here are some macros that help process ligatures and kerns.
//! We write |char_kern(f)(j)| to find the amount of kerning specified by
//! kerning command~|j| in font~|f|. If |j| is the |char_info| for a character
//! with a ligature/kern program, the first instruction of that program is either
//! |i=font_info[lig_kern_start(f)(j)]| or |font_info[lig_kern_restart(f)(i)]|,
//! depending on whether or not |skip_byte(i)<=stop_flag|.
//!
//! The constant |kern_base_offset| should be simplified, for \PASCAL\ compilers
//! that do not do local optimization.
//! @^system dependencies@>
//!
//! @d char_kern_end(#)==256*op_byte(#)+rem_byte(#)].sc
//! @d char_kern(#)==font_info[kern_base[#]+char_kern_end
//! @d kern_base_offset==256*(128+min_quarterword)
//! @d lig_kern_start(#)==lig_kern_base[#]+rem_byte {beginning of lig/kern program}
//! @d lig_kern_restart_end(#)==256*op_byte(#)+rem_byte(#)+32768-kern_base_offset
//! @d lig_kern_restart(#)==lig_kern_base[#]+lig_kern_restart_end
//!
//! @ Font parameters are referred to as |slant(f)|, |space(f)|, etc.
//!
//! @d param_end(#)==param_base[#]].sc
//! @d param(#)==font_info[#+param_end
//! @d slant==param(slant_code) {slant to the right, per unit distance upward}
//! @d space==param(space_code) {normal space between words}
//! @d space_stretch==param(space_stretch_code) {stretch between words}
//! @d space_shrink==param(space_shrink_code) {shrink between words}
//! @d x_height==param(x_height_code) {one ex}
//! @d quad==param(quad_code) {one em}
//! @d extra_space==param(extra_space_code) {additional space at end of sentence}
//!
//! @<The em width for |cur_font|@>=quad(cur_font)
//!
//! @ @<The x-height for |cur_font|@>=x_height(cur_font)
//!
//! @ \TeX\ checks the information of a \.{TFM} file for validity as the
//! file is being read in, so that no further checks will be needed when
//! typesetting is going on. The somewhat tedious subroutine that does this
//! is called |read_font_info|. It has four parameters: the user font
//! identifier~|u|, the file name and area strings |nom| and |aire|, and the
//! ``at'' size~|s|. If |s|~is negative, it's the negative of a scale factor
//! to be applied to the design size; |s=-1000| is the normal case.
//! Otherwise |s| will be substituted for the design size; in this
//! case, |s| must be positive and less than $2048\rm\,pt$
//! (i.e., it must be less than $2^{27}$ when considered as an integer).
//!
//! The subroutine opens and closes a global file variable called |tfm_file|.
//! It returns the value of the internal font number that was just loaded.
//! If an error is detected, an error message is issued and no font
//! information is stored; |null_font| is returned in this case.
//!
//! @d bad_tfm=11 {label for |read_font_info|}
//! @d abort==goto bad_tfm {do this when the \.{TFM} data is wrong}
//!
//! @p function read_font_info(@!u:pointer;@!nom,@!aire:str_number;
//!   @!s:scaled):internal_font_number; {input a \.{TFM} file}
//! label done,bad_tfm,not_found;
//! var k:font_index; {index into |font_info|}
//! @!file_opened:boolean; {was |tfm_file| successfully opened?}
//! @!lf,@!lh,@!bc,@!ec,@!nw,@!nh,@!nd,@!ni,@!nl,@!nk,@!ne,@!np:halfword;
//!   {sizes of subfiles}
//! @!f:internal_font_number; {the new font's number}
//! @!g:internal_font_number; {the number to return}
//! @!a,@!b,@!c,@!d:eight_bits; {byte variables}
//! @!qw:four_quarters;@!sw:scaled; {accumulators}
//! @!bch_label:integer; {left boundary start location, or infinity}
//! @!bchar:0..256; {right boundary character, or 256}
//! @!z:scaled; {the design size or the ``at'' size}
//! @!alpha:integer;@!beta:1..16;
//!   {auxiliary quantities used in fixed-point multiplication}
//! begin g:=null_font;@/
//! @<Read and check the font data; |abort| if the \.{TFM} file is
//!   malformed; if there's no room for this font, say so and |goto
//!   done|; otherwise |incr(font_ptr)| and |goto done|@>;
//! bad_tfm: @<Report that the font won't be loaded@>;
//! done: if file_opened then b_close(tfm_file);
//! read_font_info:=g;
//! end;
//!
//! @ There are programs called \.{TFtoPL} and \.{PLtoTF} that convert
//! between the \.{TFM} format and a symbolic property-list format
//! that can be easily edited. These programs contain extensive
//! diagnostic information, so \TeX\ does not have to bother giving
//! precise details about why it rejects a particular \.{TFM} file.
//! @.TFtoPL@> @.PLtoTF@>
//!
//! @d start_font_error_message==print_err("Font "); sprint_cs(u);
//!   print_char("="); print_file_name(nom,aire,"");
//!   if s>=0 then
//!     begin print(" at "); print_scaled(s); print("pt");
//!     end
//!   else if s<>-1000 then
//!     begin print(" scaled "); print_int(-s);
//!     end
//!
//! @<Report that the font won't be loaded@>=
//! start_font_error_message;
//! @.Font x=xx not loadable...@>
//! if file_opened then print(" not loadable: Bad metric (TFM) file")
//! else print(" not loadable: Metric (TFM) file not found");
//! help5("I wasn't able to read the size data for this font,")@/
//! ("so I will ignore the font specification.")@/
//! ("[Wizards can fix TFM files using TFtoPL/PLtoTF.]")@/
//! ("You might try inserting a different font spec;")@/
//! ("e.g., type `I\font<same font id>=<substitute font name>'.");
//! error
//!
//! @ @<Read and check...@>=
//! @<Open |tfm_file| for input@>;
//! @<Read the {\.{TFM}} size fields@>;
//! @<Use size fields to allocate font information@>;
//! @<Read the {\.{TFM}} header@>;
//! @<Read character data@>;
//! @<Read box dimensions@>;
//! @<Read ligature/kern program@>;
//! @<Read extensible character recipes@>;
//! @<Read font parameters@>;
//! @<Make final adjustments and |goto done|@>
//!
//! @ @<Open |tfm_file| for input@>=
//! file_opened:=false;
//! if aire="" then pack_file_name(nom,TEX_font_area,".tfm")
//! else pack_file_name(nom,aire,".tfm");
//! if not b_open_in(tfm_file) then abort;
//! file_opened:=true
//!
//! @ Note: A malformed \.{TFM} file might be shorter than it claims to be;
//! thus |eof(tfm_file)| might be true when |read_font_info| refers to
//! |tfm_file^| or when it says |get(tfm_file)|. If such circumstances
//! cause system error messages, you will have to defeat them somehow,
//! for example by defining |fget| to be `\ignorespaces|begin get(tfm_file);|
//! |if eof(tfm_file) then abort; end|\unskip'.
//! @^system dependencies@>
//!
//! @d fget==get(tfm_file)
//! @d fbyte==tfm_file^
//! @d read_sixteen(#)==begin #:=fbyte;
//!   if #>127 then abort;
//!   fget; #:=#*@'400+fbyte;
//!   end
//! @d store_four_quarters(#)==begin fget; a:=fbyte; qw.b0:=qi(a);
//!   fget; b:=fbyte; qw.b1:=qi(b);
//!   fget; c:=fbyte; qw.b2:=qi(c);
//!   fget; d:=fbyte; qw.b3:=qi(d);
//!   #:=qw;
//!   end
//!
//! @ @<Read the {\.{TFM}} size fields@>=
//! begin read_sixteen(lf);
//! fget; read_sixteen(lh);
//! fget; read_sixteen(bc);
//! fget; read_sixteen(ec);
//! if (bc>ec+1)or(ec>255) then abort;
//! if bc>255 then {|bc=256| and |ec=255|}
//!   begin bc:=1; ec:=0;
//!   end;
//! fget; read_sixteen(nw);
//! fget; read_sixteen(nh);
//! fget; read_sixteen(nd);
//! fget; read_sixteen(ni);
//! fget; read_sixteen(nl);
//! fget; read_sixteen(nk);
//! fget; read_sixteen(ne);
//! fget; read_sixteen(np);
//! if lf<>6+lh+(ec-bc+1)+nw+nh+nd+ni+nl+nk+ne+np then abort;
//! if (nw=0)or(nh=0)or(nd=0)or(ni=0) then abort;
//! end
//!
//! @ The preliminary settings of the index-offset variables |char_base|,
//! |width_base|, |lig_kern_base|, |kern_base|, and |exten_base| will be
//! corrected later by subtracting |min_quarterword| from them; and we will
//! subtract 1 from |param_base| too. It's best to forget about such anomalies
//! until later.
//!
//! @<Use size fields to allocate font information@>=
//! lf:=lf-6-lh; {|lf| words should be loaded into |font_info|}
//! if np<7 then lf:=lf+7-np; {at least seven parameters will appear}
//! if (font_ptr=font_max)or(fmem_ptr+lf>font_mem_size) then
//!   @<Apologize for not loading the font, |goto done|@>;
//! f:=font_ptr+1;
//! char_base[f]:=fmem_ptr-bc;
//! width_base[f]:=char_base[f]+ec+1;
//! height_base[f]:=width_base[f]+nw;
//! depth_base[f]:=height_base[f]+nh;
//! italic_base[f]:=depth_base[f]+nd;
//! lig_kern_base[f]:=italic_base[f]+ni;
//! kern_base[f]:=lig_kern_base[f]+nl-kern_base_offset;
//! exten_base[f]:=kern_base[f]+kern_base_offset+nk;
//! param_base[f]:=exten_base[f]+ne
//!
//! @ @<Apologize for not loading...@>=
//! begin start_font_error_message;
//! print(" not loaded: Not enough room left");
//! @.Font x=xx not loaded...@>
//! help4("I'm afraid I won't be able to make use of this font,")@/
//! ("because my memory for character-size data is too small.")@/
//! ("If you're really stuck, ask a wizard to enlarge me.")@/
//! ("Or maybe try `I\font<same font id>=<name of loaded font>'.");
//! error; goto done;
//! end
//!
//! @ Only the first two words of the header are needed by \TeX82.
//!
//! @<Read the {\.{TFM}} header@>=
//! begin if lh<2 then abort;
//! store_four_quarters(font_check[f]);
//! fget; read_sixteen(z); {this rejects a negative design size}
//! fget; z:=z*@'400+fbyte; fget; z:=(z*@'20)+(fbyte div@'20);
//! if z<unity then abort;
//! while lh>2 do
//!   begin fget;fget;fget;fget;decr(lh); {ignore the rest of the header}
//!   end;
//! font_dsize[f]:=z;
//! if s<>-1000 then
//!   if s>=0 then z:=s
//!   else z:=xn_over_d(z,-s,1000);
//! font_size[f]:=z;
//! end
//!
//! @ @<Read character data@>=
//! for k:=fmem_ptr to width_base[f]-1 do
//!   begin store_four_quarters(font_info[k].qqqq);
//!   if (a>=nw)or(b div @'20>=nh)or(b mod @'20>=nd)or
//!     (c div 4>=ni) then abort;
//!   case c mod 4 of
//!   lig_tag: if d>=nl then abort;
//!   ext_tag: if d>=ne then abort;
//!   list_tag: @<Check for charlist cycle@>;
//!   othercases do_nothing {|no_tag|}
//!   endcases;
//!   end
//!
//! @ We want to make sure that there is no cycle of characters linked together
//! by |list_tag| entries, since such a cycle would get \TeX\ into an endless
//! loop. If such a cycle exists, the routine here detects it when processing
//! the largest character code in the cycle.
//!
//! @d check_byte_range(#)==begin if (#<bc)or(#>ec) then abort@+end
//! @d current_character_being_worked_on==k+bc-fmem_ptr
//!
//! @<Check for charlist cycle@>=
//! begin check_byte_range(d);
//! while d<current_character_being_worked_on do
//!   begin qw:=char_info(f)(d);
//!   {N.B.: not |qi(d)|, since |char_base[f]| hasn't been adjusted yet}
//!   if char_tag(qw)<>list_tag then goto not_found;
//!   d:=qo(rem_byte(qw)); {next character on the list}
//!   end;
//! if d=current_character_being_worked_on then abort; {yes, there's a cycle}
//! not_found:end
//!
//! @ A |fix_word| whose four bytes are $(a,b,c,d)$ from left to right represents
//! the number
//! $$x=\left\{\vcenter{\halign{$#$,\hfil\qquad&if $#$\hfil\cr
//! b\cdot2^{-4}+c\cdot2^{-12}+d\cdot2^{-20}&a=0;\cr
//! -16+b\cdot2^{-4}+c\cdot2^{-12}+d\cdot2^{-20}&a=255.\cr}}\right.$$
//! (No other choices of |a| are allowed, since the magnitude of a number in
//! design-size units must be less than 16.)  We want to multiply this
//! quantity by the integer~|z|, which is known to be less than $2^{27}$.
//! If $|z|<2^{23}$, the individual multiplications $b\cdot z$,
//! $c\cdot z$, $d\cdot z$ cannot overflow; otherwise we will divide |z| by 2,
//! 4, 8, or 16, to obtain a multiplier less than $2^{23}$, and we can
//! compensate for this later. If |z| has thereby been replaced by
//! $|z|^\prime=|z|/2^e$, let $\beta=2^{4-e}$; we shall compute
//! $$\lfloor(b+c\cdot2^{-8}+d\cdot2^{-16})\,z^\prime/\beta\rfloor$$
//! if $a=0$, or the same quantity minus $\alpha=2^{4+e}z^\prime$ if $a=255$.
//! This calculation must be done exactly, in order to guarantee portability
//! of \TeX\ between computers.
//!
//! @d store_scaled(#)==begin fget; a:=fbyte; fget; b:=fbyte;
//!   fget; c:=fbyte; fget; d:=fbyte;@/
//!   sw:=(((((d*z)div@'400)+(c*z))div@'400)+(b*z))div beta;
//!   if a=0 then #:=sw@+else if a=255 then #:=sw-alpha@+else abort;
//!   end
//!
//! @<Read box dimensions@>=
//! begin @<Replace |z| by $|z|^\prime$ and compute $\alpha,\beta$@>;
//! for k:=width_base[f] to lig_kern_base[f]-1 do
//!   store_scaled(font_info[k].sc);
//! if font_info[width_base[f]].sc<>0 then abort; {\\{width}[0] must be zero}
//! if font_info[height_base[f]].sc<>0 then abort; {\\{height}[0] must be zero}
//! if font_info[depth_base[f]].sc<>0 then abort; {\\{depth}[0] must be zero}
//! if font_info[italic_base[f]].sc<>0 then abort; {\\{italic}[0] must be zero}
//! end
//!
//! @ @<Replace |z|...@>=
//! begin alpha:=16;
//! while z>=@'40000000 do
//!   begin z:=z div 2; alpha:=alpha+alpha;
//!   end;
//! beta:=256 div alpha; alpha:=alpha*z;
//! end
//!
//! @ @d check_existence(#)==@t@>@;@/
//!   begin check_byte_range(#);
//!   qw:=char_info(f)(#); {N.B.: not |qi(#)|}
//!   if not char_exists(qw) then abort;
//!   end
//!
//! @<Read ligature/kern program@>=
//! bch_label:=@'77777; bchar:=256;
//! if nl>0 then
//!   begin for k:=lig_kern_base[f] to kern_base[f]+kern_base_offset-1 do
//!     begin store_four_quarters(font_info[k].qqqq);
//!     if a>128 then
//!       begin if 256*c+d>=nl then abort;
//!       if a=255 then if k=lig_kern_base[f] then bchar:=b;
//!       end
//!     else begin if b<>bchar then check_existence(b);
//!       if c<128 then check_existence(d) {check ligature}
//!       else if 256*(c-128)+d>=nk then abort; {check kern}
//!       if a<128 then if k-lig_kern_base[f]+a+1>=nl then abort;
//!       end;
//!     end;
//!   if a=255 then bch_label:=256*c+d;
//!   end;
//! for k:=kern_base[f]+kern_base_offset to exten_base[f]-1 do
//!   store_scaled(font_info[k].sc);
//!
//! @ @<Read extensible character recipes@>=
//! for k:=exten_base[f] to param_base[f]-1 do
//!   begin store_four_quarters(font_info[k].qqqq);
//!   if a<>0 then check_existence(a);
//!   if b<>0 then check_existence(b);
//!   if c<>0 then check_existence(c);
//!   check_existence(d);
//!   end
//!
//! @ We check to see that the \.{TFM} file doesn't end prematurely; but
//! no error message is given for files having more than |lf| words.
//!
//! @<Read font parameters@>=
//! begin for k:=1 to np do
//!   if k=1 then {the |slant| parameter is a pure number}
//!     begin fget; sw:=fbyte; if sw>127 then sw:=sw-256;
//!     fget; sw:=sw*@'400+fbyte; fget; sw:=sw*@'400+fbyte;
//!     fget; font_info[param_base[f]].sc:=
//!       (sw*@'20)+(fbyte div@'20);
//!     end
//!   else store_scaled(font_info[param_base[f]+k-1].sc);
//! if eof(tfm_file) then abort;
//! for k:=np+1 to 7 do font_info[param_base[f]+k-1].sc:=0;
//! end
//!
//! @ Now to wrap it up, we have checked all the necessary things about the \.{TFM}
//! file, and all we need to do is put the finishing touches on the data for
//! the new font.
//!
//! @d adjust(#)==#[f]:=qo(#[f])
//!   {correct for the excess |min_quarterword| that was added}
//!
//! @<Make final adjustments...@>=
//! if np>=7 then font_params[f]:=np@+else font_params[f]:=7;
//! hyphen_char[f]:=default_hyphen_char; skew_char[f]:=default_skew_char;
//! if bch_label<nl then bchar_label[f]:=bch_label+lig_kern_base[f]
//! else bchar_label[f]:=non_address;
//! font_bchar[f]:=qi(bchar);
//! font_false_bchar[f]:=qi(bchar);
//! if bchar<=ec then if bchar>=bc then
//!   begin qw:=char_info(f)(bchar); {N.B.: not |qi(bchar)|}
//!   if char_exists(qw) then font_false_bchar[f]:=non_char;
//!   end;
//! font_name[f]:=nom;
//! font_area[f]:=aire;
//! font_bc[f]:=bc; font_ec[f]:=ec; font_glue[f]:=null;
//! adjust(char_base); adjust(width_base); adjust(lig_kern_base);
//! adjust(kern_base); adjust(exten_base);
//! decr(param_base[f]);
//! fmem_ptr:=fmem_ptr+lf; font_ptr:=f; g:=f; goto done
//!
//! @ Before we forget about the format of these tables, let's deal with two
//! of \TeX's basic scanning routines related to font information.
//!
//! @<Declare procedures that scan font-related stuff@>=
//! procedure scan_font_ident;
//! var f:internal_font_number;
//! @!m:halfword;
//! begin @<Get the next non-blank non-call...@>;
//! if cur_cmd=def_font then f:=cur_font
//! else if cur_cmd=set_font then f:=cur_chr
//! else if cur_cmd=def_family then
//!   begin m:=cur_chr; scan_four_bit_int; f:=equiv(m+cur_val);
//!   end
//! else  begin print_err("Missing font identifier");
//! @.Missing font identifier@>
//!   help2("I was looking for a control sequence whose")@/
//!   ("current meaning has been defined by \font.");
//!   back_error; f:=null_font;
//!   end;
//! cur_val:=f;
//! end;
//!
//! @ The following routine is used to implement `\.{\\fontdimen} |n| |f|'.
//! The boolean parameter |writing| is set |true| if the calling program
//! intends to change the parameter value.
//!
//! @<Declare procedures that scan font-related stuff@>=
//! procedure find_font_dimen(@!writing:boolean);
//!   {sets |cur_val| to |font_info| location}
//! var f:internal_font_number;
//! @!n:integer; {the parameter number}
//! begin scan_int; n:=cur_val; scan_font_ident; f:=cur_val;
//! if n<=0 then cur_val:=fmem_ptr
//! else  begin if writing and(n<=space_shrink_code)and@|
//!     (n>=space_code)and(font_glue[f]<>null) then
//!     begin delete_glue_ref(font_glue[f]);
//!     font_glue[f]:=null;
//!     end;
//!   if n>font_params[f] then
//!     if f<font_ptr then cur_val:=fmem_ptr
//!     else @<Increase the number of parameters in the last font@>
//!   else cur_val:=n+param_base[f];
//!   end;
//! @<Issue an error message if |cur_val=fmem_ptr|@>;
//! end;
//!
//! @ @<Issue an error message if |cur_val=fmem_ptr|@>=
//! if cur_val=fmem_ptr then
//!   begin print_err("Font "); print_esc(font_id_text(f));
//!   print(" has only "); print_int(font_params[f]);
//!   print(" fontdimen parameters");
//! @.Font x has only...@>
//!   help2("To increase the number of font parameters, you must")@/
//!     ("use \fontdimen immediately after the \font is loaded.");
//!   error;
//!   end
//!
//! @ @<Increase the number of parameters...@>=
//! begin repeat if fmem_ptr=font_mem_size then
//!   overflow("font memory",font_mem_size);
//! @:TeX capacity exceeded font memory}{\quad font memory@>
//! font_info[fmem_ptr].sc:=0; incr(fmem_ptr); incr(font_params[f]);
//! until n=font_params[f];
//! cur_val:=fmem_ptr-1; {this equals |param_base[f]+font_params[f]|}
//! end
//!
