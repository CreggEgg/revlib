use autocxx::include_cpp;
use ffi::{
    _c_REVLib_ErrorCode, c_SparkMax_ControlType, c_SparkMax_SetpointCommand, c_SparkMax_handle,
};

include_cpp! {
    #include "rev/CANSparkMaxDriver.h"
    safety!(unsafe)
    generate!("c_SparkMax_Create")
    generate!("c_SparkMax_MotorType")
    generate!("c_SparkMax_SparkModel")
    generate!("_c_REVLib_ErrorCode")
    generate!("c_SparkMax_SetpointCommand")
    generate!("c_SparkMax_ControlType")
}
fn main() {
    println!("Hello, world!");
    let mut error = _c_REVLib_ErrorCode::c_REVLibError_None;
    let mc = unsafe {
        ffi::c_SparkMax_Create(
            0.into(),
            ffi::c_SparkMax_MotorType::c_SparkMax_kBrushless,
            ffi::c_SparkMax_SparkModel::c_SparkMax_SparkMax,
            &mut error,
        )
    };
    set_speed(mc, 0.1);
}
fn set_speed(mc: c_SparkMax_handle, value: f32) {
    unsafe {
        c_SparkMax_SetpointCommand(
            mc,
            value,
            c_SparkMax_ControlType::c_SparkMax_kVoltage,
            0.into(),
            0.,
            0.into(),
        )
    };
}
