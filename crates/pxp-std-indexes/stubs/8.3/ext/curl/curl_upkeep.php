<?php

#if LIBCURL_VERSION_NUM >= 0x073E00 /* Available since 7.62.0 */
function curl_upkeep(\CurlHandle $handle): bool
{
}