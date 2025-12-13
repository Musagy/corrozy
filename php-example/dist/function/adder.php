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
    /** @var int $int1 */
    $int1 = $a;
    /** @var int $int2 */
    $int2 = $b;
    /**
     * @return int
     */
    function idk() use ($int1, $int2) {
        $result = $int1 + $int2;
        return $result * 2;
    }
    return idk();
}

echo adder(5, 10);