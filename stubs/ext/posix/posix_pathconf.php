<?php 

#ifdef HAVE_PATHCONF
#[\Since('8.3')]
function posix_pathconf(string $path, int $name): int|false
{
}