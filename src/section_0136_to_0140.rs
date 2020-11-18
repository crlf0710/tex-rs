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