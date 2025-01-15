<?php 

/** @param resource $brigade */
#[\Until('8.4')]
function stream_bucket_append($brigade, object $bucket): void
{
}
/** @param resource $brigade */
#[\Since('8.4')]
function stream_bucket_append($brigade, \StreamBucket $bucket): void
{
}