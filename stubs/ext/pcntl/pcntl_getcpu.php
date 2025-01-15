<?php 

#endif
#ifdef HAVE_SCHED_GETCPU
#[\Since('8.4')]
function pcntl_getcpu(): int
{
}