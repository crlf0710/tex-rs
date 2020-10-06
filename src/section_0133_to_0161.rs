//! @* \[10] Data structures for boxes and their friends.
//! From the computer's standpoint, \TeX's chief mission is to create
//! horizontal and vertical lists. We shall now investigate how the elements
//! of these lists are represented internally as nodes in the dynamic memory.
//!
//! A horizontal or vertical list is linked together by |link| fields in
//! the first word of each node. Individual nodes represent boxes, glue,
//! penalties, or special things like discretionary hyphens; because of this
//! variety, some nodes are longer than others, and we must distinguish different
//! kinds of nodes. We do this by putting a `|type|' field in the first word,
//! together with the link and an optional `|subtype|'.
//!
//! @d type(#) == mem[#].hh.b0 {identifies what kind of node this is}
//! @d subtype(#) == mem[#].hh.b1 {secondary identification in some cases}
//!
//! @ A |@!char_node|, which represents a single character, is the most important
//! kind of node because it accounts for the vast majority of all boxes.
//! Special precautions are therefore taken to ensure that a |char_node| does
//! not take up much memory space. Every such node is one word long, and in fact
//! it is identifiable by this property, since other kinds of nodes have at least
//! two words, and they appear in |mem| locations less than |hi_mem_min|.
//! This makes it possible to omit the |type| field in a |char_node|, leaving
//! us room for two bytes that identify a |font| and a |character| within
//! that font.
//!
//! Note that the format of a |char_node| allows for up to 256 different
//! fonts and up to 256 characters per font; but most implementations will
//! probably limit the total number of fonts to fewer than 75 per job,
//! and most fonts will stick to characters whose codes are
//! less than 128 (since higher codes
//! are more difficult to access on most keyboards).
//!
//! Extensions of \TeX\ intended for oriental languages will need even more
//! than $256\times256$ possible characters, when we consider different sizes
//! @^oriental characters@>@^Chinese characters@>@^Japanese characters@>
//! and styles of type.  It is suggested that Chinese and Japanese fonts be
//! handled by representing such characters in two consecutive |char_node|
//! entries: The first of these has |font=font_base|, and its |link| points
//! to the second;
//! the second identifies the font and the character dimensions.
//! The saving feature about oriental characters is that most of them have
//! the same box dimensions. The |character| field of the first |char_node|
//! is a ``\\{charext}'' that distinguishes between graphic symbols whose
//! dimensions are identical for typesetting purposes. (See the \MF\ manual.)
//! Such an extension of \TeX\ would not be difficult; further details are
//! left to the reader.
//!
//! In order to make sure that the |character| code fits in a quarterword,
//! \TeX\ adds the quantity |min_quarterword| to the actual code.
//!
//! Character nodes appear only in horizontal lists, never in vertical lists.
//!
//! @d is_char_node(#) == (#>=hi_mem_min)
//!   {does the argument point to a |char_node|?}
//! @d font == type {the font code in a |char_node|}
//! @d character == subtype {the character code in a |char_node|}
//!
//! @ An |hlist_node| stands for a box that was made from a horizontal list.
//! Each |hlist_node| is seven words long, and contains the following fields
//! (in addition to the mandatory |type| and |link|, which we shall not
//! mention explicitly when discussing the other node types): The |height| and
//! |width| and |depth| are scaled integers denoting the dimensions of the
//! box.  There is also a |shift_amount| field, a scaled integer indicating
//! how much this box should be lowered (if it appears in a horizontal list),
//! or how much it should be moved to the right (if it appears in a vertical
//! list). There is a |list_ptr| field, which points to the beginning of the
//! list from which this box was fabricated; if |list_ptr| is |null|, the box
//! is empty. Finally, there are three fields that represent the setting of
//! the glue:  |glue_set(p)| is a word of type |glue_ratio| that represents
//! the proportionality constant for glue setting; |glue_sign(p)| is
//! |stretching| or |shrinking| or |normal| depending on whether or not the
//! glue should stretch or shrink or remain rigid; and |glue_order(p)|
//! specifies the order of infinity to which glue setting applies (|normal|,
//! |fil|, |fill|, or |filll|). The |subtype| field is not used.
//!
//! @d hlist_node=0 {|type| of hlist nodes}
//! @d box_node_size=7 {number of words to allocate for a box node}
//! @d width_offset=1 {position of |width| field in a box node}
//! @d depth_offset=2 {position of |depth| field in a box node}
//! @d height_offset=3 {position of |height| field in a box node}
//! @d width(#) == mem[#+width_offset].sc {width of the box, in sp}
//! @d depth(#) == mem[#+depth_offset].sc {depth of the box, in sp}
//! @d height(#) == mem[#+height_offset].sc {height of the box, in sp}
//! @d shift_amount(#) == mem[#+4].sc {repositioning distance, in sp}
//! @d list_offset=5 {position of |list_ptr| field in a box node}
//! @d list_ptr(#) == link(#+list_offset) {beginning of the list inside the box}
//! @d glue_order(#) == subtype(#+list_offset) {applicable order of infinity}
//! @d glue_sign(#) == type(#+list_offset) {stretching or shrinking}
//! @d normal=0 {the most common case when several cases are named}
//! @d stretching = 1 {glue setting applies to the stretch components}
//! @d shrinking = 2 {glue setting applies to the shrink components}
//! @d glue_offset = 6 {position of |glue_set| in a box node}
//! @d glue_set(#) == mem[#+glue_offset].gr
//!   {a word of type |glue_ratio| for glue setting}
//!
//! @ The |new_null_box| function returns a pointer to an |hlist_node| in
//! which all subfields have the values corresponding to `\.{\\hbox\{\}}'.
//! The |subtype| field is set to |min_quarterword|, since that's the desired
//! |span_count| value if this |hlist_node| is changed to an |unset_node|.
//!
//! @p function new_null_box:pointer; {creates a new box node}
//! var p:pointer; {the new node}
//! begin p:=get_node(box_node_size); type(p):=hlist_node;
//! subtype(p):=min_quarterword;
//! width(p):=0; depth(p):=0; height(p):=0; shift_amount(p):=0; list_ptr(p):=null;
//! glue_sign(p):=normal; glue_order(p):=normal; set_glue_ratio_zero(glue_set(p));
//! new_null_box:=p;
//! end;
//!
//! @ A |vlist_node| is like an |hlist_node| in all respects except that it
//! contains a vertical list.
//!
//! @d vlist_node=1 {|type| of vlist nodes}
//!
//! @ A |rule_node| stands for a solid black rectangle; it has |width|,
//! |depth|, and |height| fields just as in an |hlist_node|. However, if
//! any of these dimensions is $-2^{30}$, the actual value will be determined
//! by running the rule up to the boundary of the innermost enclosing box.
//! This is called a ``running dimension.'' The |width| is never running in
//! an hlist; the |height| and |depth| are never running in a~vlist.
//!
//! @d rule_node=2 {|type| of rule nodes}
//! @d rule_node_size=4 {number of words to allocate for a rule node}
//! @d null_flag==-@'10000000000 {$-2^{30}$, signifies a missing item}
//! @d is_running(#) == (#=null_flag) {tests for a running dimension}
//!
//! @ A new rule node is delivered by the |new_rule| function. It
//! makes all the dimensions ``running,'' so you have to change the
//! ones that are not allowed to run.
//!
//! @p function new_rule:pointer;
//! var p:pointer; {the new node}
//! begin p:=get_node(rule_node_size); type(p):=rule_node;
//! subtype(p):=0; {the |subtype| is not used}
//! width(p):=null_flag; depth(p):=null_flag; height(p):=null_flag;
//! new_rule:=p;
//! end;
//!
//! @ Insertions are represented by |ins_node| records, where the |subtype|
//! indicates the corresponding box number. For example, `\.{\\insert 250}'
//! leads to an |ins_node| whose |subtype| is |250+min_quarterword|.
//! The |height| field of an |ins_node| is slightly misnamed; it actually holds
//! the natural height plus depth of the vertical list being inserted.
//! The |depth| field holds the |split_max_depth| to be used in case this
//! insertion is split, and the |split_top_ptr| points to the corresponding
//! |split_top_skip|. The |float_cost| field holds the |floating_penalty| that
//! will be used if this insertion floats to a subsequent page after a
//! split insertion of the same class.  There is one more field, the
//! |ins_ptr|, which points to the beginning of the vlist for the insertion.
//!
//! @d ins_node=3 {|type| of insertion nodes}
//! @d ins_node_size=5 {number of words to allocate for an insertion}
//! @d float_cost(#)==mem[#+1].int {the |floating_penalty| to be used}
//! @d ins_ptr(#)==info(#+4) {the vertical list to be inserted}
//! @d split_top_ptr(#)==link(#+4) {the |split_top_skip| to be used}
//!
//! @ A |mark_node| has a |mark_ptr| field that points to the reference count
//! of a token list that contains the user's \.{\\mark} text.
//! This field occupies a full word instead of a halfword, because
//! there's nothing to put in the other halfword; it is easier in \PASCAL\ to
//! use the full word than to risk leaving garbage in the unused half.
//!
//! @d mark_node=4 {|type| of a mark node}
//! @d small_node_size=2 {number of words to allocate for most node types}
//! @d mark_ptr(#)==mem[#+1].int {head of the token list for a mark}
//!
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
//! @ A |penalty_node| specifies the penalty associated with line or page
//! breaking, in its |penalty| field. This field is a fullword integer, but
//! the full range of integer values is not used: Any penalty |>=10000| is
//! treated as infinity, and no break will be allowed for such high values.
//! Similarly, any penalty |<=-10000| is treated as negative infinity, and a
//! break will be forced.
//!
//! @d penalty_node=12 {|type| of a penalty node}
//! @d inf_penalty=inf_bad {``infinite'' penalty value}
//! @d eject_penalty=-inf_penalty {``negatively infinite'' penalty value}
//! @d penalty(#) == mem[#+1].int {the added cost of breaking a list here}
//!
//! @ Anyone who has been reading the last few sections of the program will
//! be able to guess what comes next.
//!
//! @p function new_penalty(@!m:integer):pointer;
//! var p:pointer; {the new node}
//! begin p:=get_node(small_node_size); type(p):=penalty_node;
//! subtype(p):=0; {the |subtype| is not used}
//! penalty(p):=m; new_penalty:=p;
//! end;
//!
//! @ You might think that we have introduced enough node types by now. Well,
//! almost, but there is one more: An |unset_node| has nearly the same format
//! as an |hlist_node| or |vlist_node|; it is used for entries in \.{\\halign}
//! or \.{\\valign} that are not yet in their final form, since the box
//! dimensions are their ``natural'' sizes before any glue adjustment has been
//! made. The |glue_set| word is not present; instead, we have a |glue_stretch|
//! field, which contains the total stretch of order |glue_order| that is
//! present in the hlist or vlist being boxed.
//! Similarly, the |shift_amount| field is replaced by a |glue_shrink| field,
//! containing the total shrink of order |glue_sign| that is present.
//! The |subtype| field is called |span_count|; an unset box typically
//! contains the data for |qo(span_count)+1| columns.
//! Unset nodes will be changed to box nodes when alignment is completed.
//!
//! @d unset_node=13 {|type| for an unset node}
//! @d glue_stretch(#)==mem[#+glue_offset].sc {total stretch in an unset node}
//! @d glue_shrink==shift_amount {total shrink in an unset node}
//! @d span_count==subtype {indicates the number of spanned columns}
//!
//! @ In fact, there are still more types coming. When we get to math formula
//! processing we will see that a |style_node| has |type=14|; and a number
//! of larger type codes will also be defined, for use in math mode only.
//!
//! @ Warning: If any changes are made to these data structure layouts, such as
//! changing any of the node sizes or even reordering the words of nodes,
//! the |copy_node_list| procedure and the memory initialization code
//! below may have to be changed. Such potentially dangerous parts of the
//! program are listed in the index under `data structure assumptions'.
//! @!@^data structure assumptions@>
//! However, other references to the nodes are made symbolically in terms of
//! the \.{WEB} macro definitions above, so that format changes will leave
//! \TeX's other algorithms intact.
//! @^system dependencies@>
//!
