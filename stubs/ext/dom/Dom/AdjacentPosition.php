<?php 

namespace Dom;

#[\Since('8.4')]
enum AdjacentPosition : string
{
    case BeforeBegin = "beforebegin";
    case AfterBegin = "afterbegin";
    case BeforeEnd = "beforeend";
    case AfterEnd = "afterend";
}