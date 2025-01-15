<?php 

#if defined (HAVE_WAITID) && defined (HAVE_POSIX_IDTYPES) && defined (HAVE_DECL_WEXITED) && HAVE_DECL_WEXITED == 1
/** @param array $info */
#[\Since('8.4')]
function pcntl_waitid(int $idtype = P_ALL, ?int $id = null, &$info = [], int $flags = WEXITED): bool
{
}