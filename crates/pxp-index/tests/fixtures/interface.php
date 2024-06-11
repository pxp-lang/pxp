<?php

interface I {}
interface J {}
interface K extends I, J {}

interface L {
    const A = 1;
    
    public function a();
    public function b(): string;
    public static function c();
}