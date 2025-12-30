#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_net::Stack;
use esp_println::println;
use core::panic::PanicInfo;
use esp_hal::{
    clock::CpuClock,
    interrupt::{software::SoftwareInterruptControl, Priority},
};
use esp_hal::timer::timg::TimerGroup;
use static_cell::StaticCell;
use esp_rtos::embassy::InterruptExecutor;
use embassy_time::{Duration, Timer};

mod http;
mod wifi;

esp_bootloader_esp_idf::esp_app_desc!();

#[embassy_executor::task]
async fn interrupt_task() {
    loop {
        Timer::after(Duration::from_millis(1)).await;
    }
}

#[embassy_executor::task]
async fn run_http_server(stack: Stack<'static>) {
    let mut http_server = http::HttpServer::new(stack, 3502);
    println!("Starting HTTP server...");
    if let Err(e) = http_server.run().await {
        println!("HTTP server error: {}", e);
    }
}


#[esp_rtos::main]
async fn main(low_priority_spawner: Spawner) {
    println!("Starting WiFi example");

    // esp_println::logger::init_logger_from_env();
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let timg1 = TimerGroup::new(peripherals.TIMG1);
    let sw_ints = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg1.timer0, sw_ints.software_interrupt0);
    println!("Hardware initialized.");

    esp_alloc::heap_allocator!(size: 72 * 1024);
    
    println!("Initializing WiFi...");
    let stack = wifi::get_wifi(low_priority_spawner, peripherals.WIFI).await;
    println!("WiFi initialized.");

    static EXECUTOR: StaticCell<InterruptExecutor<2>> = StaticCell::new();
    let executor = InterruptExecutor::new(sw_ints.software_interrupt2);
    let executor = EXECUTOR.init(executor);

    let spawner = executor.start(Priority::Priority3);
    // The interrupt executor can interrupt uncooperative tasks
    spawner.must_spawn(interrupt_task());
    low_priority_spawner.must_spawn(run_http_server(stack));

}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("");
    println!("====================== PANIC ======================");
    println!("{}", info);
    loop {}
}
