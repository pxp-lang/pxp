<?php

namespace Random;

interface Engine
{
    public function generate(): string;
}