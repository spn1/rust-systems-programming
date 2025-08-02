mod particle;
mod world;

use world::World;
use piston_window::*; // Create GUI and draw shapes

use std::alloc::{GlobalAlloc, System, Layout};
use std::time::Instant; // system clock access

#[global_allocator]
static ALLOCATOR: ReportingAllocator = ReportingAllocator;

struct ReportingAllocator;

// Reports the time it takes to allocate memory on the heap compared to the size allocated
unsafe impl GlobalAlloc for ReportingAllocator {
    unsafe fn alloc (&self, layout: Layout) -> *mut u8 {
        let start = Instant::now();
        let ptr  = System.alloc(layout);
        let end = Instant::now();
        let time_taken = end - start;
        let bytes_requested = layout.size();

        eprintln!("{}\t{}", bytes_requested, time_taken.as_nanos());
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
}

pub fn run() {
    let (width, height) = (1280.0, 960.0);
    let mut window: PistonWindow = WindowSettings::new(
        "particles", [width, height]
    )
    .exit_on_esc(true)
    .build()
    .expect("Could not create window");

    let mut world = World::new(width, height);
    world.add_shapes(1000);

    while let Some(event) = window.next() {
        world.update();

        window.draw_2d(&event, |ctx, renderer, _device| {
            clear([0.15, 0.17, 0.17, 0.9], renderer);

            for s in &mut world.particles {
                let size = [s.position[0], s.position[1], s.width, s.height];
                rectangle(s.color, size, ctx.transform, renderer);
            }
        });

    }
}