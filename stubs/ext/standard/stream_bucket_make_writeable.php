<?php 

/* user_filters.c */
/** @param resource $brigade */
#[\Until('8.4')]
function stream_bucket_make_writeable($brigade): ?object
{
}
/* user_filters.c */
/**
 * @param resource $brigade
 * @refcount 1
 */
#[\Since('8.4')]
function stream_bucket_make_writeable($brigade): ?\StreamBucket
{
}