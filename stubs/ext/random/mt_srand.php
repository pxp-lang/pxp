<?php 

#[\Since('8.2')]
#[\Until('8.3')]
function mt_srand(int $seed = UNKNOWN, int $mode = MT_RAND_MT19937): void
{
}
#[\Since('8.3')]
function mt_srand(?int $seed = null, int $mode = MT_RAND_MT19937): void
{
}