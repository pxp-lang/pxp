<?php

#ifdef HAVE_PATHCONF
function posix_pathconf(string $path, int $name): int|false
{
}