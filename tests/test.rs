/*
use embedded_hal_mock::{i2c::{Mock as I2CMock, Transaction as I2CTransaction}, spi::{Mock as SPIMock, Transaction as SPITransaction}};
use bma400::{
    i2c::I2CInterface,
    BMA400, 
    types::*,
};

pub const ADDR: u8 = 0b00010100;

pub fn new_i2c(expected: &[I2CTransaction]) -> BMA400<I2CInterface<I2CMock>> {
    BMA400::new_i2c(I2CMock::new(expected)).unwrap()
}

#[test]
fn get_chip_id() {
    let expectations = [I2CTransaction::write_read(ADDR, vec![0x00], vec![0x90])];
    let mut device = new_i2c(&expectations);
    let id = device.get_id().unwrap();
    assert_eq!(id, 0x90);
    device.destroy();
}

#[test]
fn get_unscaled_data() {
    let expectations = [
            I2CTransaction::write_read(ADDR, vec![0x04], vec![0x01, 0x08, 0xFF, 0x0F, 0xFF, 0x07])
        ];
    let mut device = new_i2c(&expectations);
    let m = device.get_unscaled_data().unwrap();
    assert_eq!(m.x, -2047);
    assert_eq!(m.y, -1);
    assert_eq!(m.z, 2047);
}

fn get_scaled_data(scale: Scale) -> (i16, i16, i16) {
    let byte = match scale {
        Scale::Range2G => 0x09,
        Scale::Range4G => 0x49,
        Scale::Range8G => 0x89,
        Scale::Range16G => 0xC9,
    };
    let mut device = if let Scale::Range4G = scale {
        // The default setting is 4G so we shouldn't see any configuration write
        let expectations = [I2CTransaction::write_read(ADDR, vec![0x04], vec![0x01, 0x08, 0xFF, 0x0F, 0xFF, 0x07])];
        new_i2c(&expectations)
    } else {
        let expectations = [
            I2CTransaction::write(ADDR, vec![0x1A, byte]),
            I2CTransaction::write_read(ADDR, vec![0x04], vec![0x01, 0x08, 0xFF, 0x0F, 0xFF, 0x07])
        ];
        new_i2c(&expectations)
    };
    device.config_accel().with_scale(scale).write().unwrap();
    let m = device.get_data().unwrap();
    (m.x, m.y, m.z)
}

#[test]
fn get_data() {
    assert_eq!((-2047, -1, 2047), get_scaled_data(Scale::Range2G));
    assert_eq!((-4094, -2, 4094), get_scaled_data(Scale::Range4G));
    assert_eq!((-8188, -4, 8188), get_scaled_data(Scale::Range8G));
    assert_eq!((-16376, -8, 16376), get_scaled_data(Scale::Range16G));
}

#[test]
fn get_sensor_clock() {
    let expectations = [
        I2CTransaction::write_read(ADDR, vec![0x0A], vec![0xF8, 0xFF, 0xFF])
    ];
    let mut device = new_i2c(&expectations);
    let t = device.get_sensor_clock().unwrap();
    assert_eq!(t, 0xFFFFF8);
}

#[test]
fn read_fifo_frames() {
    let expectations = [
        I2CTransaction::write_read(ADDR, vec![0x14], vec![0x48, 0x6E, 0x9E, 0x01, 0x80, 0x0F, 0xFF, 0x0F, 0x7F, 0xA0, 0xF8, 0xFF, 0xFF, 0x80, 0x00])
    ];
    let mut device = new_i2c(&expectations);
    let mut buffer = [0u8; 15];
    let frames = device.read_fifo_frames(&mut buffer).unwrap();
    let mut count = 0;
    for frame in frames {
        match frame.frame_type() {
            FrameType::Data => {
                assert_eq!(frame.x(), -2047);
                assert_eq!(frame.y(), -1);
                assert_eq!(frame.z(), 2047);
            },
            FrameType::Time => {
                assert_eq!(frame.time(),  0xFFFFF8);
            },
            FrameType::Control => {
                assert!(frame.fifo_chg());
                assert!(frame.acc0_chg());
                assert!(frame.acc1_chg());
            }
        }
        count +=1;
    }
    assert_eq!(count, 3);
}
*/