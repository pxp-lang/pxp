<?php 

#[\Until('8.3')]
function mysqli_field_seek(\mysqli_result $result, int $index): bool
{
}
#[\Since('8.3')]
function mysqli_field_seek(\mysqli_result $result, int $index): true
{
}