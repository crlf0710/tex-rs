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
