<?php 

/** @param resource $context */
#[\Until('8.4')]
function stream_context_set_params($context, array $params): bool
{
}
/** @param resource $context */
#[\Since('8.4')]
function stream_context_set_params($context, array $params): true
{
}