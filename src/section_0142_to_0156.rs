//! @ An |adjust_node|, which occurs only in horizontal lists,
//! specifies material that will be moved out into the surrounding
//! vertical list; i.e., it is used to implement \TeX's `\.{\\vadjust}'
//! operation.  The |adjust_ptr| field points to the vlist containing this
//! material.
//!
//! @d adjust_node=5 {|type| of an adjust node}
//! @d adjust_ptr==mark_ptr {vertical list to be moved out of horizontal list}
//!
//! @ A |ligature_node|, which occurs only in horizontal lists, specifies
//! a character that was fabricated from the interaction of two or more
//! actual characters.  The second word of the node, which is called the
//! |lig_char| word, contains |font| and |character| fields just as in a
//! |char_node|. The characters that generated the ligature have not been
//! forgotten, since they are needed for diagnostic messages and for
//! hyphenation; the |lig_ptr| field points to a linked list of character
//! nodes for all original characters that have been deleted. (This list
//! might be empty if the characters that generated the ligature were
//! retained in other nodes.)
//!
//! The |subtype| field is 0, plus 2 and/or 1 if the original source of the
//! ligature included implicit left and/or right boundaries.
//!
//! @d ligature_node=6 {|type| of a ligature node}
//! @d lig_char(#)==#+1 {the word where the ligature is to be found}
//! @d lig_ptr(#)==link(lig_char(#)) {the list of characters}
//!
//! @ The |new_ligature| function creates a ligature node having given
//! contents of the |font|, |character|, and |lig_ptr| fields. We also have
//! a |new_lig_item| function, which returns a two-word node having a given
//! |character| field. Such nodes are used for temporary processing as ligatures
//! are being created.
//!
//! @p function new_ligature(@!f,@!c:quarterword; @!q:pointer):pointer;
//! var p:pointer; {the new node}
//! begin p:=get_node(small_node_size); type(p):=ligature_node;
//! font(lig_char(p)):=f; character(lig_char(p)):=c; lig_ptr(p):=q;
//! subtype(p):=0; new_ligature:=p;
//! end;
//! @#
//! function new_lig_item(@!c:quarterword):pointer;
//! var p:pointer; {the new node}
//! begin p:=get_node(small_node_size); character(p):=c; lig_ptr(p):=null;
//! new_lig_item:=p;
//! end;
//!
//! @ A |disc_node|, which occurs only in horizontal lists, specifies a
//! ``dis\-cretion\-ary'' line break. If such a break occurs at node |p|, the text
//! that starts at |pre_break(p)| will precede the break, the text that starts at
//! |post_break(p)| will follow the break, and text that appears in the next
//! |replace_count(p)| nodes will be ignored. For example, an ordinary
//! discretionary hyphen, indicated by `\.{\\-}', yields a |disc_node| with
//! |pre_break| pointing to a |char_node| containing a hyphen, |post_break=null|,
//! and |replace_count=0|. All three of the discretionary texts must be
//! lists that consist entirely of character, kern, box, rule, and ligature nodes.
//!
//! If |pre_break(p)=null|, the |ex_hyphen_penalty| will be charged for this
//! break.  Otherwise the |hyphen_penalty| will be charged.  The texts will
//! actually be substituted into the list by the line-breaking algorithm if it
//! decides to make the break, and the discretionary node will disappear at
//! that time; thus, the output routine sees only discretionaries that were
//! not chosen.
//!
//! @d disc_node=7 {|type| of a discretionary node}
//! @d replace_count==subtype {how many subsequent nodes to replace}
//! @d pre_break==llink {text that precedes a discretionary break}
//! @d post_break==rlink {text that follows a discretionary break}
//!
//! @p function new_disc:pointer; {creates an empty |disc_node|}
//! var p:pointer; {the new node}
//! begin p:=get_node(small_node_size); type(p):=disc_node;
//! replace_count(p):=0; pre_break(p):=null; post_break(p):=null;
//! new_disc:=p;
//! end;
//!
//! @ A |whatsit_node| is a wild card reserved for extensions to \TeX. The
//! |subtype| field in its first word says what `\\{whatsit}' it is, and
//! implicitly determines the node size (which must be 2 or more) and the
//! format of the remaining words. When a |whatsit_node| is encountered
//! in a list, special actions are invoked; knowledgeable people who are
//! careful not to mess up the rest of \TeX\ are able to make \TeX\ do new
//! things by adding code at the end of the program. For example, there
//! might be a `\TeX nicolor' extension to specify different colors of ink,
//! @^extensions to \TeX@>
//! and the whatsit node might contain the desired parameters.
//!
//! The present implementation of \TeX\ treats the features associated with
//! `\.{\\write}' and `\.{\\special}' as if they were extensions, in order to
//! illustrate how such routines might be coded. We shall defer further
//! discussion of extensions until the end of this program.
//!
//! @d whatsit_node=8 {|type| of special extension nodes}
//!
//! @ A |math_node|, which occurs only in horizontal lists, appears before and
//! after mathematical formulas. The |subtype| field is |before| before the
//! formula and |after| after it. There is a |width| field, which represents
//! the amount of surrounding space inserted by \.{\\mathsurround}.
//!
//! @d math_node=9 {|type| of a math node}
//! @d before=0 {|subtype| for math node that introduces a formula}
//! @d after=1 {|subtype| for math node that winds up a formula}
//!
//! @p function new_math(@!w:scaled;@!s:small_number):pointer;
//! var p:pointer; {the new node}
//! begin p:=get_node(small_node_size); type(p):=math_node;
//! subtype(p):=s; width(p):=w; new_math:=p;
//! end;
//!
//! @ \TeX\ makes use of the fact that |hlist_node|, |vlist_node|,
//! |rule_node|, |ins_node|, |mark_node|, |adjust_node|, |ligature_node|,
//! |disc_node|, |whatsit_node|, and |math_node| are at the low end of the
//! type codes, by permitting a break at glue in a list if and only if the
//! |type| of the previous node is less than |math_node|. Furthermore, a
//! node is discarded after a break if its type is |math_node| or~more.
//!
//! @d precedes_break(#)==(type(#)<math_node)
//! @d non_discardable(#)==(type(#)<math_node)
//!
//! @ A |glue_node| represents glue in a list. However, it is really only
//! a pointer to a separate glue specification, since \TeX\ makes use of the
//! fact that many essentially identical nodes of glue are usually present.
//! If |p| points to a |glue_node|, |glue_ptr(p)| points to
//! another packet of words that specify the stretch and shrink components, etc.
//!
//! Glue nodes also serve to represent leaders; the |subtype| is used to
//! distinguish between ordinary glue (which is called |normal|) and the three
//! kinds of leaders (which are called |a_leaders|, |c_leaders|, and |x_leaders|).
//! The |leader_ptr| field points to a rule node or to a box node containing the
//! leaders; it is set to |null| in ordinary glue nodes.
//!
//! Many kinds of glue are computed from \TeX's ``skip'' parameters, and
//! it is helpful to know which parameter has led to a particular glue node.
//! Therefore the |subtype| is set to indicate the source of glue, whenever
//! it originated as a parameter. We will be defining symbolic names for the
//! parameter numbers later (e.g., |line_skip_code=0|, |baseline_skip_code=1|,
//! etc.); it suffices for now to say that the |subtype| of parametric glue
//! will be the same as the parameter number, plus~one.
//!
//! In math formulas there are two more possibilities for the |subtype| in a
//! glue node: |mu_glue| denotes an \.{\\mskip} (where the units are scaled \.{mu}
//! instead of scaled \.{pt}); and |cond_math_glue| denotes the `\.{\\nonscript}'
//! feature that cancels the glue node immediately following if it appears
//! in a subscript.
//!
//! @d glue_node=10 {|type| of node that points to a glue specification}
//! @d cond_math_glue=98 {special |subtype| to suppress glue in the next node}
//! @d mu_glue=99 {|subtype| for math glue}
//! @d a_leaders=100 {|subtype| for aligned leaders}
//! @d c_leaders=101 {|subtype| for centered leaders}
//! @d x_leaders=102 {|subtype| for expanded leaders}
//! @d glue_ptr==llink {pointer to a glue specification}
//! @d leader_ptr==rlink {pointer to box or rule node for leaders}
//!
//! @ A glue specification has a halfword reference count in its first word,
//! @^reference counts@>
//! representing |null| plus the number of glue nodes that point to it (less one).
//! Note that the reference count appears in the same position as
//! the |link| field in list nodes; this is the field that is initialized
//! to |null| when a node is allocated, and it is also the field that is flagged
//! by |empty_flag| in empty nodes.
//!
//! Glue specifications also contain three |scaled| fields, for the |width|,
//! |stretch|, and |shrink| dimensions. Finally, there are two one-byte
//! fields called |stretch_order| and |shrink_order|; these contain the
//! orders of infinity (|normal|, |fil|, |fill|, or |filll|)
//! corresponding to the stretch and shrink values.
//!
//! @d glue_spec_size=4 {number of words to allocate for a glue specification}
//! @d glue_ref_count(#) == link(#) {reference count of a glue specification}
//! @d stretch(#) == mem[#+2].sc {the stretchability of this glob of glue}
//! @d shrink(#) == mem[#+3].sc {the shrinkability of this glob of glue}
//! @d stretch_order == type {order of infinity for stretching}
//! @d shrink_order == subtype {order of infinity for shrinking}
//! @d fil=1 {first-order infinity}
//! @d fill=2 {second-order infinity}
//! @d filll=3 {third-order infinity}
//!
//! @<Types...@>=
//! @!glue_ord=normal..filll; {infinity to the 0, 1, 2, or 3 power}
//!
//! @ Here is a function that returns a pointer to a copy of a glue spec.
//! The reference count in the copy is |null|, because there is assumed
//! to be exactly one reference to the new specification.
//!
//! @p function new_spec(@!p:pointer):pointer; {duplicates a glue specification}
//! var q:pointer; {the new spec}
//! begin q:=get_node(glue_spec_size);@/
//! mem[q]:=mem[p]; glue_ref_count(q):=null;@/
//! width(q):=width(p); stretch(q):=stretch(p); shrink(q):=shrink(p);
//! new_spec:=q;
//! end;
//!
//! @ And here's a function that creates a glue node for a given parameter
//! identified by its code number; for example,
//! |new_param_glue(line_skip_code)| returns a pointer to a glue node for the
//! current \.{\\lineskip}.
//!
//! @p function new_param_glue(@!n:small_number):pointer;
//! var p:pointer; {the new node}
//! @!q:pointer; {the glue specification}
//! begin p:=get_node(small_node_size); type(p):=glue_node; subtype(p):=n+1;
//! leader_ptr(p):=null;@/
//! q:=@<Current |mem| equivalent of glue parameter number |n|@>@t@>;
//! glue_ptr(p):=q; incr(glue_ref_count(q));
//! new_param_glue:=p;
//! end;
//!
//! @ Glue nodes that are more or less anonymous are created by |new_glue|,
//! whose argument points to a glue specification.
//!
//! @p function new_glue(@!q:pointer):pointer;
//! var p:pointer; {the new node}
//! begin p:=get_node(small_node_size); type(p):=glue_node; subtype(p):=normal;
//! leader_ptr(p):=null; glue_ptr(p):=q; incr(glue_ref_count(q));
//! new_glue:=p;
//! end;
//!
//! @ Still another subroutine is needed: This one is sort of a combination
//! of |new_param_glue| and |new_glue|. It creates a glue node for one of
//! the current glue parameters, but it makes a fresh copy of the glue
//! specification, since that specification will probably be subject to change,
//! while the parameter will stay put. The global variable |temp_ptr| is
//! set to the address of the new spec.
//!
//! @p function new_skip_param(@!n:small_number):pointer;
//! var p:pointer; {the new node}
//! begin temp_ptr:=new_spec(@<Current |mem| equivalent of glue parameter...@>);
//! p:=new_glue(temp_ptr); glue_ref_count(temp_ptr):=null; subtype(p):=n+1;
//! new_skip_param:=p;
//! end;
//!
//! @ A |kern_node| has a |width| field to specify a (normally negative)
//! amount of spacing. This spacing correction appears in horizontal lists
//! between letters like A and V when the font designer said that it looks
//! better to move them closer together or further apart. A kern node can
//! also appear in a vertical list, when its `|width|' denotes additional
//! spacing in the vertical direction. The |subtype| is either |normal| (for
//! kerns inserted from font information or math mode calculations) or |explicit|
//! (for kerns inserted from \.{\\kern} and \.{\\/} commands) or |acc_kern|
//! (for kerns inserted from non-math accents) or |mu_glue| (for kerns
//! inserted from \.{\\mkern} specifications in math formulas).
//!
//! @d kern_node=11 {|type| of a kern node}
//! @d explicit=1 {|subtype| of kern nodes from \.{\\kern} and \.{\\/}}
//! @d acc_kern=2 {|subtype| of kern nodes from accents}
//!
//! @ The |new_kern| function creates a kern node having a given width.
//!
//! @p function new_kern(@!w:scaled):pointer;
//! var p:pointer; {the new node}
//! begin p:=get_node(small_node_size); type(p):=kern_node;
//! subtype(p):=normal;
//! width(p):=w;
//! new_kern:=p;
//! end;
//!
