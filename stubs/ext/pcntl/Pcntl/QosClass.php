<?php 

namespace Pcntl;

#[\Since('8.4')]
enum QosClass
{
    case UserInteractive;
    case UserInitiated;
    case Default;
    case Utility;
    case Background;
}