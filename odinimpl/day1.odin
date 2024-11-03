package main
import "core:fmt"
import "core:strings"
import "core:strconv"

DEBUG :: true

when DEBUG {
    input :: #load("../data/day1.example")
} else {
    input :: #load("../data/day1.input")
}

d1run :: proc (p1, p2: ^strings.Builder) {
    total := 0;
    lines := strings.split_lines(string(input));
    for l in lines[0:len(lines)-1] {
        val := (strconv.atoi(l) / 3) - 2;
        total += val;
    }

    strings.write_int(p1, total);
    part2(p2);

    // Cleanup
}

@(private="file")
part2 :: proc (out: ^strings.Builder) {
    strings.write_i64(out, -2);
}
