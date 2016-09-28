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

use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;

use orientation::Orientation;
use pixel::{self, Pixel};
use view::{self, View};
use area::{self, Area};
use color;
use iter::pixel::{Iter as Pixels, IterMut as PixelsMut};

/// Buffer for an image.
#[derive(PartialEq, Debug)]
pub struct Buffer<C, P, D>
	where C: pixel::Channel,
	      P: Pixel<C>
{
	area: Area,
	data: D,

	_channel: PhantomData<C>,
	_pixel:   PhantomData<P>,
}

impl<C, P> Buffer<C, P, Vec<C>>
	where C: pixel::Channel,
	      P: Pixel<C>,
{
	/// Create a new `Buffer` with the requested space allocated and all channels
	/// set to `0`.
	///
	/// # Example
	///
	/// ```
	/// use picto::Buffer;
	/// use picto::color::Rgb;
	///
	/// Buffer::<u8, Rgb, _>::new(1024, 1024);
	/// ```
	#[inline]
	pub fn new(width: u32, height: u32) -> Self {
		Buffer {
			area: Area::from(0, 0, width, height),
			data: vec![zero!(); width as usize * height as usize * P::channels()],

			_channel: PhantomData,
			_pixel:   PhantomData,
		}
	}
}

impl<C, P> Buffer<C, P, Vec<C>>
	where C: pixel::Channel,
	      P: pixel::Write<C>,
{
	/// Create a new `Buffer` with the request space allocated and filled with
	/// the given pixel.
	///
	/// # Example
	///
	/// ```
	/// use picto::Buffer;
	/// use picto::color::Rgb;
	///
	/// Buffer::<u8, Rgb, _>::from_pixel(1024, 1024, &Rgb::new(1.0, 0.0, 0.0));
	/// ```
	#[inline]
	pub fn from_pixel(width: u32, height: u32, pixel: &P) -> Self {
		let mut buffer = Self::new(width, height);
		buffer.fill(pixel);

		buffer
	}

	/// Create a new `Buffer` with the request space allocated and filled with
	/// the pixel returned by the given function.
	///
	/// The function takes the coordinates and returns a pixel.
	///
	/// # Example
	///
	/// ```
	/// use picto::Buffer;
	/// use picto::color::Rgb;
	///
	/// Buffer::<u8, Rgb, _>::from_fn(1024, 1024, |x, y| {
	///     let w = (x as f32 + y as f32) / 2048.0;
	///     Rgb::new(w, w, w)
	/// });
	/// ```
	#[inline]
	pub fn from_fn<T, F>(width: u32, height: u32, mut func: F) -> Self
		where T: Into<P>,
		      F: FnMut(u32, u32) -> T
	{
		let mut buffer = Self::new(width, height);

		for (x, y) in buffer.area().absolute() {
			buffer.set(x, y, &func(x, y).into());
		}

		buffer
	}
}

impl<C, P> Buffer<C, P, Vec<C>>
	where C: pixel::Channel,
	      P: pixel::Write<C> + color::Mix + Clone,
{
	/// Create a `Buffer` from an orientation and a gradient.
	///
	/// # Example
	///
	/// ```
	/// use picto::{Buffer, Orientation};
	/// use picto::color::{Rgb, Gradient};
	///
	/// Buffer::<u8, Rgb, _>::from_gradient(1024, 1024, Orientation::Horizontal, Gradient::new(
	///     vec![Rgb::new(0.0, 0.0, 0.0), Rgb::new(1.0, 1.0, 1.0), Rgb::new(0.0, 0.0, 0.0)]));
	/// ```
	#[inline]
	pub fn from_gradient(width: u32, height: u32, mode: Orientation, gradient: color::Gradient<P>) -> Self {
		let mut buffer = Buffer::new(width, height);

		match mode {
			Orientation::Vertical => {
				for (y, px) in (0 .. height).zip(gradient.take(height as usize)) {
					for x in 0 .. width {
						buffer.set(x, y, &px);
					}
				}
			}

			Orientation::Horizontal => {
				for (x, px) in (0 .. width).zip(gradient.take(width as usize)) {
					for y in 0 .. height {
						buffer.set(x, y, &px);
					}
				}
			}
		}

		buffer
	}
}

