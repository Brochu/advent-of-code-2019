package main
import "core:fmt"
import "core:strings"

DAY :: #config(DAY, 0);

import d "day0"

main :: proc() {
    fmt.printfln("Hey -> %d", DAY);

    //TODO: Look into different allocators within the implicit context
    //context.allocator
    b1 := strings.builder_make(0, 1024);
    b2 := strings.builder_make(0, 1024);

    d.part1(&b1);
    d.part2(&b2);

    fmt.printfln("Part 1: %s ; Part 2: %s", strings.to_string(b1), strings.to_string(b2));
    strings.builder_destroy(&b1);
    strings.builder_destroy(&b2);
}
