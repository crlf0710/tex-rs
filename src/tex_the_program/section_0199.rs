//! @* \[13] Destroying boxes.
//! When we are done with a node list, we are obliged to return it to free
//! storage, including all of its sublists. The recursive procedure
//! |flush_node_list| does this for us.
//!
