<?php 

#ifdef HAVE_EACCESS
#[\Since('8.3')]
function posix_eaccess(string $filename, int $flags = 0): bool
{
}