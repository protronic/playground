fn wait(ms){let t=ms*3276;let s=ts();while ts()-s<t{}}
fn dot(){led(true);wait(200);led(false);wait(200);}
fn dash(){led(true);wait(600);led(false);wait(200);}
dot();dot();dot();wait(400);
dash();dash();dash();wait(400);
dot();dot();dot();
