use olc_pixel_game_engine as olc;

struct ExampleProgram {
	pub current_scene: Scene,
	pub render_index: u64,
	pub complete: bool
}

pub mod structs;
use crate::structs::*;

const VIEWPORT_HEIGHT: u64 = 90*1;
const VIEWPORT_WIDTH: u64 = 160*1;


impl olc::Application for ExampleProgram {
	fn on_user_create(&mut self) -> Result<(), olc::Error> {
		olc::clear(olc::BLACK);
		Ok(())
	}

	fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
		//let render_pos_x = self.render_index % VIEWPORT_WIDTH;
		//let render_pos_y = self.render_index / VIEWPORT_WIDTH;
		if self.complete != true {
			for _ in 0..(VIEWPORT_WIDTH * VIEWPORT_HEIGHT) {
				let cast_ray_final_color: Color = self.current_scene.cast_ray(self.render_index, VIEWPORT_WIDTH as i32, VIEWPORT_HEIGHT as i32); 
				olc::draw((self.render_index % VIEWPORT_WIDTH) as i32, (self.render_index / VIEWPORT_WIDTH) as i32, 
				olc::Pixel { r: cast_ray_final_color.r, g: cast_ray_final_color.g, b: cast_ray_final_color.b, a:255 });
				self.render_index += 1; 
			}
			//if self.render_index >= VIEWPORT_HEIGHT * VIEWPORT_WIDTH { self.complete = true }
			self.render_index = 0;
		}

		if olc::get_key(olc::Key::RIGHT).held {
			self.current_scene.current_camera.rot.yaw -= 0.01;
		}

		if olc::get_key(olc::Key::LEFT).held {
			self.current_scene.current_camera.rot.yaw += 0.01;
		}

		if olc::get_key(olc::Key::UP).held {
			self.current_scene.current_camera.rot.roll += 0.01;
		}

		if olc::get_key(olc::Key::DOWN).held {
			self.current_scene.current_camera.rot.roll -= 0.01;
		}

		if olc::get_key(olc::Key::R).held {
			self.current_scene.current_camera.fov += 1;
		}

		if olc::get_key(olc::Key::F).held {
			self.current_scene.current_camera.fov -= 1;
		}


		if olc::get_key(olc::Key::H).held {
			for light in self.current_scene.get_all_light_sources().iter_mut() {
				if light.id == String::from("fuckin' light") {
					light.pos.y -= 0.05;
				}
			}
		}

		if olc::get_key(olc::Key::Y).held {
			for light in self.current_scene.get_all_light_sources().iter_mut() {
				if light.id == String::from("fuckin' light") {
					light.pos.y += 0.05;
				}
			}
		}

		if olc::get_key(olc::Key::U).held {
			for light in self.current_scene.get_all_light_sources().iter_mut() {
				if light.id == String::from("fuckin' light") {
					light.pos.z -= 0.05;
				}
			}
		}

		if olc::get_key(olc::Key::T).held {
			for light in self.current_scene.get_all_light_sources().iter_mut() {
				if light.id == String::from("fuckin' light") {
					light.pos.z += 0.05;
				}
			}
		}

		if olc::get_key(olc::Key::G).held {
			for light in self.current_scene.get_all_light_sources().iter_mut() {
				if light.id == String::from("fuckin' light") {
					light.pos.x -= 0.05;
				}
			}
		}

		if olc::get_key(olc::Key::J).held {
			for light in self.current_scene.get_all_light_sources().iter_mut() {
				if light.id == String::from("fuckin' light") {
					light.pos.x += 0.05;
				}
			}
		}

		// Rotate the fuckin' cube
		for mesh in self.current_scene.get_all_meshes().iter_mut() {
			if mesh.id == String::from("fuckin' cube") {
				mesh.rot.pitch += 0.01;
				mesh.rot.roll += 0.01;
				mesh.rot.yaw += 0.01;
			}
		}

		//let _ = olc::draw_string(0, 0, &(String::from("roll ") + &self.current_scene.current_camera.rot.roll.to_string()), olc::WHITE);
		//let _ = olc::draw_string(0, 10, &(String::from("yaw ") + &self.current_scene.current_camera.rot.yaw.to_string()), olc::WHITE);
		//let _ = olc::draw_string(0, 20, &(String::from("pitch ") + &self.current_scene.current_camera.rot.pitch.to_string()), olc::WHITE);

		Ok(())
	}

	fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
		Ok(())
	}
}

fn main() {
	let mut example = ExampleProgram {
		current_scene: Scene::default_scene(),
		render_index: 0,
		complete: false
	};
	olc::start("Raytracing", &mut example, VIEWPORT_WIDTH as i32, VIEWPORT_HEIGHT as i32, 1, 1).unwrap();
}