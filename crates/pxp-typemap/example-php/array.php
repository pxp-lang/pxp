<?php

dumpType([]);
dumpType([1, 2, 3]);
dumpType([1, true, "hello"]);

$items = [];

// FIXME: Make variables work.
dumpType($items);

$items[] = 1;

dumpType($items);