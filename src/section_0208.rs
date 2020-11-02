//! @ Next are the ordinary run-of-the-mill command codes.  Codes that are
//! |min_internal| or more represent internal quantities that might be
//! expanded by `\.{\\the}'.

// @d char_num=16 {character specified numerically ( \.{\\char} )}
/// character specified numerically ( `\char` )
pub(crate) const char_num: quarterword = 16;
// @d math_char_num=17 {explicit math code ( \.{\\mathchar} )}
// @d mark=18 {mark definition ( \.{\\mark} )}
// @d xray=19 {peek inside of \TeX\ ( \.{\\show}, \.{\\showbox}, etc.~)}
// @d make_box=20 {make a box ( \.{\\box}, \.{\\copy}, \.{\\hbox}, etc.~)}
// @d hmove=21 {horizontal motion ( \.{\\moveleft}, \.{\\moveright} )}
// @d vmove=22 {vertical motion ( \.{\\raise}, \.{\\lower} )}
// @d un_hbox=23 {unglue a box ( \.{\\unhbox}, \.{\\unhcopy} )}
// @d un_vbox=24 {unglue a box ( \.{\\unvbox}, \.{\\unvcopy} )}
// @d remove_item=25 {nullify last item ( \.{\\unpenalty},
//   \.{\\unkern}, \.{\\unskip} )}
// @d hskip=26 {horizontal glue ( \.{\\hskip}, \.{\\hfil}, etc.~)}
// @d vskip=27 {vertical glue ( \.{\\vskip}, \.{\\vfil}, etc.~)}
// @d mskip=28 {math glue ( \.{\\mskip} )}
// @d kern=29 {fixed space ( \.{\\kern})}
// @d mkern=30 {math kern ( \.{\\mkern} )}
// @d leader_ship=31 {use a box ( \.{\\shipout}, \.{\\leaders}, etc.~)}
// @d halign=32 {horizontal table alignment ( \.{\\halign} )}
// @d valign=33 {vertical table alignment ( \.{\\valign} )}
// @d no_align=34 {temporary escape from alignment ( \.{\\noalign} )}
// @d vrule=35 {vertical rule ( \.{\\vrule} )}
// @d hrule=36 {horizontal rule ( \.{\\hrule} )}
// @d insert=37 {vlist inserted in box ( \.{\\insert} )}
// @d vadjust=38 {vlist inserted in enclosing paragraph ( \.{\\vadjust} )}
// @d ignore_spaces=39 {gobble |spacer| tokens ( \.{\\ignorespaces} )}
/// gobble `spacer` tokens ( `\ignorespaces` )
pub(crate) const ignore_spaces: quarterword = 39;
// @d after_assignment=40 {save till assignment is done ( \.{\\afterassignment} )}
// @d after_group=41 {save till group is done ( \.{\\aftergroup} )}
// @d break_penalty=42 {additional badness ( \.{\\penalty} )}
// @d start_par=43 {begin paragraph ( \.{\\indent}, \.{\\noindent} )}
// @d ital_corr=44 {italic correction ( \.{\\/} )}
// @d accent=45 {attach accent in text ( \.{\\accent} )}
// @d math_accent=46 {attach accent in math ( \.{\\mathaccent} )}
// @d discretionary=47 {discretionary texts ( \.{\\-}, \.{\\discretionary} )}
// @d eq_no=48 {equation number ( \.{\\eqno}, \.{\\leqno} )}
// @d left_right=49 {variable delimiter ( \.{\\left}, \.{\\right} )}
// @d math_comp=50 {component of formula ( \.{\\mathbin}, etc.~)}
// @d limit_switch=51 {diddle limit conventions ( \.{\\displaylimits}, etc.~)}
// @d above=52 {generalized fraction ( \.{\\above}, \.{\\atop}, etc.~)}
// @d math_style=53 {style specification ( \.{\\displaystyle}, etc.~)}
// @d math_choice=54 {choice specification ( \.{\\mathchoice} )}
// @d non_script=55 {conditional math glue ( \.{\\nonscript} )}
// @d vcenter=56 {vertically center a vbox ( \.{\\vcenter} )}
// @d case_shift=57 {force specific case ( \.{\\lowercase}, \.{\\uppercase}~)}
// @d message=58 {send to user ( \.{\\message}, \.{\\errmessage} )}
// @d extension=59 {extensions to \TeX\ ( \.{\\write}, \.{\\special}, etc.~)}
// @d in_stream=60 {files for reading ( \.{\\openin}, \.{\\closein} )}
// @d begin_group=61 {begin local grouping ( \.{\\begingroup} )}
// @d end_group=62 {end local grouping ( \.{\\endgroup} )}
// @d omit=63 {omit alignment template ( \.{\\omit} )}
/// omit alignment template ( `\omit` )
pub(crate) const omit: quarterword = 63;
// @d ex_space=64 {explicit space ( \.{\\\ } )}
/// explicit space (`\ `)
pub(crate) const ex_space: quarterword = 64;
// @d no_boundary=65 {suppress boundary ligatures ( \.{\\noboundary} )}
/// suppress boundary ligatures ( `\noboundary` )
pub(crate) const no_boundary: quarterword = 65;
// @d radical=66 {square root and similar signs ( \.{\\radical} )}
// @d end_cs_name=67 {end control sequence ( \.{\\endcsname} )}
// @d min_internal=68 {the smallest code that can follow \.{\\the}}
// @d char_given=68 {character code defined by \.{\\chardef}}
/// character code defined by `\chardef`
pub(crate) const char_given: quarterword = 68;
// @d math_given=69 {math code defined by \.{\\mathchardef}}
// @d last_item=70 {most recent item ( \.{\\lastpenalty},
//   \.{\\lastkern}, \.{\\lastskip} )}
// @d max_non_prefixed_command=70 {largest command code that can't be \.{\\global}}

use crate::section_0113::quarterword;
