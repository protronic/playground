/**
 * WebBluetooth Nordic UART Service (NUS) implementation.
 *
 * NUS Service UUID : 6e400001-b5a3-f393-e0a9-e50e24dcca9e
 * RX Characteristic: 6e400002-b5a3-f393-e0a9-e50e24dcca9e  (browser → device)
 * TX Characteristic: 6e400003-b5a3-f393-e0a9-e50e24dcca9e  (device → browser, notifications)
 *
 * Connection state is kept as a module-level singleton so it persists across
 * Rhai script executions.
 */

const NUS_SERVICE_UUID  = '6e400001-b5a3-f393-e0a9-e50e24dcca9e';
const NUS_TX_CHAR_UUID  = '6e400003-b5a3-f393-e0a9-e50e24dcca9e'; // device → browser
const NUS_RX_CHAR_UUID  = '6e400002-b5a3-f393-e0a9-e50e24dcca9e'; // browser → device

/** @type {BluetoothDevice|null} */
let bleDevice = null;

/** @type {BluetoothRemoteGATTCharacteristic|null} */
let rxCharacteristic = null;

/** @type {string[]} */
let receiveBuffer = [];

/** @type {Array<function({isConnected: boolean, deviceName: string|null}): void>} */
let statusListeners = [];

function notifyStatusListeners() {
    const state = {
        isConnected: isConnected(),
        deviceName: bleDevice ? (bleDevice.name || null) : null,
    };
    for (const fn of statusListeners) {
        try { fn(state); } catch (_) { /* ignore listener errors */ }
    }
}

/**
 * Register a callback that is invoked whenever the NUS connection state changes.
 * Returns an unsubscribe function.
 * @param {function({isConnected: boolean, deviceName: string|null}): void} listener
 * @returns {function(): void}
 */
export function onStatusChange(listener) {
    statusListeners.push(listener);
    return () => {
        statusListeners = statusListeners.filter(fn => fn !== listener);
    };
}

/**
 * Returns true when a BLE device is currently connected via NUS.
 * @returns {boolean}
 */
export function isConnected() {
    return bleDevice !== null && bleDevice.gatt != null && bleDevice.gatt.connected;
}

/**
 * Returns the name of the connected device, or null if not connected.
 * @returns {string|null}
 */
export function deviceName() {
    return bleDevice && bleDevice.gatt != null && bleDevice.gatt.connected ? (bleDevice.name || null) : null;
}

/**
 * Returns true if WebBluetooth is available in this browser.
 * @returns {boolean}
 */
export function isSupported() {
    return typeof navigator !== 'undefined' && !!navigator.bluetooth;
}

/**
 * Scan for and connect to a BLE device that exposes the NUS service.
 * Must be called in response to a user gesture.
 * @returns {Promise<void>}
 */
export async function connect() {
    if (!isSupported()) {
        throw new Error('WebBluetooth is not supported in this browser.');
    }
    if (isConnected()) {
        return; // already connected
    }

    const device = await navigator.bluetooth.requestDevice({
        filters: [{ services: [NUS_SERVICE_UUID] }],
    });

    device.addEventListener('gattserverdisconnected', _onDisconnected);

    const server = await device.gatt.connect();
    const service = await server.getPrimaryService(NUS_SERVICE_UUID);

    rxCharacteristic = await service.getCharacteristic(NUS_RX_CHAR_UUID);

    const txCharacteristic = await service.getCharacteristic(NUS_TX_CHAR_UUID);
    await txCharacteristic.startNotifications();
    txCharacteristic.addEventListener('characteristicvaluechanged', _onNotification);

    bleDevice = device;
    receiveBuffer = [];
    notifyStatusListeners();
}

/**
 * Disconnect from the currently connected BLE device.
 * @returns {void}
 */
export function disconnect() {
    if (bleDevice) {
        if (bleDevice.gatt.connected) {
            bleDevice.gatt.disconnect();
        }
        _cleanup();
    }
}

/**
 * Send a UTF-8 string over the NUS RX characteristic (browser → device).
 * @param {string} data
 * @returns {Promise<void>} Resolves when the write completes. Rejects if not connected
 *   or if the underlying GATT write fails.
 */
export async function send(data) {
    if (!rxCharacteristic) {
        throw new Error('NUS: not connected');
    }
    const encoded = new TextEncoder().encode(data);
    await rxCharacteristic.writeValue(encoded);
}

/**
 * Poll the receive buffer for the next chunk received from the NUS TX characteristic.
 * Returns an empty string if the buffer is empty.
 * @returns {string}
 */
export function receive() {
    return receiveBuffer.length > 0 ? receiveBuffer.shift() : '';
}

// ---- internal helpers ----

function _onNotification(event) {
    const value = new TextDecoder('utf-8').decode(event.target.value);
    receiveBuffer.push(value);
}

function _onDisconnected() {
    _cleanup();
}

function _cleanup() {
    bleDevice = null;
    rxCharacteristic = null;
    receiveBuffer = [];
    notifyStatusListeners();
}
