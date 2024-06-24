mod common;
use crate::common::{destroy_i2c, new_i2c, Register, MAG_ADDR};
use embedded_hal_mock::eh1::i2c::Transaction as I2cTrans;

#[cfg_attr(not(feature = "async"), test)]
#[cfg_attr(feature = "async", tokio::test)]
#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), keep_self,),
    async(cfg(feature = "async"), keep_self,)
)]
async fn can_change_into_continuous() {
    let sensor = new_i2c(&[I2cTrans::write(MAG_ADDR, vec![Register::CFG_REG_A_M, 0])]);
    let sensor = sensor.into_mag_continuous().await.ok().unwrap();
    destroy_i2c(sensor);
}

#[cfg_attr(not(feature = "async"), test)]
#[cfg_attr(feature = "async", tokio::test)]
#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), keep_self,),
    async(cfg(feature = "async"), keep_self,)
)]
async fn can_change_into_one_shot() {
    let sensor = new_i2c(&[
        I2cTrans::write(MAG_ADDR, vec![Register::CFG_REG_A_M, 0x0]),
        I2cTrans::write(MAG_ADDR, vec![Register::CFG_REG_A_M, 0x3]),
    ]);
    let sensor = sensor.into_mag_continuous().await.ok().unwrap();
    let sensor = sensor.into_mag_one_shot().await.ok().unwrap();
    destroy_i2c(sensor);
}
