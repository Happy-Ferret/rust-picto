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

use view;
use buffer::Buffer;
use pixel::{self, Pixel};

pub trait Scaler<CI, PI, CO, PO>
	where CI: pixel::Channel,
	      PI: Pixel<CI> + pixel::Read<CI>,
	      CO: pixel::Channel,
	      PO: Pixel<CO> + pixel::Write<CO>
{
	fn scale(input: &view::Ref<CI, PI>, width: u32, height: u32) -> Buffer<CO, PO, Vec<CO>>;
}

mod sampler;
pub use super::sampler::Linear;
pub use super::sampler::Cubic;
pub use super::sampler::Lanczos2;
pub use super::sampler::Lanczos3;

mod nearest;
pub use self::nearest::Nearest;
