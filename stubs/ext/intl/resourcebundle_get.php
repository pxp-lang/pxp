<?php 

/** @param string|int $index */
#[\Until('8.4')]
function resourcebundle_get(\ResourceBundle $bundle, $index, bool $fallback = true): mixed
{
}
#[\Since('8.4')]
function resourcebundle_get(\ResourceBundle $bundle, string|int $index, bool $fallback = true): \ResourceBundle|array|string|int|null
{
}