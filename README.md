# rust-doublepivot-quicksort
Double pivot implementation of quicksort for Rust.

It is based on Vladimir Yaroslavskiy's implementation:
http://codeblab.com/wp-content/uploads/2009/09/DualPivotQuicksort.pdf

There is still room for improvement, for example using insertion sort for slices smaller than 27 or not always using the first and last elements as pivots.
