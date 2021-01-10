//! @ Next are the ordinary run-of-the-mill command codes.  Codes that are
//! |min_internal| or more represent internal quantities that might be
//! expanded by `\.{\\the}'.

// @d char_num=16 {character specified numerically ( \.{\\char} )}
/// character specified numerically ( `\char` )
pub(crate) const char_num: quarterword = 16;
// @d math_char_num=17 {explicit math code ( \.{\\mathchar} )}
// @d mark=18 {mark definition ( \.{\\mark} )}
// @d xray=19 {peek inside of \TeX\ ( \.{\\show}, \.{\\showbox}, etc.~)}
/// peek inside of `TeX` ( `\show`, `\showbox`, etc. )
pub(crate) const xray: quarterword = 19;
// @d make_box=20 {make a box ( \.{\\box}, \.{\\copy}, \.{\\hbox}, etc.~)}
/// make a box ( `\box`, `\copy`, `\hbox`, etc. )
pub(crate) const make_box: quarterword = 20;
// @d hmove=21 {horizontal motion ( \.{\\moveleft}, \.{\\moveright} )}
/// horizontal motion ( `\moveleft`, `\moveright` )
pub(crate) const hmove: quarterword = 21;
// @d vmove=22 {vertical motion ( \.{\\raise}, \.{\\lower} )}
/// vertical motion ( `\raise`, `\lower` )
pub(crate) const vmove: quarterword = 22;
// @d un_hbox=23 {unglue a box ( \.{\\unhbox}, \.{\\unhcopy} )}
/// unglue a box ( `\unhbox`, `\unhcopy` )
pub(crate) const un_hbox: quarterword = 23;
// @d un_vbox=24 {unglue a box ( \.{\\unvbox}, \.{\\unvcopy} )}
/// unglue a box ( `\unvbox`, `\unvcopy` )
pub(crate) const un_vbox: quarterword = 24;
// @d remove_item=25 {nullify last item ( \.{\\unpenalty},
//   \.{\\unkern}, \.{\\unskip} )}
// @d hskip=26 {horizontal glue ( \.{\\hskip}, \.{\\hfil}, etc.~)}
/// horizontal glue ( `\hskip`, `\hfil`, etc. )
pub(crate) const hskip: quarterword = 26;
// @d vskip=27 {vertical glue ( \.{\\vskip}, \.{\\vfil}, etc.~)}
/// vertical glue ( `\vskip`, `\vfil`, etc. )
pub(crate) const vskip: quarterword = 27;
// @d mskip=28 {math glue ( \.{\\mskip} )}
/// math glue ( `\mskip` )
pub(crate) const mskip: quarterword = 28;
// @d kern=29 {fixed space ( \.{\\kern})}
/// fixed space ( `\kern`)
pub(crate) const kern: quarterword = 29;
// @d mkern=30 {math kern ( \.{\\mkern} )}
/// math kern ( `\mkern` )
pub(crate) const mkern: quarterword = 30;
// @d leader_ship=31 {use a box ( \.{\\shipout}, \.{\\leaders}, etc.~)}
/// use a box ( `\shipout`, `\leaders`, etc. )
pub(crate) const leader_ship: quarterword = 31;
// @d halign=32 {horizontal table alignment ( \.{\\halign} )}
/// horizontal table alignment ( `\halign` )
pub(crate) const halign: quarterword = 32;
// @d valign=33 {vertical table alignment ( \.{\\valign} )}
/// vertical table alignment ( `\valign` )
pub(crate) const valign: quarterword = 33;
// @d no_align=34 {temporary escape from alignment ( \.{\\noalign} )}
/// temporary escape from alignment ( `\noalign` )
pub(crate) const no_align: quarterword = 34;
// @d vrule=35 {vertical rule ( \.{\\vrule} )}
/// vertical rule ( `\vrule` )
pub(crate) const vrule: quarterword = 35;
// @d hrule=36 {horizontal rule ( \.{\\hrule} )}
/// horizontal rule ( `\hrule` )
pub(crate) const hrule: quarterword = 36;
// @d insert=37 {vlist inserted in box ( \.{\\insert} )}
// @d vadjust=38 {vlist inserted in enclosing paragraph ( \.{\\vadjust} )}
/// vlist inserted in enclosing paragraph ( `\vadjust` )
pub(crate) const vadjust: quarterword = 38;
// @d ignore_spaces=39 {gobble |spacer| tokens ( \.{\\ignorespaces} )}
/// gobble `spacer` tokens ( `\ignorespaces` )
pub(crate) const ignore_spaces: quarterword = 39;
// @d after_assignment=40 {save till assignment is done ( \.{\\afterassignment} )}
// @d after_group=41 {save till group is done ( \.{\\aftergroup} )}
/// save till group is done ( `\aftergroup` )
pub(crate) const after_group: quarterword = 41;
// @d break_penalty=42 {additional badness ( \.{\\penalty} )}
/// additional badness ( `\penalty` )
pub(crate) const break_penalty: quarterword = 42;
// @d start_par=43 {begin paragraph ( \.{\\indent}, \.{\\noindent} )}
/// begin paragraph ( `\indent`, `\noindent` )
pub(crate) const start_par: quarterword = 43;
// @d ital_corr=44 {italic correction ( \.{\\/} )}
// @d accent=45 {attach accent in text ( \.{\\accent} )}
/// attach accent in text ( `\accent` )
pub(crate) const accent: quarterword = 45;
// @d math_accent=46 {attach accent in math ( \.{\\mathaccent} )}
// @d discretionary=47 {discretionary texts ( \.{\\-}, \.{\\discretionary} )}
/// discretionary texts ( `\-`, `\discretionary` )
pub(crate) const discretionary: quarterword = 47;
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
/// force specific case ( `\lowercase`, `\uppercase` )
pub(crate) const case_shift: quarterword = 57;
// @d message=58 {send to user ( \.{\\message}, \.{\\errmessage} )}
/// send to user ( `\message`, `\errmessage` )
pub(crate) const message: quarterword = 58;
// @d extension=59 {extensions to \TeX\ ( \.{\\write}, \.{\\special}, etc.~)}
/// extensions to `TeX` ( `\write`, `\special`, etc. )
pub(crate) const extension: quarterword = 59;
// @d in_stream=60 {files for reading ( \.{\\openin}, \.{\\closein} )}
/// files for reading ( `\openin`, `\closein` )
pub(crate) const in_stream: quarterword = 60;
// @d begin_group=61 {begin local grouping ( \.{\\begingroup} )}
/// begin local grouping ( `\begingroup` )
pub(crate) const begin_group: quarterword = 61;
// @d end_group=62 {end local grouping ( \.{\\endgroup} )}
/// end local grouping ( `\endgroup` )
pub(crate) const end_group: quarterword = 62;
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
/// end control sequence ( `\endcsname` )
pub(crate) const end_cs_name: quarterword = 67;
// @d min_internal=68 {the smallest code that can follow \.{\\the}}
/// the smallest code that can follow `\the`
pub(crate) const min_internal: quarterword = 68;
// @d char_given=68 {character code defined by \.{\\chardef}}
/// character code defined by `\chardef`
pub(crate) const char_given: quarterword = 68;
// @d math_given=69 {math code defined by \.{\\mathchardef}}
/// math code defined by `\mathchardef`
pub(crate) const math_given: quarterword = 69;
// @d last_item=70 {most recent item ( \.{\\lastpenalty},
//   \.{\\lastkern}, \.{\\lastskip} )}
/// most recent item ( `\lastpenalty`, `\lastkern`, `\lastskip` )
pub(crate) const last_item: quarterword = 70;
// @d max_non_prefixed_command=70 {largest command code that can't be \.{\\global}}
/// largest command code that can't be `\global`
pub(crate) const max_non_prefixed_command: quarterword = 70;

use crate::section_0113::quarterword;
