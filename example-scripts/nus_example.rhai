// NUS: disable Web Worker in Config first.
if !nus_is_connected() {
    nus_connect();
    print("Connect device, re-run.");
} else {
    nus_send("hello\n");
    let r=nus_receive();
    print(if r!="" {"Got: "+r} else {"No reply."});
}
