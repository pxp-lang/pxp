<?php

trait TraitA {}
trait TraitB {}
trait TraitC {
    use TraitA, TraitB;
}

trait TraitD {
    const A = 1;
    
    public $a;

    public function a() {}
    public function b(): string {}
    protected static function c() {}
    abstract private function d();
}