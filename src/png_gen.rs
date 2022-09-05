use deflate::{self, Compression};
use crc;

pub fn default_png_gen() -> Vec<u8> {
	return png_gen(1, 1, 255, 255, 255, 255);
}

pub fn png_gen(width: u32, height: u32, red: u8, green: u8, blue: u8, alpha: u8) -> Vec<u8> {
	let signature = vec![
		137, 80, 78, 71, 13, 10, 26, 10
	];
	let iend = vec![
		// length
		vec![0, 0, 0, 0],
		// ID IEND
		vec![73, 69, 78, 68],
		// CRC
		build_crc(&[73, 69, 78, 68][..])
	].concat();

	let ihdr_h = vec![
		73, 72, 68, 82
	];
	let ihdr_b = vec![
		// Width 4 bytes
		((0xff_00_00_00 & width) >> 24) as u8, ((0xff_00_00 & width)>> 16) as u8, ((0xff_00 & width) >> 8) as u8, (0xff & width) as u8,
		// Height 4 bytes
		((0xff_00_00_00 & height) >> 24) as u8, ((0xff_00_00 & height) >> 16) as u8, ((0xff_00 & height) >> 8) as u8, (0xff & height) as u8,
		// Bit depth 1 byte
		8,
		// Colour type 1 byte
		6,
		// Compression method 1 byte
		0,
		// Filter method 1 byte
		0,
		// Interlace method 1 byte
		0
	];
	let ihdr_hb = deep_copy_from_vecs(&ihdr_h, &ihdr_b);
	let ihdr = [
		build_length(&ihdr_b),
		ihdr_h,
		ihdr_b,
		build_crc(&ihdr_hb[..])
	].concat();

	let idat_h = vec![
		73, 68, 65, 84
	];
	let idat_b = build_body(width, height, red, green, blue, alpha);
	let idat_hb = deep_copy_from_vecs(&idat_h, &idat_b);
	let idat = [
		build_length(&idat_b),
		idat_h,
		idat_b,
		build_crc(&idat_hb[..])
	].concat();

	return [signature, ihdr, idat, iend].concat();
}

fn build_body(width: u32, height: u32, red: u8, green: u8, blue: u8, alpha: u8) -> Vec<u8> {
	let mut body = Vec::new();

	for _i in 0..height {
		body.push(0);
		for _j in 0..width {
			//body.push((map_unsigned(red, 0b111) << 5) + (map_unsigned(green, 0b111) << 2) + (map_unsigned(blue, 0b11)));
			body.push(red);
			body.push(green);
			body.push(blue);
			body.push(alpha);
		}
	}

	let deflated_body = deflate::deflate_bytes_zlib_conf(&body[..], Compression::Best);

	return deflated_body;
}
fn build_length(arr: &[u8]) -> Vec<u8> {
	let len = arr.len();

	return vec![((0xff000000 & len) >> 24) as u8, ((0xff0000 & len)>> 16) as u8, ((0xff00 & len) >> 8) as u8, (0xff & len) as u8];
}
fn build_crc(arr: &[u8]) -> Vec<u8> {
	let summer = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

	let mut digest = summer.digest();
	digest.update(arr);

	let checksum = digest.finalize();

	return vec![((0xff_00_00_00 & checksum) >> 24) as u8, ((0xff_00_00 & checksum)>> 16) as u8, ((0xff_00 & checksum) >> 8) as u8, (0xff & checksum) as u8];
}
fn deep_copy_from_vecs(vec0: &Vec<u8>, vec1: &Vec<u8>) -> Vec<u8> {
	let mut vec_final = vec![];
	let mut size = vec0.len();
	for i in 0..size {
		vec_final.push(vec0[i]);
	}
	size = vec1.len();
	for i in 0..size {
		vec_final.push(vec1[i]);
	}
	return vec_final;
}