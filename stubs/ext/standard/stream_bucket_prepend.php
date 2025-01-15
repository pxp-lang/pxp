<?php 

/** @param resource $brigade */
#[\Until('8.4')]
function stream_bucket_prepend($brigade, object $bucket): void
{
}
/** @param resource $brigade */
#[\Since('8.4')]
function stream_bucket_prepend($brigade, \StreamBucket $bucket): void
{
}