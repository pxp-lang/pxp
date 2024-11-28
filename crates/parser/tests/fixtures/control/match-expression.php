<?php

match ($foo) {
    1 => 'one',
    2, 3 => 'two or three',
    default => 'other',
};