impl<C, P, D> Buffer<C, P, D>
	where C: pixel::Channel,
	      P: Pixel<C>,
	      D: Deref<Target = [C]>
{
	/// Use an existing container as backing storage for an image `Buffer`.
	///
	/// The size of the storage is compared against the supplied dimensions and
	/// `P::channel()`.
	///
	/// # Example
	///
	/// ```
	/// use picto::Buffer;
	/// use picto::color::Rgb;
	///
	/// Buffer::<u8, Rgb, _>::from_raw(2, 2, vec![
	///     255,   0,   0,
	///       0, 255,   0,
	///       0,   0, 255,
	///     255,   0, 255,
	/// ]).unwrap();
	/// ```
	#[inline]
	pub fn from_raw(width: u32, height: u32, data: D) -> Result<Self, ()> {
		if width as usize * height as usize * P::channels() != data.len() {
			return Err(());
		}

		Ok(Buffer {
			area: Area::from(0, 0, width, height),
			data: data,

			_channel: PhantomData,
			_pixel:   PhantomData,
		})
	}
}

impl<C, P, D> Buffer<C, P, D>
	where C: pixel::Channel,
	      P: Pixel<C>
{
	/// Get the backing storage of the `Buffer`.
	#[inline]
	pub fn into_raw(self) -> D {
		self.data
	}

	/// Get the `Area` of the `Buffer`.
	#[inline]
	pub fn area(&self) -> Area {
		self.area
	}

	/// Get the dimensions as a tuple containing width and height.
	#[inline]
	pub fn dimensions(&self) -> (u32, u32) {
		(self.area.width, self.area.height)
	}

	/// Get the width.
	#[inline]
	pub fn width(&self) -> u32 {
		self.area.width
	}

	/// Get the height.
	#[inline]
	pub fn height(&self) -> u32 {
		self.area.height
	}
}

impl<C, P, D> Buffer<C, P, D>
	where C: pixel::Channel,
	      P: pixel::Read<C>,
	      D: Deref<Target = [C]>
{
	/// Get the `Pixel` at the given coordinates.
	///
	/// # Panics
	///
	/// Requires that `x < self.width()` and `y < self.height()`, otherwise it will panic.
	#[inline]
	pub fn get(&self, x: u32, y: u32) -> P {
		view::Read::new(&self.data, self.area, self.area).get(x, y)
	}

	/// Get a read-only of the given area.
	///
	/// # Panics
	///
	/// Requires that `x + width <= self.width()` and `y + height <= self.height()`, otherwise it will panic.
	#[inline]
	pub fn readable(&self, area: area::Builder) -> view::Read<C, P> {
		let area = area.complete(self.area);

		if area.x + area.width > self.area.width || area.y + area.height > self.area.height {
			panic!("out of bounds");
		}

		view::Read::new(&self.data, self.area, area)
	}

	/// Get an immutable `Iterator` over the pixels.
	#[inline]
	pub fn pixels(&self) -> Pixels<C, P> {
		Pixels::new(&self.data, self.area, self.area)
	}

	/// Convert the `Buffer` to another `Buffer` with different channel and pixel type.
	///
	/// # Example
	///
	/// ```
	/// use picto::read;
	/// use picto::Area;
	/// use picto::color::{Rgb, Lumaa};
	///
	/// let image = read::from_path::<u8, Rgb, _>("tests/boat.xyz").unwrap();
	///
	/// // Convert the `Buffer` from Rgb to grayscale with alpha.
	/// image.convert::<u8, Lumaa>();
	/// ```
	#[inline]
	pub fn convert<CO, PO>(&self) -> Buffer<CO, PO, Vec<CO>>
		where CO: pixel::Channel,
		      PO: pixel::Write<CO>,
		      P: Into<PO>
	{
		let mut result = Buffer::<CO, PO, Vec<_>>::new(self.area.width, self.area.height);

		for (input, output) in self.chunks(P::channels()).zip(result.chunks_mut(PO::channels())) {
			P::read(input).into().write(output)
		}

		result
	}
}

