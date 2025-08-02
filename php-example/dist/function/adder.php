<?php
declare(strict_types=1);

namespace MyApp\Function;

/**
 * @return int
 */
function autoAdder() {
    /** @var int $a */
    $a = 5;
    /** @var int $b */
    $b = 10;
    return $a + $b;
}
/**
 * @param int $a
 * @param int $b
 * @return int
 */
function adder($a, $b) {
    $result = $a + $b;
    return $result;
}
