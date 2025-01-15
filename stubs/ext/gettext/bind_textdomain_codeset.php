<?php 

#endif
#ifdef HAVE_BIND_TEXTDOMAIN_CODESET
#[\Until('8.4')]
function bind_textdomain_codeset(string $domain, ?string $codeset): string|false
{
}
#endif
#ifdef HAVE_BIND_TEXTDOMAIN_CODESET
/** @refcount 1 */
#[\Since('8.4')]
function bind_textdomain_codeset(string $domain, ?string $codeset = null): string|false
{
}