impl<C, P, D> Buffer<C, P, D>
	where C: pixel::Channel,
	      P: pixel::Write<C>,
	      D: DerefMut<Target = [C]>
{
	/// Set the `Pixel` at the given coordinates.
	///
	/// # Panics
	///
	/// Requires that `x < self.width()` and `y < self.height()`, otherwise it will panic.
	#[inline]
	pub fn set(&mut self, x: u32, y: u32, pixel: &P) {
		view::Write::new(&mut self.data, self.area, self.area).set(x, y, pixel)
	}

	/// Get a write-only view of the given area.
	///
	/// # Panics
	///
	/// Requires that `x + width <= self.width()` and `y + height <= self.height()`, otherwise it will panic.
	#[inline]
	pub fn writable(&mut self, area: area::Builder) -> view::Write<C, P> {
		let area = area.complete(self.area);

		if area.x + area.width > self.area.width || area.y + area.height > self.area.height {
			panic!("out of bounds");
		}

		view::Write::new(&mut self.data, self.area, area)
	}

	/// Fill the buffer with the given pixel.
	///
	/// # Example
	///
	/// ```
	/// use picto::read;
	/// use picto::color::Rgb;
	///
	/// let mut image = read::from_path::<u8, Rgb, _>("tests/boat.xyz").unwrap();
	/// image.fill(&Rgb::new(1.0, 1.0, 1.0));
	/// ```
	#[inline]
	pub fn fill(&mut self, pixel: &P) {
		for chunk in self.chunks_mut(P::channels()) {
			pixel.write(chunk);
		}
	}
}

impl<C, P, D> Buffer<C, P, D>
	where C: pixel::Channel,
	      P: pixel::Write<C> + pixel::Read<C>,
	      D: DerefMut<Target = [C]>
{
	/// Get a view of the given area.
	///
	/// # Panics
	///
	/// Requires that `x + width <= self.width()` and `y + height <= self.height()`,
	/// otherwise it will panic.
	///
	/// # Example
	///
	/// ```
	/// use picto::read;
	/// use picto::Area;
	/// use picto::color::Rgba;
	///
	/// let mut image = read::from_path::<u8, Rgba, _>("tests/boat.xyz").unwrap();
	/// let mut view  = image.view(Area::new().x(10).y(10).width(20).height(30));
	///
	/// for (_, _, mut px) in view.pixels_mut() {
	///     // Get the current value.
	///     let p = px.get();
	///
	///     // Make it opaque.
	///     px.set(&Rgba { alpha: 0.5, .. p });
	/// }
	/// ```
	#[inline]
	pub fn view(&mut self, area: area::Builder) -> View<C, P> {
		let area = area.complete(self.area);

		if area.x + area.width > self.area.width || area.y + area.height > self.area.height {
			panic!("out of bounds");
		}

		View::new(&mut self.data, self.area, area)
	}

	/// Get a mutable `Iterator` over the pixels.
	///
	/// # Example
	///
	/// ```
	/// use picto::read;
	/// use picto::color::{IntoColor, Hue, RgbHue, Rgb};
	///
	/// let mut image = read::from_path::<u8, Rgb, _>("tests/boat.xyz").unwrap();
	///
	/// for (x, y, mut px) in image.pixels_mut() {
	///     // Get the pixel value.
	///     let p = px.get();
	///
	///     // Convert to HSL and shift the hue.
	///     let p = p.into_hsl().shift_hue(RgbHue::from_radians(90.0));
	///
	///     // Set the pixel value.
	///     px.set(&p.into());
	/// }
	/// ```
	#[inline]
	pub fn pixels_mut(&mut self) -> PixelsMut<C, P> {
		PixelsMut::new(&mut self.data, self.area, self.area)
	}
}

