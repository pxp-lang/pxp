<?php 

/* main/output.c */
/** @param callable $callback */
#[\Until('8.4')]
function ob_start($callback = null, int $chunk_size = 0, int $flags = PHP_OUTPUT_HANDLER_STDFLAGS): bool
{
}
/* main/output.c */
/** @param callable|null $callback */
#[\Since('8.4')]
function ob_start($callback = null, int $chunk_size = 0, int $flags = PHP_OUTPUT_HANDLER_STDFLAGS): bool
{
}