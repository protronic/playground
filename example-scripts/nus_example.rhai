// WebBluetooth NUS (Nordic UART Service) Example
//
// This script demonstrates how to communicate with a BLE device that exposes
// the Nordic UART Service (NUS).
//
// Steps:
//   1. Click "Connect BLE (NUS)" in the toolbar to pair a device, OR
//      call nus_connect() below to trigger the device picker from the script.
//   2. Click "Run" (make sure "Run script using Web Worker" is OFF in Config).
//
// Available NUS functions:
//   nus_connect()           -- trigger BLE device picker (fire-and-forget)
//   nus_disconnect()        -- disconnect the current device
//   nus_send(text)          -- send a UTF-8 string to the device
//   nus_receive()           -- poll for the next received string ("" if none)
//   nus_is_connected()      -- returns true when a device is connected

// Check whether a device is already connected via the toolbar button.
if !nus_is_connected() {
    print("No BLE device connected. Attempting to connect...");
    nus_connect();
    // nus_connect() fires asynchronously; in a real script you would
    // wait and check nus_is_connected() after some time.
    print("Device picker opened. Connect a device and re-run the script.");
} else {
    print("BLE device connected: sending command...");

    // Send a command to the device.
    nus_send("hello\n");
    print("Sent: hello");

    // Poll for a response up to 100 times. Each poll is instantaneous (no
    // sleep), so this drains any data already buffered by BLE notifications
    // at the time the script runs. Increase the limit if the device sends
    // many small notification chunks.
    let response = "";
    let attempts = 0;
    while response == "" && attempts < 100 {
        response = nus_receive();
        attempts += 1;
    }

    if response != "" {
        print("Received: " + response);
    } else {
        print("No response received within polling window.");
    }

    print("Done.");
}
