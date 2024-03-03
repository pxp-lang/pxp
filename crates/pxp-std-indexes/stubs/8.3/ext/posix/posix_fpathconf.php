<?php

#endif
#ifdef HAVE_FPATHCONF
/** @param resource|int $file_descriptor */
function posix_fpathconf($file_descriptor, int $name): int|false
{
}