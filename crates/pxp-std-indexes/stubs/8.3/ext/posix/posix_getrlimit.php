<?php

#ifdef HAVE_GETRLIMIT
/**
 * @refcount 1
 */
function posix_getrlimit(): array|false
{
}
#ifdef HAVE_GETRLIMIT
/**
 * @return array<int|string, int|string>|false
 * @refcount 1
 */
function posix_getrlimit(?int $resource = null): array|false
{
}