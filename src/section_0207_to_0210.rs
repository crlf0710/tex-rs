//! @* \[15] The command codes.
//! Before we can go any further, we need to define symbolic names for the internal
//! code numbers that represent the various commands obeyed by \TeX. These codes
//! are somewhat arbitrary, but not completely so. For example, the command
//! codes for character types are fixed by the language, since a user says,
//! e.g., `\.{\\catcode \`\\\${} = 3}' to make \.{\char'44} a math delimiter,
//! and the command code |math_shift| is equal to~3. Some other codes have
//! been made adjacent so that |case| statements in the program need not consider
//! cases that are widely spaced, or so that |case| statements can be replaced
//! by |if| statements.
//!
//! At any rate, here is the list, for future reference. First come the
//! ``catcode'' commands, several of which share their numeric codes with
//! ordinary commands when the catcode cannot emerge from \TeX's scanning routine.
//!
//! @d escape=0 {escape delimiter (called \.\\ in {\sl The \TeX book\/})}
//! @:TeXbook}{\sl The \TeX book@>
//! @d relax=0 {do nothing ( \.{\\relax} )}
//! @d left_brace=1 {beginning of a group ( \.\{ )}
//! @d right_brace=2 {ending of a group ( \.\} )}
//! @d math_shift=3 {mathematics shift character ( \.\$ )}
//! @d tab_mark=4 {alignment delimiter ( \.\&, \.{\\span} )}
//! @d car_ret=5 {end of line ( |carriage_return|, \.{\\cr}, \.{\\crcr} )}
//! @d out_param=5 {output a macro parameter}
//! @d mac_param=6 {macro parameter symbol ( \.\# )}
//! @d sup_mark=7 {superscript ( \.{\char'136} )}
//! @d sub_mark=8 {subscript ( \.{\char'137} )}
//! @d ignore=9 {characters to ignore ( \.{\^\^@@} )}
//! @d endv=9 {end of \<v_j> list in alignment template}
//! @d spacer=10 {characters equivalent to blank space ( \.{\ } )}
//! @d letter=11 {characters regarded as letters ( \.{A..Z}, \.{a..z} )}
//! @d other_char=12 {none of the special character types}
//! @d active_char=13 {characters that invoke macros ( \.{\char`\~} )}
//! @d par_end=13 {end of paragraph ( \.{\\par} )}
//! @d match=13 {match a macro parameter}
//! @d comment=14 {characters that introduce comments ( \.\% )}
//! @d end_match=14 {end of parameters to macro}
//! @d stop=14 {end of job ( \.{\\end}, \.{\\dump} )}
//! @d invalid_char=15 {characters that shouldn't appear ( \.{\^\^?} )}
//! @d delim_num=15 {specify delimiter numerically ( \.{\\delimiter} )}
//! @d max_char_code=15 {largest catcode for individual characters}
//!
//! @ Next are the ordinary run-of-the-mill command codes.  Codes that are
//! |min_internal| or more represent internal quantities that might be
//! expanded by `\.{\\the}'.
//!
//! @d char_num=16 {character specified numerically ( \.{\\char} )}
//! @d math_char_num=17 {explicit math code ( \.{\\mathchar} )}
//! @d mark=18 {mark definition ( \.{\\mark} )}
//! @d xray=19 {peek inside of \TeX\ ( \.{\\show}, \.{\\showbox}, etc.~)}
//! @d make_box=20 {make a box ( \.{\\box}, \.{\\copy}, \.{\\hbox}, etc.~)}
//! @d hmove=21 {horizontal motion ( \.{\\moveleft}, \.{\\moveright} )}
//! @d vmove=22 {vertical motion ( \.{\\raise}, \.{\\lower} )}
//! @d un_hbox=23 {unglue a box ( \.{\\unhbox}, \.{\\unhcopy} )}
//! @d un_vbox=24 {unglue a box ( \.{\\unvbox}, \.{\\unvcopy} )}
//! @d remove_item=25 {nullify last item ( \.{\\unpenalty},
//!   \.{\\unkern}, \.{\\unskip} )}
//! @d hskip=26 {horizontal glue ( \.{\\hskip}, \.{\\hfil}, etc.~)}
//! @d vskip=27 {vertical glue ( \.{\\vskip}, \.{\\vfil}, etc.~)}
//! @d mskip=28 {math glue ( \.{\\mskip} )}
//! @d kern=29 {fixed space ( \.{\\kern})}
//! @d mkern=30 {math kern ( \.{\\mkern} )}
//! @d leader_ship=31 {use a box ( \.{\\shipout}, \.{\\leaders}, etc.~)}
//! @d halign=32 {horizontal table alignment ( \.{\\halign} )}
//! @d valign=33 {vertical table alignment ( \.{\\valign} )}
//! @d no_align=34 {temporary escape from alignment ( \.{\\noalign} )}
//! @d vrule=35 {vertical rule ( \.{\\vrule} )}
//! @d hrule=36 {horizontal rule ( \.{\\hrule} )}
//! @d insert=37 {vlist inserted in box ( \.{\\insert} )}
//! @d vadjust=38 {vlist inserted in enclosing paragraph ( \.{\\vadjust} )}
//! @d ignore_spaces=39 {gobble |spacer| tokens ( \.{\\ignorespaces} )}
//! @d after_assignment=40 {save till assignment is done ( \.{\\afterassignment} )}
//! @d after_group=41 {save till group is done ( \.{\\aftergroup} )}
//! @d break_penalty=42 {additional badness ( \.{\\penalty} )}
//! @d start_par=43 {begin paragraph ( \.{\\indent}, \.{\\noindent} )}
//! @d ital_corr=44 {italic correction ( \.{\\/} )}
//! @d accent=45 {attach accent in text ( \.{\\accent} )}
//! @d math_accent=46 {attach accent in math ( \.{\\mathaccent} )}
//! @d discretionary=47 {discretionary texts ( \.{\\-}, \.{\\discretionary} )}
//! @d eq_no=48 {equation number ( \.{\\eqno}, \.{\\leqno} )}
//! @d left_right=49 {variable delimiter ( \.{\\left}, \.{\\right} )}
//! @d math_comp=50 {component of formula ( \.{\\mathbin}, etc.~)}
//! @d limit_switch=51 {diddle limit conventions ( \.{\\displaylimits}, etc.~)}
//! @d above=52 {generalized fraction ( \.{\\above}, \.{\\atop}, etc.~)}
//! @d math_style=53 {style specification ( \.{\\displaystyle}, etc.~)}
//! @d math_choice=54 {choice specification ( \.{\\mathchoice} )}
//! @d non_script=55 {conditional math glue ( \.{\\nonscript} )}
//! @d vcenter=56 {vertically center a vbox ( \.{\\vcenter} )}
//! @d case_shift=57 {force specific case ( \.{\\lowercase}, \.{\\uppercase}~)}
//! @d message=58 {send to user ( \.{\\message}, \.{\\errmessage} )}
//! @d extension=59 {extensions to \TeX\ ( \.{\\write}, \.{\\special}, etc.~)}
//! @d in_stream=60 {files for reading ( \.{\\openin}, \.{\\closein} )}
//! @d begin_group=61 {begin local grouping ( \.{\\begingroup} )}
//! @d end_group=62 {end local grouping ( \.{\\endgroup} )}
//! @d omit=63 {omit alignment template ( \.{\\omit} )}
//! @d ex_space=64 {explicit space ( \.{\\\ } )}
//! @d no_boundary=65 {suppress boundary ligatures ( \.{\\noboundary} )}
//! @d radical=66 {square root and similar signs ( \.{\\radical} )}
//! @d end_cs_name=67 {end control sequence ( \.{\\endcsname} )}
//! @d min_internal=68 {the smallest code that can follow \.{\\the}}
//! @d char_given=68 {character code defined by \.{\\chardef}}
//! @d math_given=69 {math code defined by \.{\\mathchardef}}
//! @d last_item=70 {most recent item ( \.{\\lastpenalty},
//!   \.{\\lastkern}, \.{\\lastskip} )}
//! @d max_non_prefixed_command=70 {largest command code that can't be \.{\\global}}
//!
//! @ The next codes are special; they all relate to mode-independent
//! assignment of values to \TeX's internal registers or tables.
//! Codes that are |max_internal| or less represent internal quantities
//! that might be expanded by `\.{\\the}'.
//!
//! @d toks_register=71 {token list register ( \.{\\toks} )}
//! @d assign_toks=72 {special token list ( \.{\\output}, \.{\\everypar}, etc.~)}
//! @d assign_int=73 {user-defined integer ( \.{\\tolerance}, \.{\\day}, etc.~)}
//! @d assign_dimen=74 {user-defined length ( \.{\\hsize}, etc.~)}
//! @d assign_glue=75 {user-defined glue ( \.{\\baselineskip}, etc.~)}
//! @d assign_mu_glue=76 {user-defined muglue ( \.{\\thinmuskip}, etc.~)}
//! @d assign_font_dimen=77 {user-defined font dimension ( \.{\\fontdimen} )}
//! @d assign_font_int=78 {user-defined font integer ( \.{\\hyphenchar},
//!   \.{\\skewchar} )}
//! @d set_aux=79 {specify state info ( \.{\\spacefactor}, \.{\\prevdepth} )}
//! @d set_prev_graf=80 {specify state info ( \.{\\prevgraf} )}
//! @d set_page_dimen=81 {specify state info ( \.{\\pagegoal}, etc.~)}
//! @d set_page_int=82 {specify state info ( \.{\\deadcycles},
//!   \.{\\insertpenalties} )}
//! @d set_box_dimen=83 {change dimension of box ( \.{\\wd}, \.{\\ht}, \.{\\dp} )}
//! @d set_shape=84 {specify fancy paragraph shape ( \.{\\parshape} )}
//! @d def_code=85 {define a character code ( \.{\\catcode}, etc.~)}
//! @d def_family=86 {declare math fonts ( \.{\\textfont}, etc.~)}
//! @d set_font=87 {set current font ( font identifiers )}
//! @d def_font=88 {define a font file ( \.{\\font} )}
//! @d register=89 {internal register ( \.{\\count}, \.{\\dimen}, etc.~)}
//! @d max_internal=89 {the largest code that can follow \.{\\the}}
//! @d advance=90 {advance a register or parameter ( \.{\\advance} )}
//! @d multiply=91 {multiply a register or parameter ( \.{\\multiply} )}
//! @d divide=92 {divide a register or parameter ( \.{\\divide} )}
//! @d prefix=93 {qualify a definition ( \.{\\global}, \.{\\long}, \.{\\outer} )}
//! @d let=94 {assign a command code ( \.{\\let}, \.{\\futurelet} )}
//! @d shorthand_def=95 {code definition ( \.{\\chardef}, \.{\\countdef}, etc.~)}
//! @d read_to_cs=96 {read into a control sequence ( \.{\\read} )}
//! @d def=97 {macro definition ( \.{\\def}, \.{\\gdef}, \.{\\xdef}, \.{\\edef} )}
//! @d set_box=98 {set a box ( \.{\\setbox} )}
//! @d hyph_data=99 {hyphenation data ( \.{\\hyphenation}, \.{\\patterns} )}
//! @d set_interaction=100 {define level of interaction ( \.{\\batchmode}, etc.~)}
//! @d max_command=100 {the largest command code seen at |big_switch|}
//!
//! @ The remaining command codes are extra special, since they cannot get through
//! \TeX's scanner to the main control routine. They have been given values higher
//! than |max_command| so that their special nature is easily discernible.
//! The ``expandable'' commands come first.
//!
//! @d undefined_cs=max_command+1 {initial state of most |eq_type| fields}
//! @d expand_after=max_command+2 {special expansion ( \.{\\expandafter} )}
//! @d no_expand=max_command+3 {special nonexpansion ( \.{\\noexpand} )}
//! @d input=max_command+4 {input a source file ( \.{\\input}, \.{\\endinput} )}
//! @d if_test=max_command+5 {conditional text ( \.{\\if}, \.{\\ifcase}, etc.~)}
//! @d fi_or_else=max_command+6 {delimiters for conditionals ( \.{\\else}, etc.~)}
//! @d cs_name=max_command+7 {make a control sequence from tokens ( \.{\\csname} )}
//! @d convert=max_command+8 {convert to text ( \.{\\number}, \.{\\string}, etc.~)}
//! @d the=max_command+9 {expand an internal quantity ( \.{\\the} )}
//! @d top_bot_mark=max_command+10 {inserted mark ( \.{\\topmark}, etc.~)}
//! @d call=max_command+11 {non-long, non-outer control sequence}
//! @d long_call=max_command+12 {long, non-outer control sequence}
//! @d outer_call=max_command+13 {non-long, outer control sequence}
//! @d long_outer_call=max_command+14 {long, outer control sequence}
//! @d end_template=max_command+15 {end of an alignment template}
//! @d dont_expand=max_command+16 {the following token was marked by \.{\\noexpand}}
//! @d glue_ref=max_command+17 {the equivalent points to a glue specification}
//! @d shape_ref=max_command+18 {the equivalent points to a parshape specification}
//! @d box_ref=max_command+19 {the equivalent points to a box node, or is |null|}
//! @d data=max_command+20 {the equivalent is simply a halfword number}
//!
