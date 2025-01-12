#![no_std]
#![no_main]

mod keys;
mod oc;

use defmt as _;
use defmt::{error, info};
use defmt_rtt as _;
use panic_probe as _;
use rtic_monotonics::rp2040_timer_monotonic;

rp2040_timer_monotonic!(Mono);

pub const PINS_IN_MASK: u32 = 0b1111_1111_1111_1111;
pub const PINS_OUT_SHIFT: u8 = 16;

#[rtic::app(
    device = rp_pico::hal::pac, dispatchers = [TIMER_IRQ_1]
)]
mod app {
    use super::*;
    use core::sync::atomic::{AtomicU32, Ordering};
    use hal::gpio::PinState;
    use rp_pico::hal::gpio::PullNone;
    use rp_pico::hal::{self, watchdog::Watchdog};
    use rp_pico::XOSC_CRYSTAL_FREQ;
    use usbh::{
        driver::kbd::{KbdDriver, KbdEvent},
        PollResult, UsbHost,
    };
    use usbh_rp2040::UsbHostBus;

    // Shared resources go here
    #[shared]
    struct Shared {
        col_enabled_pins: [AtomicU32; 4],
    }

    // Local resources go here
    #[local]
    struct Local {
        usb_host: UsbHost<UsbHostBus>,
        kbd_driver: KbdDriver,
        sio: rp_pico::hal::pac::SIO,
    }

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local) {
        unsafe { hal::sio::spinlock_reset() };

        // Setup the clock. This is required.
        let mut watchdog = Watchdog::new(ctx.device.WATCHDOG);
        let clocks = oc::init_clocks_and_plls(
            XOSC_CRYSTAL_FREQ,
            ctx.device.XOSC,
            ctx.device.CLOCKS,
            ctx.device.PLL_SYS,
            ctx.device.PLL_USB,
            &mut ctx.device.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();
        Mono::start(ctx.device.TIMER, &ctx.device.RESETS); // default rp2040 clock-rate is 125MHz

        let sio = hal::Sio::new(ctx.device.SIO);

        let pins = hal::gpio::Pins::new(
            ctx.device.IO_BANK0,
            ctx.device.PADS_BANK0,
            sio.gpio_bank0,
            &mut ctx.device.RESETS,
        );

        // config inputs
        pins.gpio0.into_pull_up_input();
        pins.gpio1.into_pull_up_input();
        pins.gpio2.into_pull_up_input();
        pins.gpio3.into_pull_up_input();
        pins.gpio4.into_pull_up_input();
        pins.gpio5.into_pull_up_input();
        pins.gpio6.into_pull_up_input();
        pins.gpio7.into_pull_up_input();
        pins.gpio8.into_pull_up_input();
        pins.gpio9.into_pull_up_input();
        pins.gpio10.into_pull_up_input();
        pins.gpio11.into_pull_up_input();
        pins.gpio12.into_pull_up_input();
        pins.gpio13.into_pull_up_input();
        pins.gpio14.into_pull_up_input();
        pins.gpio15.into_pull_up_input();

        // config outputs. outputs should pull low, or float
        // the CBM II has pull-up resistors on these already
        pins.gpio16
            .into_pull_type::<PullNone>()
            .into_push_pull_output_in_state(PinState::High);
        pins.gpio17
            .into_pull_type::<PullNone>()
            .into_push_pull_output_in_state(PinState::High);
        pins.gpio18
            .into_pull_type::<PullNone>()
            .into_push_pull_output_in_state(PinState::High);
        pins.gpio19
            .into_pull_type::<PullNone>()
            .into_push_pull_output_in_state(PinState::High);
        pins.gpio20
            .into_pull_type::<PullNone>()
            .into_push_pull_output_in_state(PinState::High);
        pins.gpio21
            .into_pull_type::<PullNone>()
            .into_push_pull_output_in_state(PinState::High);

        // config unused gpios
        pins.gpio22.into_pull_down_disabled();
        pins.gpio23.into_pull_down_disabled();
        pins.gpio24.into_pull_down_disabled();
        pins.gpio25.into_pull_down_disabled();
        pins.gpio26.into_pull_down_disabled();
        pins.gpio27.into_pull_down_disabled();
        pins.gpio28.into_pull_down_disabled();
        pins.gpio29.into_pull_down_disabled();

        let usb_host = UsbHost::new(usbh_rp2040::UsbHostBus::new(
            ctx.device.USBCTRL_REGS,
            ctx.device.USBCTRL_DPRAM,
            clocks.usb_clock,
            &mut ctx.device.RESETS,
        ));
        (
            Shared {
                col_enabled_pins: [const { AtomicU32::new(0) }; 4],
            },
            Local {
                usb_host,
                kbd_driver: KbdDriver::new(),
                sio: unsafe { rp_pico::hal::pac::SIO::steal() },
            },
        )
    }

    #[idle(
        local = [sio],
        shared = [&col_enabled_pins]
    )]
    fn idle(ctx: idle::Context) -> ! {
        // 4 bits to 0xFF in those byte positions
        let lookup: [u32; 16] = [
            0x00000000, 0x000000FF, 0x0000FF00, 0x0000FFFF, 0x00FF0000, 0x00FF00FF, 0x00FFFF00,
            0x00FFFFFF, 0xFF000000, 0xFF0000FF, 0xFF00FF00, 0xFF00FFFF, 0xFFFF0000, 0xFFFF00FF,
            0xFFFFFF00, 0xFFFFFFFF,
        ];

        loop {
            // masking not needed, only checking the low bits, as the bit index matches the row
            // index
            let cols_in = !ctx.local.sio.gpio_in().read().bits() as u16; // & PINS_IN_MASK

            let mask = [
                lookup[(cols_in & 0b1111) as usize],
                lookup[((cols_in >> 4) & 0b1111) as usize],
                lookup[((cols_in >> 8) & 0b1111) as usize],
                lookup[((cols_in >> 12) & 0b1111) as usize],
            ];

            let out = (ctx.shared.col_enabled_pins[0].load(Ordering::Relaxed) & mask[0])
                | (ctx.shared.col_enabled_pins[1].load(Ordering::Relaxed) & mask[1])
                | (ctx.shared.col_enabled_pins[2].load(Ordering::Relaxed) & mask[2])
                | (ctx.shared.col_enabled_pins[3].load(Ordering::Relaxed) & mask[3]);
            let [out0, out1, out2, out3] = out.to_ne_bytes();
            let out = !(out0 | out1 | out2 | out3);

            ctx.local
                .sio
                .gpio_out()
                .write(|w| unsafe { w.bits((out as u32) << PINS_OUT_SHIFT) });
        }
    }

    #[task(
        binds = USBCTRL_IRQ,
        local = [usb_host, kbd_driver],
        shared = [&col_enabled_pins]
    )]
    fn usbctrl_irq(ctx: usbctrl_irq::Context) {
        match ctx
            .local
            .usb_host
            .poll(&mut [ctx.local.kbd_driver /* as &mut dyn Driver<_> */])
        {
            PollResult::NoDevice => {
                return;
            }
            PollResult::Busy => {}
            PollResult::Idle => {}
            PollResult::BusError(error) => {
                error!("Bus error: {}", error);
            }
            PollResult::DiscoveryError(dev_addr) => {
                error!("Discovery for device {} failed", dev_addr);
            }
            _ => {}
        }

        match ctx.local.kbd_driver.take_event() {
            None => {}
            Some(event) => match event {
                KbdEvent::DeviceAdded(dev_addr) => {
                    info!("Keyboard with address {} added", dev_addr);
                    ctx.local
                        .kbd_driver
                        .set_idle(dev_addr, 0, ctx.local.usb_host)
                        .ok()
                        .unwrap();
                }
                KbdEvent::DeviceRemoved(dev_addr) => {
                    info!("Keyboard with address {} removed", dev_addr);
                }
                KbdEvent::InputChanged(_, report) => {
                    let mut col_gpio_bits = [0u32; 4];

                    let cast_to_bytes = bytemuck::cast_mut(&mut col_gpio_bits);

                    for key in report.pressed_keys() {
                        key_set(key, cast_to_bytes);
                    }
                    let modifier_status = report.modifier_status;
                    if modifier_status.left_shift() || modifier_status.right_shift() {
                        key_set(keys::KEY_LEFTSHIFT, cast_to_bytes);
                    }
                    if modifier_status.left_ctrl() || modifier_status.right_ctrl() {
                        key_set(keys::KEY_LEFTCTRL, cast_to_bytes);
                    }

                    for (storage, new) in ctx.shared.col_enabled_pins.iter().zip(col_gpio_bits) {
                        storage.store(new, Ordering::Relaxed);
                    }
                }
                _ => {}
            },
        }
    }
}

fn key_set(key: u8, col_gpio_bits: &mut [u8; 16]) {
    unsafe {
        let (col, row_bits) = *keys::INVERSE_KEYMAP.get_unchecked(key as usize);
        *col_gpio_bits.get_unchecked_mut(col as usize) |= row_bits;
    };
}
