<?php

/* {{{ html.c */
function htmlspecialchars(string $string, int $flags = ENT_COMPAT, ?string $encoding = null, bool $double_encode = true): string
{
}
/* {{{ html.c */
/** @refcount 1 */
function htmlspecialchars(string $string, int $flags = ENT_QUOTES | ENT_SUBSTITUTE | ENT_HTML401, ?string $encoding = null, bool $double_encode = true): string
{
}