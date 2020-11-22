//! @ The active list also contains ``delta'' nodes that help the algorithm
//! compute the badness of individual lines. Such nodes appear only between two
//! active nodes, and they have |type=delta_node|. If |p| and |r| are active nodes
//! and if |q| is a delta node between them, so that |link(p)=q| and |link(q)=r|,
//! then |q| tells the space difference between lines in the horizontal list that
//! start after breakpoint |p| and lines that start after breakpoint |r|. In
//! other words, if we know the length of the line that starts after |p| and
//! ends at our current position, then the corresponding length of the line that
//! starts after |r| is obtained by adding the amounts in node~|q|. A delta node
//! contains six scaled numbers, since it must record the net change in glue
//! stretchability with respect to all orders of infinity. The natural width
//! difference appears in |mem[q+1].sc|; the stretch differences in units of
//! pt, fil, fill, and filll appear in |mem[q+2..q+5].sc|; and the shrink difference
//! appears in |mem[q+6].sc|. The |subtype| field of a delta node is not used.
//
// @d delta_node_size=7 {number of words in a delta node}
// @d delta_node=2 {|type| field in a delta node}
//
