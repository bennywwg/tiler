pub mod math {
    use glam::*;
    use std::ops::*;

    pub trait math_utils {
        fn correct_mod(&self, div: Self) -> Self;
        fn floor_on_interval(&self, div: Self) -> Self;
        fn ceil_on_interval(&self, div: Self) -> Self;
    }

    impl math_utils for i32 {
        fn correct_mod(&self, div: Self) -> Self {
            let mod_val = self.abs() % div;
            if *self < 0 && mod_val > 0 {
                div - mod_val
            } else {
                mod_val
            }
        }

        fn floor_on_interval(&self, div: Self) -> Self {
            *self - self.correct_mod(div)
        }

        fn ceil_on_interval(&self, div: Self) -> Self {
            self.floor_on_interval(div) + div
        }
    }

    impl math_utils for IVec2 {
        fn correct_mod(&self, div: Self) -> Self {
            ivec2(self.x.correct_mod(div.x), self.y.correct_mod(div.y))
        }

        fn floor_on_interval(&self, div: Self) -> Self {
            ivec2(self.x.floor_on_interval(div.x), self.y.floor_on_interval(div.y))
        }
        
        fn ceil_on_interval(&self, div: Self) -> Self {
            ivec2(self.x.ceil_on_interval(div.x), self.y.ceil_on_interval(div.y))
        }
    }
}