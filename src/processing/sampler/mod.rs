//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

use num::Float;

pub trait Sampler<T: Float = f32> {
	fn kernel(value: T) -> T;
	fn support() -> T;
}

mod nearest;
pub use self::nearest::Nearest;

mod linear;
pub use self::linear::Linear;

pub mod cubic;
pub use self::cubic::Cubic;

pub mod gaussian;
pub use self::gaussian::Gaussian;

pub mod lanczos;
pub use self::lanczos::{Lanczos2, Lanczos3};
