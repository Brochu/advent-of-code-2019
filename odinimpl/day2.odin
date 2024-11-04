package main
import "core:fmt"
import "core:slice"
import "core:strings"
import "core:strconv"

d2run :: proc (p1, p2: ^strings.Builder) {
    when 1 == 1 { input : []u8 : #load("../data/day2.example") }
    else { input : []u8 : #load("../data/day2.input") }
    nums := strings.split(string(input), ",");
    mem := make([]int, len(nums));

    for n, i in nums {
        mem[i] = strconv.atoi(nums[i]);
    }
    p1mem, err1 := slice.clone(mem);
    p2mem, err2 := slice.clone(mem);

    part1(p1mem, p1);
    part2(p2mem, p2);

    delete(mem);
    delete(p1mem);
    delete(p2mem);
    delete(nums);
}

@(private="file")
part1 :: proc (mem: []int, out: ^strings.Builder) {
    for i := 0; i < len(mem); i += 4 {
        if mem[i] == 99 { break; }
        //fmt.println(mem);

        op := mem[i + 0];
        a := mem[i + 1];
        b := mem[i + 2];
        target := mem[i + 3];
        //fmt.printfln("[%v] ->", i);
        //fmt.printfln(" -a: %v", a);
        //fmt.printfln(" -b: %v", b);
        //fmt.printfln(" -target: %v", target);

        if (mem[i] == 1) {
            mem[target] = mem[a] + mem[b];
        }
        else if (mem[i] == 2) {
            mem[target] = mem[a] * mem[b];
        }
        //fmt.println(mem);
    }

    strings.write_int(out, mem[0]);
}

@(private="file")
part2 :: proc (mem: []int, out: ^strings.Builder) {
    strings.write_string(out, "PART2");
}
