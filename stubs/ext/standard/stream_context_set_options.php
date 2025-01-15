<?php 

/** @param resource $context */
#[\Since('8.3')]
#[\Until('8.4')]
function stream_context_set_options($context, array $options): bool
{
}
/** @param resource $context */
#[\Since('8.4')]
function stream_context_set_options($context, array $options): true
{
}