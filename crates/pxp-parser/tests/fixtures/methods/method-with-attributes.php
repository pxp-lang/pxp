<?php

class A
{
    #[B]
    #[C, D("hello")]
    public function a() {}
}