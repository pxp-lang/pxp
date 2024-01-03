<?php

$foo = true;

dumpType($foo ?: 'bar');
dumpType($foo ? [1, 2] : []);