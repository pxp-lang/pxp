<?php 

#[\Since('8.4')]
enum PropertyHookType : string
{
    case Get = 'get';
    case Set = 'set';
}