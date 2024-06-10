<?php

class A {}
class B extends A {}
class C extends B implements I {}

class Properties {
    public $a;
    public string $b;
    protected int $c;
    private bool $d;
    public static $e;
    var $f;
}

class Methods {
    public function a() {}
    public function b(): string {}
    protected function c(): int {}
    private function d(): bool {}
    public static function e() {}
}