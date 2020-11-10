//! @* \[47] Building boxes and lists.
//! The most important parts of |main_control| are concerned with \TeX's
//! chief mission of box-making. We need to control the activities that put
//! entries on vlists and hlists, as well as the activities that convert
//! those lists into boxes. All of the necessary machinery has already been
//! developed; it remains for us to ``push the buttons'' at the right times.
//!
