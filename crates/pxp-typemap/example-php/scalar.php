<?php

dumpType(1);
dumpType(1.0);
dumpType(true);
dumpType(false);
dumpType("hello");
dumpType('hello');
dumpType(<<<EOT
Hello
EOT);
dumpType(<<<'EOT'
Hello
EOT);
dumpType(null);