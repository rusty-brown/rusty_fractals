
pub struct ResultElementStatic {
    value: u32,
    quad: f64,
    qiad: f64,
    color_value: u32,
}

impl ResultElementStatic {
    pub fn set_finished_state(&mut self, iterator: u32, q: f64) {
        /*
        self.quad = q;
        if iterator < 1 {
            self.value = 1;
        } else if iterator == ITERATION_MAX {
            self.value = 0;
        } else {
            self.value = iterator;
        }
        */
    }

    fn color_value(&self) -> u32 {
        self.color_value
    }

    fn set_color_value(&mut self, palette_colour_index: u32) {
        self.color_value = palette_colour_index;
    }

    pub fn set_average_with(&mut self, e: ResultElementStatic) {
        self.value = (((self.value + e.value) as f64) / 2.0) as u32;
    }
}


#[test]
fn test_set_average_with() {
    let mut me = ResultElementStatic {
        value: 10,
        quad: 0.0,
        qiad: 0.0,
        color_value: 0
    };
    let other = ResultElementStatic {
        value: 3,
        quad: 0.0,
        qiad: 0.0,
        color_value: 0
    };

    me.set_average_with(other);
    assert_eq!(me.value, 55);
}
