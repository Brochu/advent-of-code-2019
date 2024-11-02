package main
import "core:fmt"
import "core:strings"

d1run :: proc (p1, p2: ^strings.Builder) {
    // Parse input

    part1(p1);
    part2(p2);
    
    // Cleanup
}

@(private="file")
part1 :: proc (out: ^strings.Builder) {
    strings.write_i64(out, -1);
}

@(private="file")
part2 :: proc (out: ^strings.Builder) {
    strings.write_i64(out, -2);
}
