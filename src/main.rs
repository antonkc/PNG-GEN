use iron::{prelude::*, response::WriteBody};
use read_get_png_request::read_png_reques;

mod read_get_png_request;
mod png_gen;

fn main() {
	Iron::new(|req: &mut Request| {

		let procesed_data = read_png_reques( req);
		let mut response = Response::new();


		let response_content: Box<dyn WriteBody + 'static>;
		if procesed_data.is_err() {
			response.headers.set(iron::headers::ContentType::plaintext());

			let err = procesed_data.err();

			if err.is_none() {
				response.status = Some(iron::status::Status::InternalServerError);

				response_content = Box::new( "unexpected read request error");
			} else {
				let inner_err = err.unwrap();
				response.status = Some(iron::status::Status::BadRequest);

				response_content = Box::new( inner_err.as_str());
			}

		} else {
			response.status = Some(iron::status::Status::Ok);
			response.headers.set(iron::headers::ContentType::png());

			let outer_data = procesed_data.unwrap();
			if outer_data.is_none() {
				response_content = Box::new( png_gen::default_png_gen());
			} else {
				let inner_data = outer_data.unwrap();
				response_content =  Box::new( png_gen::png_gen(inner_data.width, inner_data.height, inner_data.red, inner_data.green, inner_data.blue, inner_data.alpha));
			}
		}
		let mut _body = response.body.insert(response_content);


		return Ok(response);

	}).http("localhost:8000").unwrap();
}