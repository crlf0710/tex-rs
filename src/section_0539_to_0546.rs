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
