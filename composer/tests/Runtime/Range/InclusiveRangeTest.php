<?php

use Pxp\Runtime\Range\InclusiveRange;

it('can be created', function () {
    expect(InclusiveRange::make(1, 5))->toBeInstanceOf(InclusiveRange::class);
});

it('returns the first item', function () {
    expect(InclusiveRange::make(1, 5))->first()->toBe(1);
});

it('is marked as inclusive', function () {
    expect(InclusiveRange::make(1, 5)->inclusive())->toBeTrue();
});

it('returns the last item', function () {
    expect(InclusiveRange::make(1, 5)->last())->toBe(5);
});

it('returns the correct count', function () {
    expect(InclusiveRange::make(1, 5)->count())->toBe(5);
});

it('can be iterated over', function () {
    $range = InclusiveRange::make(1, 5);
    $collected = [];

    foreach ($range as $j) {
        $collected[] = $j;
    }

    expect($collected)->toHaveLength(5)->toBe([
        1, 2, 3, 4, 5,
    ]);
});

it('can change step', function () {
    $range = InclusiveRange::make(1, 5)->step(2);
    $collected = [];

    foreach ($range as $j) {
        $collected[] = $j;
    }

    expect($collected)->toHaveLength(3)->toBe([1, 3, 5]);
});