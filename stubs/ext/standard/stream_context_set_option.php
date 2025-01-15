<?php 

/** @param resource $context */
#[\Until('8.4')]
function stream_context_set_option($context, array|string $wrapper_or_options, ?string $option_name = null, mixed $value = UNKNOWN): bool
{
}
/** @param resource $context */
#[\Since('8.4')]
function stream_context_set_option($context, array|string $wrapper_or_options, ?string $option_name = null, mixed $value = UNKNOWN): true
{
}