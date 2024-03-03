<?php

/** @param resource $stream */
function fputcsv($stream, array $fields, string $separator = ",", string $enclosure = "\"", string $escape = "\\"): int|false
{
}
/** @param resource $stream */
function fputcsv($stream, array $fields, string $separator = ",", string $enclosure = "\"", string $escape = "\\", string $eol = "\n"): int|false
{
}