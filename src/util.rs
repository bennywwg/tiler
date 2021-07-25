pub mod math {
    use glam::*;
    use serde::{Serialize, Deserialize};
    use std::ops::*;

    // discrete abb2
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Dabb2 {
        pub begin: IVec2,
        pub end: IVec2
    }

    impl Dabb2 {
        pub fn area(&self) -> usize {
            assert!(self.end.x - self.begin.x >= 0 && self.end.y - self.begin.y >= 0);
            (self.end.x - self.begin.x) as usize * (self.end.y - self.begin.y) as usize
        }
        pub fn cell(position: IVec2) -> Self {
            Self {
                begin: position,
                end: position + 1
            }
        }
        pub fn bounds(begin: IVec2, end: IVec2) -> Self {
            Self {
                begin,
                end
            }
        }
    }

    impl Add<IVec2> for Dabb2 {
        type Output = Self;

        fn add(self, rhs: IVec2) -> Self {
            Self {
                begin: self.begin + rhs,
                end: self.end + rhs
            }
        }
    }

    impl Sub<IVec2> for Dabb2 {
        type Output = Self;

        fn sub(self, rhs: IVec2) -> Self {
            Self {
                begin: self.begin - rhs,
                end: self.end - rhs
            }
        }
    }
    
    impl Mul<IVec2> for Dabb2 {
        type Output = Self;

        fn mul(self, rhs: IVec2) -> Self {
            Self {
                begin: self.begin * rhs,
                end: self.end * rhs
            }
        }
    }

    impl Mul<i32> for Dabb2 {
        type Output = Self;

        fn mul(self, rhs: i32) -> Self {
            Self {
                begin: self.begin * rhs,
                end: self.end * rhs
            }
        }
    }

    impl Div<IVec2> for Dabb2 {
        type Output = Self;

        fn div(self, rhs: IVec2) -> Self {
            Self {
                begin: self.begin / rhs,
                end: self.end / rhs
            }
        }
    }

    impl Div<i32> for Dabb2 {
        type Output = Self;

        fn div(self, rhs: i32) -> Self {
            Self {
                begin: self.begin / rhs,
                end: self.end / rhs
            }
        }
    }

    impl<'a> IntoIterator for &'a Dabb2 {
        type Item = IVec2;
        type IntoIter = Dabb2IntoIterator<'a>;
    
        fn into_iter(self) -> Self::IntoIter {
            Dabb2IntoIterator {
                owner: self,
                index: self.begin
            }
        }
    }

    pub struct Dabb2IntoIterator<'a> {
        owner: &'a Dabb2,
        index: IVec2
    }

    impl<'a> Iterator for Dabb2IntoIterator<'a> {
        type Item = IVec2;

        fn next(&mut self) -> Option<Self::Item> {
            let res = self.index;
            self.index.x += 1;
            if self.index.x >= self.owner.end.x {
                self.index.y += 1;
                self.index.x = self.owner.begin.x;
            }
            if res.y >= self.owner.end.y {
                None
            } else {
                Some(res)
            }
        }

        fn count(self) -> usize {
            self.owner.area()
        }
    }

    pub trait MathUtils {
        fn correct_mod(&self, div: Self) -> Self;
        fn floor_on_interval(&self, div: Self) -> Self;
        fn ceil_on_interval(&self, div: Self) -> Self;
    }

    impl MathUtils for i32 {
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

    impl MathUtils for IVec2 {
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