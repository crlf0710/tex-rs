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
