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
