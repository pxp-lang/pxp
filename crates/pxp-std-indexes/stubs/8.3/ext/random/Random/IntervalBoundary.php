<?php

namespace Random;

enum IntervalBoundary
{
    case ClosedOpen;
    case ClosedClosed;
    case OpenClosed;
    case OpenOpen;
}