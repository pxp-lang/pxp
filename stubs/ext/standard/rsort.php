<?php 

#[\Until('8.3')]
function rsort(array &$array, int $flags = SORT_REGULAR): bool
{
}
#[\Since('8.3')]
function rsort(array &$array, int $flags = SORT_REGULAR): true
{
}