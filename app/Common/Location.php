<?php

namespace App\Common;

final class Location
{
    public function __construct(
        public readonly File $file,
        public readonly int $position,
    ) {}
}
