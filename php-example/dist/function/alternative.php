<?php
declare(strict_types=1);

namespace MyApp\Function;

function common_function() {
    return 1 + 1;
}
function outer_function() {
    $inner_function = 
    inner_function();
}
