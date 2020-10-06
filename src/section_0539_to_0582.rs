//! @* \[30] Font metric data.
//! \TeX\ gets its knowledge about fonts from font metric files, also called
//! \.{TFM} files; the `\.T' in `\.{TFM}' stands for \TeX,
//! but other programs know about them too.
//! @:TFM files}{\.{TFM} files@>
//! @^font metric files@>
//!
//! The information in a \.{TFM} file appears in a sequence of 8-bit bytes.
//! Since the number of bytes is always a multiple of 4, we could
//! also regard the file as a sequence of 32-bit words, but \TeX\ uses the
//! byte interpretation. The format of \.{TFM} files was designed by
//! Lyle Ramshaw in 1980. The intent is to convey a lot of different kinds
//! @^Ramshaw, Lyle Harold@>
//! of information in a compact but useful form.
//!
//! @<Glob...@>=
//! @!tfm_file:byte_file;
//!
//! @ The first 24 bytes (6 words) of a \.{TFM} file contain twelve 16-bit
//! integers that give the lengths of the various subsequent portions
//! of the file. These twelve integers are, in order:
//! $$\vbox{\halign{\hfil#&$\null=\null$#\hfil\cr
//! |lf|&length of the entire file, in words;\cr
//! |lh|&length of the header data, in words;\cr
//! |bc|&smallest character code in the font;\cr
//! |ec|&largest character code in the font;\cr
//! |nw|&number of words in the width table;\cr
//! |nh|&number of words in the height table;\cr
//! |nd|&number of words in the depth table;\cr
//! |ni|&number of words in the italic correction table;\cr
//! |nl|&number of words in the lig/kern table;\cr
//! |nk|&number of words in the kern table;\cr
//! |ne|&number of words in the extensible character table;\cr
//! |np|&number of font parameter words.\cr}}$$
//! They are all nonnegative and less than $2^{15}$. We must have |bc-1<=ec<=255|,
//! and
//! $$\hbox{|lf=6+lh+(ec-bc+1)+nw+nh+nd+ni+nl+nk+ne+np|.}$$
//! Note that a font may contain as many as 256 characters (if |bc=0| and |ec=255|),
//! and as few as 0 characters (if |bc=ec+1|).
//!
//! Incidentally, when two or more 8-bit bytes are combined to form an integer of
//! 16 or more bits, the most significant bytes appear first in the file.
//! This is called BigEndian order.
//! @!@^BigEndian order@>
//!
//! @ The rest of the \.{TFM} file may be regarded as a sequence of ten data
//! arrays having the informal specification
//! $$\def\arr$[#1]#2${\&{array} $[#1]$ \&{of} #2}
//! \vbox{\halign{\hfil\\{#}&$\,:\,$\arr#\hfil\cr
//! header&|[0..lh-1]@t\\{stuff}@>|\cr
//! char\_info&|[bc..ec]char_info_word|\cr
//! width&|[0..nw-1]fix_word|\cr
//! height&|[0..nh-1]fix_word|\cr
//! depth&|[0..nd-1]fix_word|\cr
//! italic&|[0..ni-1]fix_word|\cr
//! lig\_kern&|[0..nl-1]lig_kern_command|\cr
//! kern&|[0..nk-1]fix_word|\cr
//! exten&|[0..ne-1]extensible_recipe|\cr
//! param&|[1..np]fix_word|\cr}}$$
//! The most important data type used here is a |@!fix_word|, which is
//! a 32-bit representation of a binary fraction. A |fix_word| is a signed
//! quantity, with the two's complement of the entire word used to represent
//! negation. Of the 32 bits in a |fix_word|, exactly 12 are to the left of the
//! binary point; thus, the largest |fix_word| value is $2048-2^{-20}$, and
//! the smallest is $-2048$. We will see below, however, that all but two of
//! the |fix_word| values must lie between $-16$ and $+16$.
//!
//! @ The first data array is a block of header information, which contains
//! general facts about the font. The header must contain at least two words,
//! |header[0]| and |header[1]|, whose meaning is explained below.
//! Additional header information of use to other software routines might
//! also be included, but \TeX82 does not need to know about such details.
//! For example, 16 more words of header information are in use at the Xerox
//! Palo Alto Research Center; the first ten specify the character coding
//! scheme used (e.g., `\.{XEROX text}' or `\.{TeX math symbols}'), the next five
//! give the font identifier (e.g., `\.{HELVETICA}' or `\.{CMSY}'), and the
//! last gives the ``face byte.'' The program that converts \.{DVI} files
//! to Xerox printing format gets this information by looking at the \.{TFM}
//! file, which it needs to read anyway because of other information that
//! is not explicitly repeated in \.{DVI}~format.
//!
//! \yskip\hang|header[0]| is a 32-bit check sum that \TeX\ will copy into
//! the \.{DVI} output file. Later on when the \.{DVI} file is printed,
//! possibly on another computer, the actual font that gets used is supposed
//! to have a check sum that agrees with the one in the \.{TFM} file used by
//! \TeX. In this way, users will be warned about potential incompatibilities.
//! (However, if the check sum is zero in either the font file or the \.{TFM}
//! file, no check is made.)  The actual relation between this check sum and
//! the rest of the \.{TFM} file is not important; the check sum is simply an
//! identification number with the property that incompatible fonts almost
//! always have distinct check sums.
//! @^check sum@>
//!
//! \yskip\hang|header[1]| is a |fix_word| containing the design size of
//! the font, in units of \TeX\ points. This number must be at least 1.0; it is
//! fairly arbitrary, but usually the design size is 10.0 for a ``10 point''
//! font, i.e., a font that was designed to look best at a 10-point size,
//! whatever that really means. When a \TeX\ user asks for a font
//! `\.{at} $\delta$ \.{pt}', the effect is to override the design size
//! and replace it by $\delta$, and to multiply the $x$ and~$y$ coordinates
//! of the points in the font image by a factor of $\delta$ divided by the
//! design size.  {\sl All other dimensions in the\/ \.{TFM} file are
//! |fix_word|\kern-1pt\ numbers in design-size units}, with the exception of
//! |param[1]| (which denotes the slant ratio). Thus, for example, the value
//! of |param[6]|, which defines the \.{em} unit, is often the |fix_word| value
//! $2^{20}=1.0$, since many fonts have a design size equal to one em.
//! The other dimensions must be less than 16 design-size units in absolute
//! value; thus, |header[1]| and |param[1]| are the only |fix_word|
//! entries in the whole \.{TFM} file whose first byte might be something
//! besides 0 or 255.
//!
//! @ Next comes the |char_info| array, which contains one |@!char_info_word|
//! per character. Each word in this part of the file contains six fields
//! packed into four bytes as follows.
//!
//! \yskip\hang first byte: |@!width_index| (8 bits)\par
//! \hang second byte: |@!height_index| (4 bits) times 16, plus |@!depth_index|
//!   (4~bits)\par
//! \hang third byte: |@!italic_index| (6 bits) times 4, plus |@!tag|
//!   (2~bits)\par
//! \hang fourth byte: |@!remainder| (8 bits)\par
//! \yskip\noindent
//! The actual width of a character is \\{width}|[width_index]|, in design-size
//! units; this is a device for compressing information, since many characters
//! have the same width. Since it is quite common for many characters
//! to have the same height, depth, or italic correction, the \.{TFM} format
//! imposes a limit of 16 different heights, 16 different depths, and
//! 64 different italic corrections.
//!
//! @!@^italic correction@>
//! The italic correction of a character has two different uses.
//! (a)~In ordinary text, the italic correction is added to the width only if
//! the \TeX\ user specifies `\.{\\/}' after the character.
//! (b)~In math formulas, the italic correction is always added to the width,
//! except with respect to the positioning of subscripts.
//!
//! Incidentally, the relation $\\{width}[0]=\\{height}[0]=\\{depth}[0]=
//! \\{italic}[0]=0$ should always hold, so that an index of zero implies a
//! value of zero.  The |width_index| should never be zero unless the
//! character does not exist in the font, since a character is valid if and
//! only if it lies between |bc| and |ec| and has a nonzero |width_index|.
//!
//! @ The |tag| field in a |char_info_word| has four values that explain how to
//! interpret the |remainder| field.
//!
//! \yskip\hangg|tag=0| (|no_tag|) means that |remainder| is unused.\par
//! \hangg|tag=1| (|lig_tag|) means that this character has a ligature/kerning
//! program starting at position |remainder| in the |lig_kern| array.\par
//! \hangg|tag=2| (|list_tag|) means that this character is part of a chain of
//! characters of ascending sizes, and not the largest in the chain.  The
//! |remainder| field gives the character code of the next larger character.\par
//! \hangg|tag=3| (|ext_tag|) means that this character code represents an
//! extensible character, i.e., a character that is built up of smaller pieces
//! so that it can be made arbitrarily large. The pieces are specified in
//! |@!exten[remainder]|.\par
//! \yskip\noindent
//! Characters with |tag=2| and |tag=3| are treated as characters with |tag=0|
//! unless they are used in special circumstances in math formulas. For example,
//! the \.{\\sum} operation looks for a |list_tag|, and the \.{\\left}
//! operation looks for both |list_tag| and |ext_tag|.
//!
//! @d no_tag=0 {vanilla character}
//! @d lig_tag=1 {character has a ligature/kerning program}
//! @d list_tag=2 {character has a successor in a charlist}
//! @d ext_tag=3 {character is extensible}
//!
//! @ The |lig_kern| array contains instructions in a simple programming language
//! that explains what to do for special letter pairs. Each word in this array is a
//! |@!lig_kern_command| of four bytes.
//!
//! \yskip\hang first byte: |skip_byte|, indicates that this is the final program
//!   step if the byte is 128 or more, otherwise the next step is obtained by
//!   skipping this number of intervening steps.\par
//! \hang second byte: |next_char|, ``if |next_char| follows the current character,
//!   then perform the operation and stop, otherwise continue.''\par
//! \hang third byte: |op_byte|, indicates a ligature step if less than~128,
//!   a kern step otherwise.\par
//! \hang fourth byte: |remainder|.\par
//! \yskip\noindent
//! In a kern step, an
//! additional space equal to |kern[256*(op_byte-128)+remainder]| is inserted
//! between the current character and |next_char|. This amount is
//! often negative, so that the characters are brought closer together
//! by kerning; but it might be positive.
//!
//! There are eight kinds of ligature steps, having |op_byte| codes $4a+2b+c$ where
//! $0\le a\le b+c$ and $0\le b,c\le1$. The character whose code is
//! |remainder| is inserted between the current character and |next_char|;
//! then the current character is deleted if $b=0$, and |next_char| is
//! deleted if $c=0$; then we pass over $a$~characters to reach the next
//! current character (which may have a ligature/kerning program of its own).
//!
//! If the very first instruction of the |lig_kern| array has |skip_byte=255|,
//! the |next_char| byte is the so-called right boundary character of this font;
//! the value of |next_char| need not lie between |bc| and~|ec|.
//! If the very last instruction of the |lig_kern| array has |skip_byte=255|,
//! there is a special ligature/kerning program for a left boundary character,
//! beginning at location |256*op_byte+remainder|.
//! The interpretation is that \TeX\ puts implicit boundary characters
//! before and after each consecutive string of characters from the same font.
//! These implicit characters do not appear in the output, but they can affect
//! ligatures and kerning.
//!
//! If the very first instruction of a character's |lig_kern| program has
//! |skip_byte>128|, the program actually begins in location
//! |256*op_byte+remainder|. This feature allows access to large |lig_kern|
//! arrays, because the first instruction must otherwise
//! appear in a location |<=255|.
//!
//! Any instruction with |skip_byte>128| in the |lig_kern| array must satisfy
//! the condition
//! $$\hbox{|256*op_byte+remainder<nl|.}$$
//! If such an instruction is encountered during
//! normal program execution, it denotes an unconditional halt; no ligature
//! or kerning command is performed.
//!
//! @d stop_flag==qi(128) {value indicating `\.{STOP}' in a lig/kern program}
//! @d kern_flag==qi(128) {op code for a kern step}
//! @d skip_byte(#)==#.b0
//! @d next_char(#)==#.b1
//! @d op_byte(#)==#.b2
//! @d rem_byte(#)==#.b3
//!
//! @ Extensible characters are specified by an |@!extensible_recipe|, which
//! consists of four bytes called |@!top|, |@!mid|, |@!bot|, and |@!rep| (in this
//! order). These bytes are the character codes of individual pieces used to
//! build up a large symbol.  If |top|, |mid|, or |bot| are zero, they are not
//! present in the built-up result. For example, an extensible vertical line is
//! like an extensible bracket, except that the top and bottom pieces are missing.
//!
//! Let $T$, $M$, $B$, and $R$ denote the respective pieces, or an empty box
//! if the piece isn't present. Then the extensible characters have the form
//! $TR^kMR^kB$ from top to bottom, for some |k>=0|, unless $M$ is absent;
//! in the latter case we can have $TR^kB$ for both even and odd values of~|k|.
//! The width of the extensible character is the width of $R$; and the
//! height-plus-depth is the sum of the individual height-plus-depths of the
//! components used, since the pieces are butted together in a vertical list.
//!
//! @d ext_top(#)==#.b0 {|top| piece in a recipe}
//! @d ext_mid(#)==#.b1 {|mid| piece in a recipe}
//! @d ext_bot(#)==#.b2 {|bot| piece in a recipe}
//! @d ext_rep(#)==#.b3 {|rep| piece in a recipe}
//!
//! @ The final portion of a \.{TFM} file is the |param| array, which is another
//! sequence of |fix_word| values.
//!
//! \yskip\hang|param[1]=slant| is the amount of italic slant, which is used
//! to help position accents. For example, |slant=.25| means that when you go
//! up one unit, you also go .25 units to the right. The |slant| is a pure
//! number; it's the only |fix_word| other than the design size itself that is
//! not scaled by the design size.
//!
//! \hang|param[2]=space| is the normal spacing between words in text.
//! Note that character |" "| in the font need not have anything to do with
//! blank spaces.
//!
//! \hang|param[3]=space_stretch| is the amount of glue stretching between words.
//!
//! \hang|param[4]=space_shrink| is the amount of glue shrinking between words.
//!
//! \hang|param[5]=x_height| is the size of one ex in the font; it is also
//! the height of letters for which accents don't have to be raised or lowered.
//!
//! \hang|param[6]=quad| is the size of one em in the font.
//!
//! \hang|param[7]=extra_space| is the amount added to |param[2]| at the
//! ends of sentences.
//!
//! \yskip\noindent
//! If fewer than seven parameters are present, \TeX\ sets the missing parameters
//! to zero. Fonts used for math symbols are required to have
//! additional parameter information, which is explained later.
//!
//! @d slant_code=1
//! @d space_code=2
//! @d space_stretch_code=3
//! @d space_shrink_code=4
//! @d x_height_code=5
//! @d quad_code=6
//! @d extra_space_code=7
//!
//! @ So that is what \.{TFM} files hold. Since \TeX\ has to absorb such information
//! about lots of fonts, it stores most of the data in a large array called
//! |font_info|. Each item of |font_info| is a |memory_word|; the |fix_word|
//! data gets converted into |scaled| entries, while everything else goes into
//! words of type |four_quarters|.
//!
//! When the user defines \.{\\font\\f}, say, \TeX\ assigns an internal number
//! to the user's font~\.{\\f}. Adding this number to |font_id_base| gives the
//! |eqtb| location of a ``frozen'' control sequence that will always select
//! the font.
//!
//! @<Types...@>=
//! @!internal_font_number=font_base..font_max; {|font| in a |char_node|}
//! @!font_index=0..font_mem_size; {index into |font_info|}
//!
//! @ Here now is the (rather formidable) array of font arrays.
//!
//! @d non_char==qi(256) {a |halfword| code that can't match a real character}
//! @d non_address=0 {a spurious |bchar_label|}
//!
//! @<Glob...@>=
//! @!font_info:array[font_index] of memory_word;
//!   {the big collection of font data}
//! @!fmem_ptr:font_index; {first unused word of |font_info|}
//! @!font_ptr:internal_font_number; {largest internal font number in use}
//! @!font_check:array[internal_font_number] of four_quarters; {check sum}
//! @!font_size:array[internal_font_number] of scaled; {``at'' size}
//! @!font_dsize:array[internal_font_number] of scaled; {``design'' size}
//! @!font_params:array[internal_font_number] of font_index; {how many font
//!   parameters are present}
//! @!font_name:array[internal_font_number] of str_number; {name of the font}
//! @!font_area:array[internal_font_number] of str_number; {area of the font}
//! @!font_bc:array[internal_font_number] of eight_bits;
//!   {beginning (smallest) character code}
//! @!font_ec:array[internal_font_number] of eight_bits;
//!   {ending (largest) character code}
//! @!font_glue:array[internal_font_number] of pointer;
//!   {glue specification for interword space, |null| if not allocated}
//! @!font_used:array[internal_font_number] of boolean;
//!   {has a character from this font actually appeared in the output?}
//! @!hyphen_char:array[internal_font_number] of integer;
//!   {current \.{\\hyphenchar} values}
//! @!skew_char:array[internal_font_number] of integer;
//!   {current \.{\\skewchar} values}
//! @!bchar_label:array[internal_font_number] of font_index;
//!   {start of |lig_kern| program for left boundary character,
//!   |non_address| if there is none}
//! @!font_bchar:array[internal_font_number] of min_quarterword..non_char;
//!   {right boundary character, |non_char| if there is none}
//! @!font_false_bchar:array[internal_font_number] of min_quarterword..non_char;
//!   {|font_bchar| if it doesn't exist in the font, otherwise |non_char|}
//!
//! @ Besides the arrays just enumerated, we have directory arrays that make it
//! easy to get at the individual entries in |font_info|. For example, the
//! |char_info| data for character |c| in font |f| will be in
//! |font_info[char_base[f]+c].qqqq|; and if |w| is the |width_index|
//! part of this word (the |b0| field), the width of the character is
//! |font_info[width_base[f]+w].sc|. (These formulas assume that
//! |min_quarterword| has already been added to |c| and to |w|, since \TeX\
//! stores its quarterwords that way.)
//!
//! @<Glob...@>=
//! @!char_base:array[internal_font_number] of integer;
//!   {base addresses for |char_info|}
//! @!width_base:array[internal_font_number] of integer;
//!   {base addresses for widths}
//! @!height_base:array[internal_font_number] of integer;
//!   {base addresses for heights}
//! @!depth_base:array[internal_font_number] of integer;
//!   {base addresses for depths}
//! @!italic_base:array[internal_font_number] of integer;
//!   {base addresses for italic corrections}
//! @!lig_kern_base:array[internal_font_number] of integer;
//!   {base addresses for ligature/kerning programs}
//! @!kern_base:array[internal_font_number] of integer;
//!   {base addresses for kerns}
//! @!exten_base:array[internal_font_number] of integer;
//!   {base addresses for extensible recipes}
//! @!param_base:array[internal_font_number] of integer;
//!   {base addresses for font parameters}
//!
//! @ @<Set init...@>=
//! for k:=font_base to font_max do font_used[k]:=false;
//!
//! @ \TeX\ always knows at least one font, namely the null font. It has no
//! characters, and its seven parameters are all equal to zero.
//!
//! @<Initialize table...@>=
//! font_ptr:=null_font; fmem_ptr:=7;
//! font_name[null_font]:="nullfont"; font_area[null_font]:="";
//! hyphen_char[null_font]:="-"; skew_char[null_font]:=-1;
//! bchar_label[null_font]:=non_address;
//! font_bchar[null_font]:=non_char; font_false_bchar[null_font]:=non_char;
//! font_bc[null_font]:=1; font_ec[null_font]:=0;
//! font_size[null_font]:=0; font_dsize[null_font]:=0;
//! char_base[null_font]:=0; width_base[null_font]:=0;
//! height_base[null_font]:=0; depth_base[null_font]:=0;
//! italic_base[null_font]:=0; lig_kern_base[null_font]:=0;
//! kern_base[null_font]:=0; exten_base[null_font]:=0;
//! font_glue[null_font]:=null; font_params[null_font]:=7;
//! param_base[null_font]:=-1;
//! for k:=0 to 6 do font_info[k].sc:=0;
//!
//! @ @<Put each...@>=
//! primitive("nullfont",set_font,null_font);
//! @!@:null_font_}{\.{\\nullfont} primitive@>
//! text(frozen_null_font):="nullfont"; eqtb[frozen_null_font]:=eqtb[cur_val];
//!
//! @ Of course we want to define macros that suppress the detail of how font
//! information is actually packed, so that we don't have to write things like
//! $$\hbox{|font_info[width_base[f]+font_info[char_base[f]+c].qqqq.b0].sc|}$$
//! too often. The \.{WEB} definitions here make |char_info(f)(c)| the
//! |four_quarters| word of font information corresponding to character
//! |c| of font |f|. If |q| is such a word, |char_width(f)(q)| will be
//! the character's width; hence the long formula above is at least
//! abbreviated to
//! $$\hbox{|char_width(f)(char_info(f)(c))|.}$$
//! Usually, of course, we will fetch |q| first and look at several of its
//! fields at the same time.
//!
//! The italic correction of a character will be denoted by
//! |char_italic(f)(q)|, so it is analogous to |char_width|.  But we will get
//! at the height and depth in a slightly different way, since we usually want
//! to compute both height and depth if we want either one.  The value of
//! |height_depth(q)| will be the 8-bit quantity
//! $$b=|height_index|\times16+|depth_index|,$$ and if |b| is such a byte we
//! will write |char_height(f)(b)| and |char_depth(f)(b)| for the height and
//! depth of the character |c| for which |q=char_info(f)(c)|. Got that?
//!
//! The tag field will be called |char_tag(q)|; the remainder byte will be
//! called |rem_byte(q)|, using a macro that we have already defined above.
//!
//! Access to a character's |width|, |height|, |depth|, and |tag| fields is
//! part of \TeX's inner loop, so we want these macros to produce code that is
//! as fast as possible under the circumstances.
//! @^inner loop@>
//!
//! @d char_info_end(#)==#].qqqq
//! @d char_info(#)==font_info[char_base[#]+char_info_end
//! @d char_width_end(#)==#.b0].sc
//! @d char_width(#)==font_info[width_base[#]+char_width_end
//! @d char_exists(#)==(#.b0>min_quarterword)
//! @d char_italic_end(#)==(qo(#.b2)) div 4].sc
//! @d char_italic(#)==font_info[italic_base[#]+char_italic_end
//! @d height_depth(#)==qo(#.b1)
//! @d char_height_end(#)==(#) div 16].sc
//! @d char_height(#)==font_info[height_base[#]+char_height_end
//! @d char_depth_end(#)==(#) mod 16].sc
//! @d char_depth(#)==font_info[depth_base[#]+char_depth_end
//! @d char_tag(#)==((qo(#.b2)) mod 4)
//!
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
//! @ When \TeX\ wants to typeset a character that doesn't exist, the
//! character node is not created; thus the output routine can assume
//! that characters exist when it sees them. The following procedure
//! prints a warning message unless the user has suppressed it.
//!
//! @p procedure char_warning(@!f:internal_font_number;@!c:eight_bits);
//! begin if tracing_lost_chars>0 then
//!   begin begin_diagnostic;
//!   print_nl("Missing character: There is no ");
//! @.Missing character@>
//!   print_ASCII(c); print(" in font ");
//!   slow_print(font_name[f]); print_char("!"); end_diagnostic(false);
//!   end;
//! end;
//!
//! @ Here is a function that returns a pointer to a character node for a
//! given character in a given font. If that character doesn't exist,
//! |null| is returned instead.
//!
//! @p function new_character(@!f:internal_font_number;@!c:eight_bits):pointer;
//! label exit;
//! var p:pointer; {newly allocated node}
//! begin if font_bc[f]<=c then if font_ec[f]>=c then
//!   if char_exists(char_info(f)(qi(c))) then
//!     begin p:=get_avail; font(p):=f; character(p):=qi(c);
//!     new_character:=p; return;
//!     end;
//! char_warning(f,c);
//! new_character:=null;
//! exit:end;
//!
