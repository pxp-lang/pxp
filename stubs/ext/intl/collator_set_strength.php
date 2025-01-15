<?php 

#[\Until('8.4')]
function collator_set_strength(\Collator $object, int $strength): bool
{
}
#[\Since('8.4')]
function collator_set_strength(\Collator $object, int $strength): true
{
}