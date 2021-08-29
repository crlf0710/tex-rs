//! @ Here is a list of all the commands that may appear in a \.{DVI} file. Each
//! command is specified by its symbolic name (e.g., |bop|), its opcode byte
//! (e.g., 139), and its parameters (if any). The parameters are followed
//! by a bracketed number telling how many bytes they occupy; for example,
//! `|p[4]|' means that parameter |p| is four bytes long.
//!
//! \yskip\hang|set_char_0| 0. Typeset character number~0 from font~|f|
//! such that the reference point of the character is at |(h,v)|. Then
//! increase |h| by the width of that character. Note that a character may
//! have zero or negative width, so one cannot be sure that |h| will advance
//! after this command; but |h| usually does increase.
//!
//! \yskip\hang\\{set\_char\_1} through \\{set\_char\_127} (opcodes 1 to 127).
//! Do the operations of |set_char_0|; but use the character whose number
//! matches the opcode, instead of character~0.
//!
//! \yskip\hang|set1| 128 |c[1]|. Same as |set_char_0|, except that character
//! number~|c| is typeset. \TeX82 uses this command for characters in the
//! range |128<=c<256|.
//!
//! \yskip\hang|@!set2| 129 |c[2]|. Same as |set1|, except that |c|~is two
//! bytes long, so it is in the range |0<=c<65536|. \TeX82 never uses this
//! command, but it should come in handy for extensions of \TeX\ that deal
//! with oriental languages.
//! @^oriental characters@>@^Chinese characters@>@^Japanese characters@>
//!
//! \yskip\hang|@!set3| 130 |c[3]|. Same as |set1|, except that |c|~is three
//! bytes long, so it can be as large as $2^{24}-1$. Not even the Chinese
//! language has this many characters, but this command might prove useful
//! in some yet unforeseen extension.
//!
//! \yskip\hang|@!set4| 131 |c[4]|. Same as |set1|, except that |c|~is four
//! bytes long. Imagine that.
//!
//! \yskip\hang|set_rule| 132 |a[4]| |b[4]|. Typeset a solid black rectangle
//! of height~|a| and width~|b|, with its bottom left corner at |(h,v)|. Then
//! set |h:=h+b|. If either |a<=0| or |b<=0|, nothing should be typeset. Note
//! that if |b<0|, the value of |h| will decrease even though nothing else happens.
//! See below for details about how to typeset rules so that consistency with
//! \MF\ is guaranteed.
//!
//! \yskip\hang|@!put1| 133 |c[1]|. Typeset character number~|c| from font~|f|
//! such that the reference point of the character is at |(h,v)|. (The `put'
//! commands are exactly like the `set' commands, except that they simply put out a
//! character or a rule without moving the reference point afterwards.)
//!
//! \yskip\hang|@!put2| 134 |c[2]|. Same as |set2|, except that |h| is not changed.
//!
//! \yskip\hang|@!put3| 135 |c[3]|. Same as |set3|, except that |h| is not changed.
//!
//! \yskip\hang|@!put4| 136 |c[4]|. Same as |set4|, except that |h| is not changed.
//!
//! \yskip\hang|put_rule| 137 |a[4]| |b[4]|. Same as |set_rule|, except that
//! |h| is not changed.
//!
//! \yskip\hang|nop| 138. No operation, do nothing. Any number of |nop|'s
//! may occur between \.{DVI} commands, but a |nop| cannot be inserted between
//! a command and its parameters or between two parameters.
//!
//! \yskip\hang|bop| 139 $c_0[4]$ $c_1[4]$ $\ldots$ $c_9[4]$ $p[4]$. Beginning
//! of a page: Set |(h,v,w,x,y,z):=(0,0,0,0,0,0)| and set the stack empty. Set
//! the current font |f| to an undefined value.  The ten $c_i$ parameters hold
//! the values of \.{\\count0} $\ldots$ \.{\\count9} in \TeX\ at the time
//! \.{\\shipout} was invoked for this page; they can be used to identify
//! pages, if a user wants to print only part of a \.{DVI} file. The parameter
//! |p| points to the previous |bop| in the file; the first
//! |bop| has $p=-1$.
//!
//! \yskip\hang|eop| 140.  End of page: Print what you have read since the
//! previous |bop|. At this point the stack should be empty. (The \.{DVI}-reading
//! programs that drive most output devices will have kept a buffer of the
//! material that appears on the page that has just ended. This material is
//! largely, but not entirely, in order by |v| coordinate and (for fixed |v|) by
//! |h|~coordinate; so it usually needs to be sorted into some order that is
//! appropriate for the device in question.)
//!
//! \yskip\hang|push| 141. Push the current values of |(h,v,w,x,y,z)| onto the
//! top of the stack; do not change any of these values. Note that |f| is
//! not pushed.
//!
//! \yskip\hang|pop| 142. Pop the top six values off of the stack and assign
//! them respectively to |(h,v,w,x,y,z)|. The number of pops should never
//! exceed the number of pushes, since it would be highly embarrassing if the
//! stack were empty at the time of a |pop| command.
//!
//! \yskip\hang|right1| 143 |b[1]|. Set |h:=h+b|, i.e., move right |b| units.
//! The parameter is a signed number in two's complement notation, |-128<=b<128|;
//! if |b<0|, the reference point moves left.
//!
//! \yskip\hang|@!right2| 144 |b[2]|. Same as |right1|, except that |b| is a
//! two-byte quantity in the range |-32768<=b<32768|.
//!
//! \yskip\hang|@!right3| 145 |b[3]|. Same as |right1|, except that |b| is a
//! three-byte quantity in the range |@t$-2^{23}$@><=b<@t$2^{23}$@>|.
//!
//! \yskip\hang|@!right4| 146 |b[4]|. Same as |right1|, except that |b| is a
//! four-byte quantity in the range |@t$-2^{31}$@><=b<@t$2^{31}$@>|.
//!
//! \yskip\hang|w0| 147. Set |h:=h+w|; i.e., move right |w| units. With luck,
//! this parameterless command will usually suffice, because the same kind of motion
//! will occur several times in succession; the following commands explain how
//! |w| gets particular values.
//!
//! \yskip\hang|w1| 148 |b[1]|. Set |w:=b| and |h:=h+b|. The value of |b| is a
//! signed quantity in two's complement notation, |-128<=b<128|. This command
//! changes the current |w|~spacing and moves right by |b|.
//!
//! \yskip\hang|@!w2| 149 |b[2]|. Same as |w1|, but |b| is two bytes long,
//! |-32768<=b<32768|.
//!
//! \yskip\hang|@!w3| 150 |b[3]|. Same as |w1|, but |b| is three bytes long,
//! |@t$-2^{23}$@><=b<@t$2^{23}$@>|.
//!
//! \yskip\hang|@!w4| 151 |b[4]|. Same as |w1|, but |b| is four bytes long,
//! |@t$-2^{31}$@><=b<@t$2^{31}$@>|.
//!
//! \yskip\hang|x0| 152. Set |h:=h+x|; i.e., move right |x| units. The `|x|'
//! commands are like the `|w|' commands except that they involve |x| instead
//! of |w|.
//!
//! \yskip\hang|x1| 153 |b[1]|. Set |x:=b| and |h:=h+b|. The value of |b| is a
//! signed quantity in two's complement notation, |-128<=b<128|. This command
//! changes the current |x|~spacing and moves right by |b|.
//!
//! \yskip\hang|@!x2| 154 |b[2]|. Same as |x1|, but |b| is two bytes long,
//! |-32768<=b<32768|.
//!
//! \yskip\hang|@!x3| 155 |b[3]|. Same as |x1|, but |b| is three bytes long,
//! |@t$-2^{23}$@><=b<@t$2^{23}$@>|.
//!
//! \yskip\hang|@!x4| 156 |b[4]|. Same as |x1|, but |b| is four bytes long,
//! |@t$-2^{31}$@><=b<@t$2^{31}$@>|.
//!
//! \yskip\hang|down1| 157 |a[1]|. Set |v:=v+a|, i.e., move down |a| units.
//! The parameter is a signed number in two's complement notation, |-128<=a<128|;
//! if |a<0|, the reference point moves up.
//!
//! \yskip\hang|@!down2| 158 |a[2]|. Same as |down1|, except that |a| is a
//! two-byte quantity in the range |-32768<=a<32768|.
//!
//! \yskip\hang|@!down3| 159 |a[3]|. Same as |down1|, except that |a| is a
//! three-byte quantity in the range |@t$-2^{23}$@><=a<@t$2^{23}$@>|.
//!
//! \yskip\hang|@!down4| 160 |a[4]|. Same as |down1|, except that |a| is a
//! four-byte quantity in the range |@t$-2^{31}$@><=a<@t$2^{31}$@>|.
//!
//! \yskip\hang|y0| 161. Set |v:=v+y|; i.e., move down |y| units. With luck,
//! this parameterless command will usually suffice, because the same kind of motion
//! will occur several times in succession; the following commands explain how
//! |y| gets particular values.
//!
//! \yskip\hang|y1| 162 |a[1]|. Set |y:=a| and |v:=v+a|. The value of |a| is a
//! signed quantity in two's complement notation, |-128<=a<128|. This command
//! changes the current |y|~spacing and moves down by |a|.
//!
//! \yskip\hang|@!y2| 163 |a[2]|. Same as |y1|, but |a| is two bytes long,
//! |-32768<=a<32768|.
//!
//! \yskip\hang|@!y3| 164 |a[3]|. Same as |y1|, but |a| is three bytes long,
//! |@t$-2^{23}$@><=a<@t$2^{23}$@>|.
//!
//! \yskip\hang|@!y4| 165 |a[4]|. Same as |y1|, but |a| is four bytes long,
//! |@t$-2^{31}$@><=a<@t$2^{31}$@>|.
//!
//! \yskip\hang|z0| 166. Set |v:=v+z|; i.e., move down |z| units. The `|z|' commands
//! are like the `|y|' commands except that they involve |z| instead of |y|.
//!
//! \yskip\hang|z1| 167 |a[1]|. Set |z:=a| and |v:=v+a|. The value of |a| is a
//! signed quantity in two's complement notation, |-128<=a<128|. This command
//! changes the current |z|~spacing and moves down by |a|.
//!
//! \yskip\hang|@!z2| 168 |a[2]|. Same as |z1|, but |a| is two bytes long,
//! |-32768<=a<32768|.
//!
//! \yskip\hang|@!z3| 169 |a[3]|. Same as |z1|, but |a| is three bytes long,
//! |@t$-2^{23}$@><=a<@t$2^{23}$@>|.
//!
//! \yskip\hang|@!z4| 170 |a[4]|. Same as |z1|, but |a| is four bytes long,
//! |@t$-2^{31}$@><=a<@t$2^{31}$@>|.
//!
//! \yskip\hang|fnt_num_0| 171. Set |f:=0|. Font 0 must previously have been
//! defined by a \\{fnt\_def} instruction, as explained below.
//!
//! \yskip\hang\\{fnt\_num\_1} through \\{fnt\_num\_63} (opcodes 172 to 234). Set
//! |f:=1|, \dots, \hbox{|f:=63|}, respectively.
//!
//! \yskip\hang|fnt1| 235 |k[1]|. Set |f:=k|. \TeX82 uses this command for font
//! numbers in the range |64<=k<256|.
//!
//! \yskip\hang|@!fnt2| 236 |k[2]|. Same as |fnt1|, except that |k|~is two
//! bytes long, so it is in the range |0<=k<65536|. \TeX82 never generates this
//! command, but large font numbers may prove useful for specifications of
//! color or texture, or they may be used for special fonts that have fixed
//! numbers in some external coding scheme.
//!
//! \yskip\hang|@!fnt3| 237 |k[3]|. Same as |fnt1|, except that |k|~is three
//! bytes long, so it can be as large as $2^{24}-1$.
//!
//! \yskip\hang|@!fnt4| 238 |k[4]|. Same as |fnt1|, except that |k|~is four
//! bytes long; this is for the really big font numbers (and for the negative ones).
//!
//! \yskip\hang|xxx1| 239 |k[1]| |x[k]|. This command is undefined in
//! general; it functions as a $(k+2)$-byte |nop| unless special \.{DVI}-reading
//! programs are being used. \TeX82 generates |xxx1| when a short enough
//! \.{\\special} appears, setting |k| to the number of bytes being sent. It
//! is recommended that |x| be a string having the form of a keyword followed
//! by possible parameters relevant to that keyword.
//!
//! \yskip\hang|@!xxx2| 240 |k[2]| |x[k]|. Like |xxx1|, but |0<=k<65536|.
//!
//! \yskip\hang|@!xxx3| 241 |k[3]| |x[k]|. Like |xxx1|, but |0<=k<@t$2^{24}$@>|.
//!
//! \yskip\hang|xxx4| 242 |k[4]| |x[k]|. Like |xxx1|, but |k| can be ridiculously
//! large. \TeX82 uses |xxx4| when sending a string of length 256 or more.
//!
//! \yskip\hang|fnt_def1| 243 |k[1]| |c[4]| |s[4]| |d[4]| |a[1]| |l[1]| |n[a+l]|.
//! Define font |k|, where |0<=k<256|; font definitions will be explained shortly.
//!
//! \yskip\hang|@!fnt_def2| 244 |k[2]| |c[4]| |s[4]| |d[4]| |a[1]| |l[1]| |n[a+l]|.
//! Define font |k|, where |0<=k<65536|.
//!
//! \yskip\hang|@!fnt_def3| 245 |k[3]| |c[4]| |s[4]| |d[4]| |a[1]| |l[1]| |n[a+l]|.
//! Define font |k|, where |0<=k<@t$2^{24}$@>|.
//!
//! \yskip\hang|@!fnt_def4| 246 |k[4]| |c[4]| |s[4]| |d[4]| |a[1]| |l[1]| |n[a+l]|.
//! Define font |k|, where |@t$-2^{31}$@><=k<@t$2^{31}$@>|.
//!
//! \yskip\hang|pre| 247 |i[1]| |num[4]| |den[4]| |mag[4]| |k[1]| |x[k]|.
//! Beginning of the preamble; this must come at the very beginning of the
//! file. Parameters |i|, |num|, |den|, |mag|, |k|, and |x| are explained below.
//!
//! \yskip\hang|post| 248. Beginning of the postamble, see below.
//!
//! \yskip\hang|post_post| 249. Ending of the postamble, see below.
//!
//! \yskip\noindent Commands 250--255 are undefined at the present time.
//!
