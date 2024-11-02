package main
import "core:fmt"
import "core:strings"

d1run :: proc (p1, p2: ^strings.Builder) {
    strings.write_u64(p1, 69, 10);
    strings.write_u64(p2, 70, 10);
}
