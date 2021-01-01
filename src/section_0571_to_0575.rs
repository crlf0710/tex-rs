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
