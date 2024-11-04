package main
import "core:fmt"
import "core:strings"
import "core:strconv"

d1run :: proc (p1, p2: ^strings.Builder) {
    when 0 == 1 { input :: #load("../data/day1.example") }
    else { input :: #load("../data/day1.input") }

    total := 0;
    p2total := 0;
    lines := strings.split_lines(string(input));
    for l in lines[0:len(lines)-1] {
        val := (strconv.atoi(l) / 3) - 2;
        //fmt.printfln("[P1]%s -> %v", l, val);

        total += val;
        for (val > 0) {
            p2total += val;
            val = (val / 3) - 2;
        }
        //fmt.printfln("[P2]%s -> %v", l, p2total);
    }

    strings.write_int(p1, total);
    strings.write_int(p2, p2total);

    defer delete(lines);
}
