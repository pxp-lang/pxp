<?php

use Pxp\Runtime\Range\EndlessRange;
use Pxp\Runtime\Range\Exceptions\InvalidRangeOperationException;

it('can be created', function () {
    expect(EndlessRange::make(1))->toBeInstanceOf(EndlessRange::class);
});

it('returns the first item', function () {
    expect(EndlessRange::make(1)->first())->toBe(1);
});

it('is marked as endless', function () {
    expect(EndlessRange::make(1)->endless())->toBeTrue();
});

it('returns INF as last and max', function () {
    expect(EndlessRange::make(1))
        ->last()
        ->toBe(INF)
        ->max()
        ->toBe(INF);
});

it('throws when trying to count', function () {
    EndlessRange::make(1)->count();
    count(EndlessRange::make(1));
})->throws(InvalidRangeOperationException::class, "Cannot call `count()` on an endless range.");

it('can be iterated over', function () {
    $range = EndlessRange::make(1);
    $collected = [];

    foreach ($range as $i => $j) {
        if ($i >= 10) {
            break;
        }

        $collected[] = $j;
    }

    expect($collected)->toHaveLength(10)->toBe([
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10
    ]);
});

it('can change step', function () {
    $range = EndlessRange::make(1)->step(2);
    $collected = [];

    foreach ($range as $i => $j) {
        if ($i > 2) {
            break;
        }

        $collected[] = $j;
    }

    expect($collected)->toHaveLength(3)->toBe([1, 3, 5]);
});