use std::{collections::HashMap};
use iron::{Request, Url, url::percent_encoding::percent_decode};

pub struct SolidPngData {
	pub width: u32,
	pub height: u32,
	pub red: u8,
	pub green: u8,
	pub blue: u8,
	pub alpha: u8
}

#[derive(Debug)]
pub enum ParsePngRequestError {
	DimensionsOutOfRange,
	ZeroToTwoFiveFiveChannels
}

impl ParsePngRequestError {
	pub fn as_str(&self) -> &'static str {
		match self {
				ParsePngRequestError::DimensionsOutOfRange => "width and height must be between 1 and 4294967295",
				ParsePngRequestError::ZeroToTwoFiveFiveChannels => "colors and alpha cannot be outside of 0-255 range"
		}
	}
}

pub fn read_png_reques(req: &mut Request) -> Result<Option<SolidPngData>, ParsePngRequestError> {
	let queries = parse_query(&req.url);

	let is_width = queries.contains_key("w") && queries["w"].is_some();
	let is_height = queries.contains_key("h") && queries["h"].is_some();
	let is_red = queries.contains_key("r") && queries["r"].is_some();
	let is_green = queries.contains_key("g") && queries["g"].is_some();
	let is_blue = queries.contains_key("b") && queries["b"].is_some();
	let are_colors = is_red || is_green || is_blue;
	let is_alpha = queries.contains_key("a") && queries["a"].is_some();

	if !(are_colors || is_alpha || is_width || is_height) {
		return Ok( None);
	}

	let mut dat = SolidPngData { width: 5, height: 5, red: 0, green: 0, blue: 0, alpha: 255 };

	if is_width {
		let width_str = queries["w"].as_ref().unwrap();
		let parsed_w = width_str.parse::<u32>();

		if parsed_w.is_ok() {
			dat.width = parsed_w.unwrap();
			if dat.width < 1 {
				return Err( ParsePngRequestError::DimensionsOutOfRange);
			}
		} else {
			return Err( ParsePngRequestError::DimensionsOutOfRange);
		}
	}
	if is_height {
		let height_str = queries["h"].as_ref().unwrap();
		let parsed_h = height_str.parse::<u32>();

		if parsed_h.is_ok() {
			dat.height = parsed_h.unwrap();
			if dat.height < 1 {
				return Err( ParsePngRequestError::DimensionsOutOfRange);
			}
		} else {
			return Err( ParsePngRequestError::DimensionsOutOfRange);
		}
	}

	if are_colors {
		if is_red {
			let str = queries["r"].as_ref().unwrap();
			let parsed = str.parse::<u8>();

			if parsed.is_ok() {
				dat.red = parsed.unwrap();
			} else {
				return Err( ParsePngRequestError::ZeroToTwoFiveFiveChannels);
			}
		}
		if is_green {
			let str = queries["g"].as_ref().unwrap();
			let parsed = str.parse::<u8>();

			if parsed.is_ok() {
				dat.green = parsed.unwrap();
			} else {
				return Err( ParsePngRequestError::ZeroToTwoFiveFiveChannels);
			}
		}
		if is_blue {
			let str = queries["b"].as_ref().unwrap();
			let parsed = str.parse::<u8>();

			if parsed.is_ok() {
				dat.blue = parsed.unwrap();
			} else {
				return Err( ParsePngRequestError::ZeroToTwoFiveFiveChannels);
			}
		}
	} else {
		dat.red = 255;
		dat.green = 255;
		dat.blue = 255;
	}

	if is_alpha {
		let str = queries["a"].as_ref().unwrap();
		let parsed = str.parse::<u8>();

		if parsed.is_ok() {
			dat.alpha = parsed.unwrap();
		} else {
			return Err( ParsePngRequestError::ZeroToTwoFiveFiveChannels);
		}
	}

	return Ok( Some(dat));
}

fn parse_query(url: &Url) -> HashMap<String, Option<String>> {
	let mut map: HashMap<String, Option<String>> = HashMap::new();
	let opt_query = url.query();
	if opt_query.is_none() {
		return map;
	} else {
		let query = opt_query.unwrap();
		let params_def: Vec<&str> = query.split('&').collect();

		for i in 0..params_def.len() {
			let param_def = params_def[i];
			if !param_def.is_empty() {
				let last_split_opt = param_def.split_once('=');
				if last_split_opt.is_none() {
					let _inserted = map.insert( whole_percent_decode(param_def) , None);
				} else {
					let last_split = last_split_opt.unwrap();
					let _inserted = map.insert(whole_percent_decode(last_split.0), Some(whole_percent_decode(last_split.1)));
				}
			}
		}

		return map;
	}
}

fn whole_percent_decode(str: &str) -> String {
	// if str is user input, should I worry about unwrapping panic here?
	percent_decode(str.as_bytes()).decode_utf8().unwrap().to_string()
}