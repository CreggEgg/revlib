// use autocxx::include_cpp;
// use ffi::{
//     _c_REVLib_ErrorCode, c_SparkMax_ControlType, c_SparkMax_SetpointCommand, c_SparkMax_handle,
// };

// include_cpp! {
//     #include "rev/CANSparkMaxDriver.h"
//     safety!(unsafe)
//     generate!("c_SparkMax_Create")
//     generate!("c_SparkMax_MotorType")
//     generate!("c_SparkMax_SparkModel")
//     generate!("_c_REVLib_ErrorCode")
//     generate!("c_SparkMax_SetpointCommand")
//     generate!("c_SparkMax_ControlType")
// }
#[allow(warnings)]
mod rev;
// fn main() {
    
//     println!("Hello, world!");
//     let mut error = rev::_c_REVLib_ErrorCode::c_REVLibError_None;
//     let mc = unsafe {
//         dbg!(rev::c_SparkMax_RegisterId(6));
//         rev::c_SparkMax_Create(
//             6.into(),
//             rev::c_SparkMax_MotorType::c_SparkMax_kBrushless,
//             rev::c_SparkMax_SparkModel::c_SparkMax_SparkMax,
//             &mut error,
//         )
//     };
//     dbg!(&error);
    
//     set_speed(mc, 1.0);
//     dbg!(&error);
//     std::thread::sleep(std::time::Duration::from_secs(3));
//     dbg!(&error);
//     set_speed(mc, 0.0);
// }
pub struct SparkMax;

impl MotorControllerType for SparkMax {
    fn raw() -> rev::c_SparkMax_SparkModel {
        rev::c_SparkMax_SparkModel::c_SparkMax_SparkMax
    }
}

pub struct SparkFlex;

impl MotorControllerType for SparkFlex {
    fn raw() -> rev::c_SparkMax_SparkModel {
        rev::c_SparkMax_SparkModel::c_SparkMax_SparkFlex
    }
}


// pub enum MotorControllerType {
//     SparkMax,
//     SparkFlex
// } 
// impl MotorControllerType {
//     fn get_raw(&self)  -> rev::c_SparkMax_SparkModel
// }
pub trait MotorControllerType {
    fn raw()  -> rev::c_SparkMax_SparkModel;
}

pub enum MotorType {
    Brushless,
    Brushed
}

impl MotorType {
    fn raw(&self) ->rev::c_SparkMax_MotorType  {
        match self {
            MotorType::Brushless => rev::c_SparkMax_MotorType::c_SparkMax_kBrushless,
            MotorType::Brushed => rev::c_SparkMax_MotorType::c_SparkMax_kBrushed,
        }
    }
}

pub struct MotorController<Controller> where Controller: MotorControllerType {
    handle: rev::c_SparkMax_handle,
    _controller: Controller
}
impl<Controller: MotorControllerType> MotorController<Controller> {
    pub fn new(controller: Controller, id: i32, motor_type: MotorType) -> Self {
        unsafe {dbg!(rev::c_SparkMax_RegisterId(id))};
        let mut error = rev:: _c_REVLib_ErrorCode::c_REVLibError_None;
        let mc = MotorController {
            handle: 
                    unsafe {rev::c_SparkMax_Create(
                            id,
                             motor_type.raw(),
                            Controller::raw(),
                            &mut error,
                        )},
            _controller: controller
        };
        assert_eq!(error, rev::_c_REVLib_ErrorCode::c_REVLibError_None);
        mc
        
    }
    pub fn set_speed(&mut self, value: f32) {
        assert_eq!(unsafe {
            rev::c_SparkMax_SetpointCommand(
                self.handle,
                value,
                rev::c_SparkMax_ControlType::c_SparkMax_kVoltage,
                0.into(),
                0.,
                0.into(),
            )
        }, rev::_c_REVLib_ErrorCode::c_REVLibError_None);
    }
}
