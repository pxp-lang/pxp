<?php 

#if defined(MYSQLI_USE_MYSQLND)
function mysqli_fetch_all(\mysqli_result $result, int $mode = MYSQLI_NUM): array
{
}