<?php

/* syslog.c */
#ifdef HAVE_SYSLOG_H
function openlog(string $prefix, int $flags, int $facility): bool
{
}