<?php

final class mysqli_sql_exception extends \RuntimeException
{
    public function getSqlState(): string
    {
    }
}