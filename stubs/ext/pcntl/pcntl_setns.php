<?php 

#endif
#ifdef HAVE_PIDFD_OPEN
#[\Since('8.4')]
function pcntl_setns(?int $process_id = null, int $nstype = CLONE_NEWNET): bool
{
}