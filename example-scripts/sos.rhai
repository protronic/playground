// SOS LED blink (Morse: ... --- ...)
// wait() uses a busy-loop on ts() to simulate a hardware delay.
// This is for demonstration only — it will consume 100% CPU while waiting.
fn wait(ms) {
    let t = ms * 32;
    let s = ts();
    while ts() - s < t {}
}

fn dot() {
    led(true);
    wait(200);
    led(false);
    wait(200);
}

fn dash() {
    led(true);
    wait(600);
    led(false);
    wait(200);
}

fn letter_gap() {
    wait(400); // 200ms already elapsed after last symbol → total 600ms gap
}

// S (...)
dot(); dot(); dot();
letter_gap();

// O (---)
dash(); dash(); dash();
letter_gap();

// S (...)
dot(); dot(); dot();
