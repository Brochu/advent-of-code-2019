package main
import "core:fmt"
import "core:strings"

d0run :: proc (p1, p2: ^strings.Builder) {
    // Parse input

    part1(p1);
    part2(p2);
    
    // Cleanup
}

@(private="file")
part1 :: proc (out: ^strings.Builder) {
    strings.write_f64(out, 0.5, 'f');
}

@(private="file")
part2 :: proc (out: ^strings.Builder) {
    strings.write_f64(out, 0.25, 'f');
}
