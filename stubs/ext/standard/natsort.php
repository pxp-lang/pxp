<?php 

#[\Until('8.3')]
function natsort(array &$array): bool
{
}
#[\Since('8.3')]
function natsort(array &$array): true
{
}