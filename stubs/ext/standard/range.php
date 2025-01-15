<?php 

/**
 * @param string|int|float $start
 * @param string|int|float $end
 */
#[\Until('8.3')]
function range($start, $end, int|float $step = 1): array
{
}
#[\Since('8.3')]
function range(string|int|float $start, string|int|float $end, int|float $step = 1): array
{
}