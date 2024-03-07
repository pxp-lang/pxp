<?php

<<<'REGEX'
/<<<[ \t]*(['"]?)([a-zA-Z_\x80-\xff][a-zA-Z0-9_\x80-\xff]*)\1\r?\n
(?:.*\r?\n)*?
(?<indentation>\h*)\2(?![a-zA-Z0-9_\x80-\xff])(?<separator>(?:;?[\r\n])?)/x
REGEX;