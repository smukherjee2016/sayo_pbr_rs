/*
    Initial draft idea v0 (20190820): each thread of the scoped threadpool will be a sender,
    and main thread will be the receiver. Each thread will calculate the final SPP-averaged
    value of the pixel(s) it is given in charge of. Possibly store the block start positions in
    the image array and make the threads work on it till all the blocks are exhausted.
*/
