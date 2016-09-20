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

use std::io::{Read, Seek, Cursor};

use decoder;
use color;
use pixel::{self, Pixel};
use buffer::Buffer;
use format::{self, Format};
use error::{self, Error};

/// Load an image from an input stream, guessing its format.
pub fn from<C, P, R>(mut input: R) -> error::Result<Buffer<C, P, Vec<C>>>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Write<C>,
	      P: From<color::Rgb> + From<color::Rgba> + From<color::Luma> + From<color::Lumaa>,
	      R: Read + Seek
{
	let format = try!(format::guess(input.by_ref()).ok_or(Error::Format("unsupported image format".into())));
	with_format(input, format)
}

/// Load an image from memory, guessing its format.
pub fn from_memory<C, P>(slice: &[u8]) -> error::Result<Buffer<C, P, Vec<C>>>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Write<C>,
	      P: From<color::Rgb> + From<color::Rgba> + From<color::Luma> + From<color::Lumaa>
{
	from(Cursor::new(slice))
}

/// Load an image from an input stream with the given format.
pub fn with_format<C, P, R>(input: R, format: Format) -> error::Result<Buffer<C, P, Vec<C>>>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Write<C>,
	      P: From<color::Rgb> + From<color::Rgba> + From<color::Luma> + From<color::Lumaa>,
	      R: Read
{
	match format {
		#[cfg(feature = "png")]
		Format::Png =>
			decoder::load::<C, P, _>(decoder::png::Decoder::new(input)),

		#[cfg(feature = "jpeg")]
		Format::Jpeg =>
			decoder::load::<C, P, _>(decoder::jpeg::Decoder::new(input)),

		_ =>
			Err(Error::Format("unsupported image format".into())),
	}
}
