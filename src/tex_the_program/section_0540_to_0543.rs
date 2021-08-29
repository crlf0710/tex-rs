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
