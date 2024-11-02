package main

import "core:fmt"
import "core:mem"
import os "core:os/os2"
import "core:strconv"
import "core:strings"

day_proc :: proc(^strings.Builder, ^strings.Builder)
solutions: []day_proc = {
    d0run,
    d1run,
};

main :: proc() {
    day, valid := which_day();
    if (!valid) { return; }

    ptr, err := mem.alloc_bytes(1024);
    arena: mem.Arena;
    mem.arena_init(&arena, ptr);
    alloc := mem.arena_allocator(&arena);
    context.allocator = alloc;

    b1, b1err := strings.builder_make(0, 512);
    b2, b2err := strings.builder_make(0, 512);
    if (b1err != nil || b2err != nil) {
        fmt.println("[AoC19] Could not allocate result buffers");
        return;
    }
    defer strings.builder_destroy(&b1);
    defer strings.builder_destroy(&b2);

    fmt.printfln("[AoC19] Day %v", day);
    solutions[day](&b1, &b2);
    //TODO: Setup timer to check processing time

    fmt.printfln(" > Part 1 = %v", strings.to_string(b1));
    fmt.printfln(" > Part 2 = %v", strings.to_string(b2));
}

which_day :: proc() -> (int, bool) {
    fields : os.Process_Info_Fields;
    fields = { .Command_Line, .Command_Args };

    pinfo, _ := os.current_process_info(fields, context.allocator);
    defer os.free_process_info(pinfo, context.allocator);
    /*
    fmt.println("[AoC19] Process Info :");
    fmt.printfln(" - pid = %v", pinfo.pid);
    fmt.printfln(" - cmdargs = %v", pinfo.command_args);
    fmt.println("--------------------\n");
    */

    if (len(pinfo.command_args) < 2) {
        fmt.println("[AoC19] usage: aoc19.exe <daynum>");
        return 0, false;
    }
    day := strconv.atoi(pinfo.command_args[1]);
    return day, true;
}
