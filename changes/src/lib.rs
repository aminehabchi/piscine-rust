#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
	pub alias: String,
	pub brightness: u8,
}

impl Light {
	pub fn new(alias: &str) -> Self {
        let light:Light=Light{
            alias:alias.to_string(),
            brightness:0,
        };
        light
	}
}

pub fn change_brightness(lights: &mut [Light], alias: &str, value: u8) {
    // for l in lights{
    //     if l.alias==alias{
    //         l.brightness=value;
    //         break;
    //     }
    // }

    let l:Option<&mut Light>=lights.iter_mut().find(|l| l.alias==alias);
    if l.is_some(){
        l.unwrap().brightness=value;
    }
}