impl<'a, C, P, D> From<&'a Buffer<C, P, D>> for view::Read<'a, C, P>
	where C: pixel::Channel,
	      P: pixel::Read<C>,
	      D: Deref<Target = [C]>
{
	#[inline]
	fn from(value: &'a Buffer<C, P, D>) -> view::Read<'a, C, P> {
		value.readable(Default::default())
	}
}

impl<'a, C, P, D> From<&'a mut Buffer<C, P, D>> for view::Write<'a, C, P>
	where C: pixel::Channel,
	      P: pixel::Write<C>,
	      D: DerefMut<Target = [C]>,
{
	#[inline]
	fn from(mut value: &'a mut Buffer<C, P, D>) -> view::Write<'a, C, P> {
		value.writable(Default::default())
	}
}

impl<'a, C, P, D> From<&'a mut Buffer<C, P, D>> for View<'a, C, P>
	where C: pixel::Channel,
	      P: pixel::Write<C> + pixel::Read<C>,
	      D: DerefMut<Target = [C]>
{
	#[inline]
	fn from(mut value: &'a mut Buffer<C, P, D>) -> View<'a, C, P> {
		value.view(Default::default())
	}
}

impl<C, P, D> Deref for Buffer<C, P, D>
	where C: pixel::Channel,
	      P: Pixel<C>,
	      D: Deref<Target = [C]>
{
	type Target = D::Target;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.data
	}
}

impl<C, P, D> DerefMut for Buffer<C, P, D>
	where C: pixel::Channel,
	      P: Pixel<C>,
	      D: DerefMut<Target = [C]>
{
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.data
	}
}

impl<C, P, D> Clone for Buffer<C, P, D>
	where C: pixel::Channel,
	      P: Pixel<C>,
	      D: Clone
{
	#[inline]
	fn clone(&self) -> Self {
		Buffer {
			area: self.area,
			data: self.data.clone(),

			_channel: PhantomData,
			_pixel:   PhantomData,
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use color::*;

	#[test]
	fn new() {
		assert_eq!(3, Buffer::<u8, Rgb, Vec<_>>::new(1, 1).into_raw().len());
		assert_eq!(6, Buffer::<u8, Rgb, Vec<_>>::new(1, 2).into_raw().len());
		assert_eq!(6, Buffer::<u8, Rgb, Vec<_>>::new(2, 1).into_raw().len());
		assert_eq!(12, Buffer::<u8, Rgb, Vec<_>>::new(2, 2).into_raw().len());
	}

	#[test]
	fn from_raw() {
		assert!(Buffer::<u8, Rgb, _>::from_raw(1, 1, vec![0, 0, 0]).is_ok());
		assert!(Buffer::<u8, Rgb, _>::from_raw(1, 2, vec![0, 0, 0, 0, 0, 0]).is_ok());
		assert!(Buffer::<u8, Rgb, _>::from_raw(2, 1, vec![0, 0, 0, 0, 0, 0]).is_ok());
		assert!(Buffer::<u8, Rgb, _>::from_raw(2, 2, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).is_ok());

		assert!(Buffer::<u8, Rgb, _>::from_raw(1, 1, vec![0, 0, 0, 0]).is_err());
		assert!(Buffer::<u8, Rgb, _>::from_raw(1, 1, vec![0, 0, 0, 0]).is_err());
	}

	#[test]
	fn into_raw() {
		assert_eq!(vec![1, 2, 3],
			Buffer::<u8, Rgb, _>::from_raw(1, 1, vec![1, 2, 3]).unwrap().into_raw());

		assert_eq!(vec![0, 0, 0],
			Buffer::<u8, Rgb, Vec<_>>::new(1, 1).into_raw());
	}

	#[test]
	fn deref() {
		assert!(Buffer::<u8, Rgb, _>::from_raw(1, 1, vec![0, 0, 0]).unwrap().len() == 3);
	}

	#[test]
	fn clone() {
		let a = Buffer::<u8, Rgb, _>::from_raw(1, 1, vec![0, 0, 0]).unwrap();
		let b = a.clone();

		assert_eq!(a.get(0, 0), b.get(0, 0));
	}

	#[test]
	fn eq() {
		let a = Buffer::<u8, Rgb, _>::from_raw(1, 1, vec![0, 0, 0]).unwrap();
		let b = a.clone();

		assert_eq!(a, b);
	}

	#[test]
	fn convert() {
		let a = Buffer::<u8, Rgb, _>::from_raw(1, 1, vec![255, 0, 255]).unwrap();
		let b = a.convert::<u8, Rgba>();

		assert_eq!(Rgba::new(1.0, 0.0, 1.0, 1.0),
			b.get(0, 0));

		assert_eq!(vec![255, 0, 255, 255],
			b.into_raw());
	}
}
