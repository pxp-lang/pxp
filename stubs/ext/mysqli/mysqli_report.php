<?php 

#[\Until('8.4')]
function mysqli_report(int $flags): bool
{
}
#[\Since('8.4')]
function mysqli_report(int $flags): true
{
}