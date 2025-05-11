#![deny(unsafe_code)]
#![no_main]
#![no_std]

const TICKS_PER_SEC: u32 = 400;
const THRESHOLD: f32 = 1.5;

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use embedded_hal::digital::InputPin;
use embedded_hal::digital::OutputPin;
use embedded_hal::delay::DelayNs;

use microbit::{
    hal::{twim, Timer, gpio::{Level}},
    pac::twim0::frequency::FREQUENCY_A,
    //display::nonblocking::Display,
    display::blocking::Display,
};

use lsm303agr::{AccelMode, AccelOutputDataRate, AccelScale, Lsm303agr};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    //let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut delay = Timer::new(board.TIMER0);
    //let mut display = Display::new(board.TIMER0, board.display_pins);
    let mut display = Display::new(board.display_pins);
    /*let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor
        .set_accel_mode_and_odr(&mut delay, AccelMode::Normal, AccelOutputDataRate::Hz400)
        .unwrap();
    // Allow the sensor to measure up to 16 G since human punches
    // can actually be quite fast
    sensor.set_accel_scale(AccelScale::G16).unwrap();

    let mut max_g = 0.;
    let mut countdown_ticks = None;*/

    // When the board is NOT falling (IMU >= 0.5g),
    // then the board has the center LED on only:
    let mut no_falling: [[u8; 5]; 5] = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 255, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    // The exclamation point display when the board is falling.
    // (IMU < 0.5g):
    let mut falling: [[u8; 5]; 5] = [
        [0, 0, 255, 0, 0],
        [0, 0, 255, 0, 0],
        [0, 0, 255, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 255, 0, 0],
    ];

    let mut buttons = board.buttons; // Assign the board's buttons to var buttons.
    let mut A = buttons.button_a;
    let mut B = buttons.button_b;

    let mut drop: bool = false;

    let mut speaker = board.speaker_pin.into_push_pull_output(Level::Low);

    loop {

        if A.is_low().unwrap() {
            drop = true;
        }
        if B.is_low().unwrap() {
            drop = false;
        }

        if drop {
            //display.show(&falling);
            display.show(
                &mut delay, /* image_data: */ falling, /* ms: */ 100,
            ); // Source: https://docs.rs/microbit-v2/latest/microbit/display/blocking/index.html

            speaker.set_high().unwrap();
            delay.delay_us(500u32);
        } else {
            //display.show(&no_falling);
            display.show(&mut delay, /* image_data: */ no_falling, /* ms: */ 100,);

            speaker.set_low().unwrap();
            delay.delay_us(500u32);
        }

        /*
        while !sensor.accel_status().unwrap().xyz_new_data() {
            nop();
        }
        // x acceleration in g
        let (x, _, _) = sensor.acceleration().unwrap().xyz_mg();
        let g_x = x as f32 / 1000.0;

        if let Some(ticks) = countdown_ticks {
            if ticks > 0 {
                // countdown isn't done yet
                if g_x > max_g {
                    max_g = g_x;
                }
                countdown_ticks = Some(ticks - 1);
            } else {
                // Countdown is done: report max value
                rprintln!("Max acceleration: {}g", max_g);

                // Reset
                max_g = 0.;
                countdown_ticks = None;
            }
        } else {
            // If acceleration goes above a threshold, we start measuring
            if g_x > THRESHOLD {
                rprintln!("START!");

                max_g = g_x;
                countdown_ticks = Some(TICKS_PER_SEC);
            }
        }
        */
    }
}




/*#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use microbit::hal::Delay;

// Randomness source: https://blog.orhun.dev/zero-deps-random-in-rust/#:~:text=Using%20rand,is%20using%20the%20rand%20crate.&text=Generates%20random%20numbers%20with%20help,thread%20has%20an%20initialized%20generator.
use nanorand::wyrand::WyRand; // Source: ChatGPT
use nanorand::Rng;

use embedded_hal::digital::InputPin;
#[rustfmt::skip]
use microbit::{
    board::Board,
    display::blocking::Display,
};

#[entry]
// Source from the template: (template mb2): https://github.com/pdx-cs-rust-embedded/mb2-template
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut display = Display::new(board.display_pins);
    let mut delay = Delay::new(board.SYST);

    let mut buttons = board.buttons; // Assign the board's buttons to var buttons.

    // Create a random value with a seed:
    let mut rng = WyRand::new_seed(35435);

    // 1) Initialize the board with all zeros:
    let mut fb: [[u8; 5]; 5] = [[0; 5]; 5];

    loop {
        // When the A button is pressed, randomize the board:
        if buttons.button_a.is_low().unwrap() {
            rprintln!("Button A pressed! Board Randomized.");
            // When A is pushed, generate a new random board each frame.
            for y in 0..5 {
                for x in 0..5 {
                    fb[y][x] = rng.generate_range(0..=1);
                }
            }
        }
        
        // Convert fb (u8s) to bools for the display:
        let mut leds = [[false; 5]; 5];
        for row in 0..5 {
            for col in 0..5 {
                leds[row][col] = fb[row][col] == 1;
            }
        }

        // Show the frame for 100 ms:
        let mut leds_u8 = [[0u8; 5]; 5];
        for row in 0..5 {
            for col in 0..5 {
                leds_u8[row][col] = if leds[row][col] { 255 } else { 0 };
            }
        }

        // Run the game at 10 frames per second (update once per 100ms):
        // display.show(&mut delay, image_data, duration_ms);
        display.show(
            &mut delay, /* image_data: */ leds_u8, /* ms: */ 100,
        ); // Source: https://docs.rs/microbit-v2/latest/microbit/display/blocking/index.html
    }
}
*/

