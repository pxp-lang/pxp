<?php

use Pxp\Runtime\Range\ExclusiveRange;

it('can be created', function () {
    expect(ExclusiveRange::make(1, 5))->toBeInstanceOf(ExclusiveRange::class);
});

it('returns the first item', function () {
    expect(ExclusiveRange::make(1, 5))->first()->toBe(1);
});

it('is marked as exclusive', function () {
    expect(ExclusiveRange::make(1, 5)->exclusive())->toBeTrue();
});

it('returns the last item', function () {
    expect(ExclusiveRange::make(1, 5)->last())->toBe(4);
});

it('returns the correct count', function () {
    expect(ExclusiveRange::make(1, 5)->count())->toBe(4);
});

it('can be iterated over', function () {
    $range = ExclusiveRange::make(1, 5);
    $collected = [];

    foreach ($range as $j) {
        $collected[] = $j;
    }

    expect($collected)->toHaveLength(4)->toBe([
        1, 2, 3, 4,
    ]);
});

it('can change step', function () {
    $range = ExclusiveRange::make(1, 5)->step(2);
    $collected = [];

    foreach ($range as $j) {
        $collected[] = $j;
    }

    expect($collected)->toHaveLength(2)->toBe([1, 3]);
});