// think if it's really necessary to use static_alloc:
// use static_alloc::Bump;
//
// #[global_allocator]
// static A: Bump<[u8; 1 << 12]> = Bump::uninit(); // 4 kB
//

// TODO: prepare a release mode that coredumps into flash or smth and resets
#[alloc_error_handler]
fn on_oom(_layout: core::alloc::Layout) -> ! {
    // No memory left, fell free to take a look at the layout :)
    cortex_m::asm::bkpt();
    loop {}
}

