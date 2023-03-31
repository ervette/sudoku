#[derive(Copy, Clone)]
pub struct Terminal {
    width: u16,
    height: u16
}

impl Terminal {
    pub fn new(width: u16, height: u16) -> Self {
        Terminal { width, height }
    }

    pub fn h_center(self) -> u16 {
        self.width / 2
    }

    pub fn v_center(self) -> u16 {
        self.height / 2
    }

    pub fn h_center_str(self, string: &str) -> u16 {
        (self.width - string.len() as u16) / 2
    }

    pub fn v_center_str(self, string: &str) -> u16 {
        let string_height = string.chars().fold(1, |acc, c| match c {'\n' => acc+1, _ => acc});
        (self.height - string_height) / 2
    }

    // return true if changed
    pub fn set_size(&mut self, width: u16, height: u16) -> bool {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            true
        } else {
            false
        }
    }
}
