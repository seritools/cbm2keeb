#![allow(unused)]

use rp_pico::hal::{
    clocks::{ClocksManager, InitError},
    pac,
    pll::{
        common_configs::{PLL_SYS_125MHZ, PLL_USB_48MHZ},
        setup_pll_blocking, PLLConfig,
    },
    xosc::setup_xosc_blocking,
    Watchdog,
};
use rtic_monotonics::fugit::{HertzU32, RateExtU32};

/// Initialize the clocks and plls according to the reference implementation
pub fn init_clocks_and_plls(
    xosc_crystal_freq: u32,
    xosc_dev: pac::XOSC,
    clocks_dev: pac::CLOCKS,
    pll_sys_dev: pac::PLL_SYS,
    pll_usb_dev: pac::PLL_USB,
    resets: &mut pac::RESETS,
    watchdog: &mut Watchdog,
) -> Result<ClocksManager, InitError> {
    let xosc = setup_xosc_blocking(xosc_dev, xosc_crystal_freq.Hz()).map_err(InitError::XoscErr)?;

    // Configure watchdog tick generation to tick over every microsecond
    watchdog.enable_tick_generation((xosc_crystal_freq / 1_000_000) as u8);

    let mut clocks = ClocksManager::new(clocks_dev);

    // Run at 150MHz
    const PLL_SYS_OC_150: PLLConfig = PLLConfig {
        vco_freq: HertzU32::MHz(1500),
        refdiv: 1,
        post_div1: 5,
        post_div2: 2,
    };

    // Run at 300MHz
    const PLL_SYS_OC_300: PLLConfig = PLLConfig {
        vco_freq: HertzU32::MHz(1500),
        refdiv: 1,
        post_div1: 5,
        post_div2: 1,
    };

    // M3 at 150MHz, M3 50% faster than M0+ → 225MHz equivalent
    const PLL_SYS_OC_CORTEX_M3_EQUIV: PLLConfig = PLLConfig {
        vco_freq: HertzU32::MHz(900),
        refdiv: 1,
        post_div1: 4,
        post_div2: 1,
    };

    let pll_sys = setup_pll_blocking(
        pll_sys_dev,
        xosc.operating_frequency(),
        PLL_SYS_OC_CORTEX_M3_EQUIV,
        &mut clocks,
        resets,
    )
    .map_err(InitError::PllError)?;
    let pll_usb = setup_pll_blocking(
        pll_usb_dev,
        xosc.operating_frequency(),
        PLL_USB_48MHZ,
        &mut clocks,
        resets,
    )
    .map_err(InitError::PllError)?;

    clocks
        .init_default(&xosc, &pll_sys, &pll_usb)
        .map_err(InitError::ClockError)?;

    Ok(clocks)
}
