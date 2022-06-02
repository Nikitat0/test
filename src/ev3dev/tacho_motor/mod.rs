mod polarity;

pub use polarity::*;

use crate as ev3robot;
use crate::device::{ReadOnlyAttributeFile, ReadWriteAttributeFile};

#[derive(Debug, Device)]
#[ev3robot(class = "tacho-motor")]
pub struct TachoMotorDevice {
    pub count_per_rot: u32,
    pub duty_cycle: ReadOnlyAttributeFile<i8>,
    pub duty_cycle_sp: ReadWriteAttributeFile<i8>,
    pub polarity: ReadWriteAttributeFile<Polarity>,
    pub position: ReadWriteAttributeFile<i32>,
    pub position_sp: ReadWriteAttributeFile<i32>,
    pub max_speed: u32,
    pub speed: ReadOnlyAttributeFile<i32>,
    pub speed_sp: ReadWriteAttributeFile<i32>,
}
