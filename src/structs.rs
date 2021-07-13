//
//	Data Structures
//

use uuid::Uuid;
use std::cell::RefCell;
use core::any::Any;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl Vec3 {
	pub fn sub(&self, other: &Vec3) -> Vec3 {
		Self { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
	}

	pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
		Self { x: (u.y * v.z) - (u.z * v.y), y: (u.z * v.x) - (u.x * v.z) , z: (u.x * v.y) - (u.y * v.x) }
	}

	pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
		u.x * v.x + u.y * v.y + u.z * v.z 
	}

	pub fn mul(&self, v: f64) -> Vec3 {
		Vec3 { x: self.x * v, y: self.y * v, z: self.z * v }
	}

	pub fn div(&self, v: f64) -> Vec3 {
		Vec3 { x: self.x / v, y: self.y / v, z: self.z / v }
	}

	pub fn add(&self, other: &Vec3) -> Vec3 {
		Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
	}

	pub fn dist(&self, other: &Vec3) -> f64 {
		((other.x - self.x) * (other.x - self.x) + (other.y - self.y) * (other.y - self.y) + (other.z - self.z) * (other.z - self.z)).sqrt()
	}

	pub fn normalize(&self) -> Vec3 {
		let len_squared = self.x * self.x + self.y * self.y + self.z * self.z;
		if len_squared > 0.0 {
			let inv = 1.0 / len_squared.sqrt();
			return Vec3 { x: self.x * inv, y: self.y * inv, z: self.z * inv };
		}
		return self.clone();
	}

	pub fn rotate(&self, rot: &Rot3) -> Vec3 {
        let su = rot.roll.sin();
        let cu = rot.roll.cos();
        let sv = rot.pitch.sin();
        let cv = rot.pitch.cos();
        let sw = rot.yaw.sin();
        let cw = rot.yaw.cos();

		let r11 = cv*cw;
        let r12 = su*sv*cw - cu*sw;
        let r13 = su*sw + cu*sv*cw;
        let r21 = cv*sw;
        let r22 = cu*cw + su*sv*sw;
        let r23 = cu*sv*sw - su*cw;
        let r31 = -sv;
        let r32 = su*cv;
        let r33 = cu*cv; 
		let x = r11* self.x + r12 * self.y + r13 * self.z;
		let y = r21* self.x + r22 * self.y + r23 * self.z;
		let z = r31* self.x + r32 * self.y + r33 * self.z;

		Vec3 { x, y, z }
	}
}

pub struct Rot3 {
	pub yaw: f64,
	pub pitch: f64,
	pub roll: f64,
}

impl Rot3 {
	pub fn new() -> Self {
		Self {
			yaw: 0.0,
			pitch: 0.0,
			roll: 0.0
		}
	}

	pub fn to_vec(rot: &Rot3) -> Vec3 {
		Vec3 { x: rot.yaw.cos() * rot.pitch.cos(), y: rot.yaw.sin() * rot.pitch.cos(), z: rot.pitch.sin() }
	}
}

pub struct Tri {
	pub a: Vec3,
	pub b: Vec3,
	pub c: Vec3,
	pub mat: Material
}

impl Tri {
	pub fn transformed_rot(&self, rot: &Rot3) -> Tri {
		Tri {
			a: self.a.rotate(&rot),
			b: self.b.rotate(&rot),
			c: self.c.rotate(&rot),
			mat: self.mat
		}
	}

	pub fn transformed_pos(&self, pos: &Vec3) -> Tri {
		Tri {
			a: Vec3 { x: self.a.x + pos.x, y: self.a.y + pos.y, z: self.a.z + pos.z },
			b: Vec3 { x: self.b.x + pos.x, y: self.b.y + pos.y, z: self.b.z + pos.z },
			c: Vec3 { x: self.c.x + pos.x, y: self.c.y + pos.y, z: self.c.z + pos.z },
			mat: self.mat
		}
	}

	pub fn normal(&self) -> Vec3 {
		let u = self.b.sub(&self.a);
		let v = self.c.sub(&self.a);
		Vec3 { x: u.y*v.z - u.z * v.y, y: u.z * v.x - u.z * v.z, z: u.z * v.y - u.y * v.x }
	}

	pub fn transformed(&self, pos: &Vec3, rot: &Rot3) -> Tri {
		return self.transformed_rot(rot).transformed_pos(pos);
	}

	pub fn ray_hit(&self, ray: &Ray) -> Option<Vec3> {
		let epsilon = 0.0000001;
		let edge1 = self.b.sub(&self.a); 
		let edge2 = self.c.sub(&self.a);
		let ray_dir_edge2 = Vec3::cross(&ray.direction, &edge2);
		let det = Vec3::dot(&edge1, &ray_dir_edge2);
		if det > -epsilon && det < epsilon { return None }
		let inv_det = 1.0 / det;
		let orig_minus_a = ray.origin.sub(&self.a);
		let barymetric_u = Vec3::dot(&ray_dir_edge2, &orig_minus_a) * inv_det;
		if barymetric_u < 0.0 || barymetric_u > 1.0 { return None }
		let cross_oma_a = Vec3::cross(&orig_minus_a, &edge1);
		let barymetric_v = Vec3::dot(&ray.direction, &cross_oma_a) * inv_det;
		if barymetric_v < 0.0 || barymetric_v + barymetric_u > 1.0 { return None }
		let ray_t = Vec3::dot(&edge2, &cross_oma_a) * inv_det;
		if ray_t < epsilon { return None }
		return Some(Vec3::add(&ray.origin, &ray.direction.mul(ray_t)));
	}
}

pub trait SceneObject {
	fn get_pos(&self) -> &Vec3;
	fn get_rot(&self) -> &Rot3;
	fn ray_hit(&self, ray: &Ray) -> Option<(Vec3, Material, Vec3)>;
	fn as_any(&mut self) -> &mut dyn Any;
	fn as_any_immut(&self) -> &dyn Any;
	fn get_id(&self) -> &String;
}

pub struct Mesh {
	pub anchor: Vec3,
	pub rot: Rot3,
	pub tri_list: Vec<Tri>,
	pub id: String
}

impl Mesh {
	pub fn new(anchor: Vec3, rot: Rot3, tris: Vec<Tri>) -> Self {
		Mesh {
			anchor,
			rot,
			tri_list: tris,
			id: Uuid::new_v4().to_hyphenated().to_string()
		}
	}
}

impl SceneObject for Mesh {
	fn get_pos(&self) -> &Vec3 { return &self.anchor }
	fn get_rot(&self) -> &Rot3 { return &self.rot }
	fn ray_hit(&self, ray: &Ray) -> Option<(Vec3, Material, Vec3)> { 
		let mut min = f64::MAX;
		let mut final_val = None;
		let mut final_tri = None;
		for tri in &self.tri_list {
			let tr = tri.transformed(&self.get_pos(), &self.get_rot());
			let dist = tr.ray_hit(&ray);
			if dist.is_some() {
				let val = ray.origin.dist(&dist.unwrap());
				if val > 0.01 {
					if val < min { min = val; final_val = Some(dist.unwrap()); final_tri = Some(tr) }
				}
				
			}
		}
		if final_tri.is_some() {
			let trr = final_tri.unwrap();
			if min == f64::MAX { return None } else { return Some((final_val.unwrap(), trr.mat, trr.normal())) }
		}
		return None
	}
	fn as_any(&mut self) -> &mut dyn Any { self }
	fn as_any_immut(&self) -> &dyn Any { self }
	fn get_id(&self) -> &String { &self.id }
}

pub struct Sphere {
	pub center: Vec3,
	pub radius: f32,
	pub rot: Rot3,
	pub material: Material,
	pub id: String,
}

impl Sphere {
	pub fn new(pos: Vec3, rad: f32, mat: Material) -> Self {
		Sphere {
			center: pos,
			radius: rad,
			material: mat,
			rot: Rot3::new(),
			id: Uuid::new_v4().to_hyphenated().to_string()
		}
	}
}


impl SceneObject for Sphere {
	fn get_pos(&self) -> &Vec3 { return &self.center }
	fn get_rot(&self) -> &Rot3 { return &self.rot }
	fn ray_hit(&self, ray: &Ray) -> Option<(Vec3, Material, Vec3)> { 
		let oc = ray.origin.sub(&self.center);
		let oc_d = Vec3::dot(&oc, &ray.direction);
		if oc_d > 0.0 || Vec3::dot(&oc, &oc) < (self.radius * self.radius) as f64 { return None }
		let a = oc.sub(&ray.direction.mul(oc_d));
		let adot = Vec3::dot(&a, &a);
		if adot > (self.radius * self.radius) as f64 { return None }
		let h = (((self.radius * self.radius) as f64) - adot).sqrt();
		let i = a.sub(&ray.direction.mul(h));
		let intersection = self.center.add(&i);
		Some((intersection, self.material, i.div(self.radius as f64)))
	}
	fn as_any(&mut self) -> &mut dyn Any { self }
	fn as_any_immut(&self) -> &dyn Any { self }
	fn get_id(&self) -> &String { &self.id }
}

#[derive(Clone, Copy)]
pub struct Material {
	pub transparency: f32,
	pub reflectivity: f32,
	pub color: Color
}

#[derive(Clone, Copy)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

pub struct LightSource {
	pub pos: Vec3,
	pub rot: Rot3,
	pub intensity: f32,
	pub color: Color,
	pub id: String
}

impl LightSource {
	pub fn new(pos: Vec3, rot: Rot3, intensity: f32) -> Self {
		LightSource {
			pos,
			rot,
			intensity,
			color: Color {r: 255, g: 255, b: 255 },
			id: Uuid::new_v4().to_hyphenated().to_string()
		}
	}
}

pub struct Ray {
	pub origin: Vec3,
	pub direction: Vec3,
}

impl Ray {
	pub fn from_to(origin: &Vec3, destination: &Vec3) -> Self {
		Self {
			origin: origin.clone(),
			direction: destination.sub(&origin).normalize()
		}
	}

	pub fn nudge(&mut self) {
		self.origin = self.origin.add(&self.direction)
	}
}

impl SceneObject for LightSource {
	fn get_pos(&self) -> &Vec3 { return &self.pos }
	fn get_rot(&self) -> &Rot3 { return &self.rot }
	fn ray_hit(&self, _ray: &Ray) -> Option<(Vec3, Material, Vec3)> { return None }
	fn as_any(&mut self) -> &mut dyn Any { self }
	fn as_any_immut(&self) -> &dyn Any { self }
	fn get_id(&self) -> &String { &self.id }
}

pub struct Camera {
	pub pos: Vec3,
	pub rot: Rot3,
	pub fov: u16,
	pub id: String
}

impl Camera {
	pub fn new(pos: Vec3, rot: Rot3, fov: u16) -> Self {
		Camera {
			pos,
			rot,
			fov,
			id: Uuid::new_v4().to_hyphenated().to_string()
		}
	}
}

impl SceneObject for Camera {
	fn get_pos(&self) -> &Vec3 { return &self.pos }
	fn get_rot(&self) -> &Rot3 { return &self.rot }
	fn ray_hit(&self, _ray: &Ray) -> Option<(Vec3, Material, Vec3)> { return None; }
	fn as_any(&mut self) -> &mut dyn Any { self }
	fn as_any_immut(&self) -> &dyn Any { self }
	fn get_id(&self) -> &String { &self.id }
}

pub struct Scene {
	pub objects: Vec<Box<dyn SceneObject>>,
	pub current_camera: Box<Camera>
}

fn create_cube(center: Vec3, rot: Rot3) -> Mesh {
	let mut tris: Vec<Tri> = Vec::new();
	let white_difuse = Material {
		color: Color {r: 255, g: 255, b: 255},
		transparency: 0.0,
		reflectivity: 0.0
	};

	let funky = Material {
		color: Color {r: 255, g: 10, b: 255},
		transparency: 0.0,
		reflectivity: 0.0
	};

	tris.push(Tri { a: {Vec3 {x: -1.0, y: -1.0, z: -1.0}}, b: {Vec3 {x: -1.0, y: -1.0, z: 1.0}}, c: {Vec3 {x: -1.0, y: 1.0, z: 1.0}}, mat: funky });
	tris.push(Tri { a: {Vec3 {x: 1.0, y: 1.0, z: -1.0}}, b: {Vec3 {x: -1.0, y: -1.0, z: -1.0}}, c: {Vec3 {x: -1.0, y: 1.0, z: -1.0}}, mat: white_difuse });

	tris.push(Tri { a: {Vec3 {x: 1.0, y: -1.0, z: 1.0}}, b: {Vec3 {x: -1.0, y: -1.0, z: -1.0}}, c: {Vec3 {x: 1.0, y: -1.0, z: -1.0}}, mat: white_difuse });
	tris.push(Tri { a: {Vec3 {x: 1.0, y: 1.0, z: -1.0}}, b: {Vec3 {x: -1.0, y: -1.0, z: -1.0}}, c: {Vec3 {x: 1.0, y: -1.0, z: -1.0}}, mat: funky });

	tris.push(Tri { a: {Vec3 {x: -1.0, y: -1.0, z: -1.0}}, b: {Vec3 {x: -1.0, y: 1.0, z: 1.0}}, c: {Vec3 {x: -1.0, y: 1.0, z: -1.0}}, mat: funky });
	tris.push(Tri { a: {Vec3 {x: 1.0, y: -1.0, z: 1.0}}, b: {Vec3 {x: -1.0, y: -1.0, z: 1.0}}, c: {Vec3 {x: -1.0, y: -1.0, z: -1.0}}, mat: white_difuse });

	tris.push(Tri { a: {Vec3 {x: -1.0, y: 1.0, z: 1.0}}, b: {Vec3 {x: -1.0, y: -1.0, z: 1.0}}, c: {Vec3 {x: 1.0, y: -1.0, z: 1.0}}, mat: funky });
	tris.push(Tri { a: {Vec3 {x: 1.0, y: 1.0, z: 1.0}}, b: {Vec3 {x: -1.0, y: 1.0, z: 1.0}}, c: {Vec3 {x: 1.0, y: -1.0, z: 1.0}}, mat: white_difuse });

	tris.push(Tri { a: {Vec3 {x: 1.0, y: 1.0, z: 1.0}}, b: {Vec3 {x: 1.0, y: -1.0, z: -1.0}}, c: {Vec3 {x: 1.0, y: 1.0, z: -1.0}}, mat: funky });
	tris.push(Tri { a: {Vec3 {x: 1.0, y: -1.0, z: -1.0}}, b: {Vec3 {x: 1.0, y: 1.0, z: 1.0}}, c: {Vec3 {x: 1.0, y: -1.0, z: 1.0}}, mat: white_difuse });

	tris.push(Tri { a: {Vec3 {x: 1.0, y: 1.0, z: 1.0}}, b: {Vec3 {x: 1.0, y: 1.0, z: -1.0}}, c: {Vec3 {x: -1.0, y: 1.0, z: -1.0}}, mat: funky });
	tris.push(Tri { a: {Vec3 {x: 1.0, y: 1.0, z: 1.0}}, b: {Vec3 {x: -1.0, y: 1.0, z: -1.0}}, c: {Vec3 {x: -1.0, y: 1.0, z: 1.0}}, mat: white_difuse });

	Mesh::new(center, rot, tris)
}

fn create_big_plane(center: Vec3, rot: Rot3) -> Mesh {
	let mut tris: Vec<Tri> = Vec::new();
	let white_difuse = Material {
		color: Color {r: 255, g: 255, b: 255},
		transparency: 0.0,
		reflectivity: 0.0
	};

	tris.push(Tri { a: {Vec3 {x: 4.0, y: 4.0, z: 0.0}}, b: {Vec3 {x: -4.0, y: 4.0, z: 0.0}}, c: {Vec3 {x: 4.0, y: -4.0, z: 0.0}}, mat: white_difuse });
	tris.push(Tri { a: {Vec3 {x: -4.0, y: 4.0, z: 0.0}}, b: {Vec3 {x: -4.0, y: -4.0, z: 0.0}}, c: {Vec3 {x: 4.0, y: -4.0, z: 0.0}}, mat: white_difuse });

	Mesh::new(center, rot, tris)
}

fn deg_to_rad(deg: f64) -> f64 {
	(std::f64::consts::PI / 180.0) * deg
}

fn capped_f64(v: f64, floor: f64, max: f64) -> f64 {
	if v < floor { return floor }
	if v > max { return max }
	v
}

impl Scene {
	pub fn default_scene() -> Self {
		let mut objects: Vec<Box<dyn SceneObject>> = Vec::new();

		let white_difuse = Material {
			color: Color {r: 255, g: 255, b: 255},
			transparency: 0.0,
			reflectivity: 0.0
		};
	
		let camera = Box::new(Camera::new(
			Vec3 { x: 3.0, y: 3.0, z: 3.0 }, // pos
			Rot3 { pitch: deg_to_rad(0.0), yaw: -3.0, roll: 1.5 }, // rot
			40 // fov
		));

		let mut light_souce = Box::new(LightSource::new(
			Vec3 { x: -1.0, y: -1.0, z: 2.0 },
			Rot3::new(),
			10.0,
		));
		light_souce.id = String::from("fuckin' light");
		objects.push(light_souce);

		//let light_souce2 = Box::new(LightSource::new(
		//	Vec3 { x: 1.5, y: 1.5, z: 3.0 },
		//	Rot3::new(),
		//	10.0,
		//));
		//objects.push(light_souce2);

		let mut default_cube = Box::new(create_cube(Vec3 { x: 0.0, y: 0.0, z: 1.5 }, Rot3 { pitch: deg_to_rad(0.0), yaw: deg_to_rad(30.0), roll: deg_to_rad(60.0) }));
		default_cube.id = String::from("fuckin' cube");
		objects.push(default_cube);
		let plane = Box::new(create_big_plane(Vec3 { x: 0.0, y: 0.0, z: 0.0 }, Rot3::new()));
		objects.push(plane);
		//let sphere = Box::new(Sphere::new(Vec3 { x: 0.0, y: 0.0, z: 1.5 }, 1.0, white_difuse));
		//objects.push(sphere);
		//let sphere = Box::new(Sphere::new(Vec3 { x: 1.2, y: 1.2, z: 2.3 }, 0.4, white_difuse));
		//objects.push(sphere);

		Self {
			objects,
			current_camera: camera
		}
	}

	pub fn get_all_light_sources(&mut self) -> Vec<&mut LightSource> {
		let mut res = Vec::new();
		for object in self.objects.iter_mut() {
			let any_v = object.as_any();
			if let Some(hit) = any_v.downcast_mut::<LightSource>(){
				res.push(hit);
			}
		}
		return res;
	}

	pub fn get_all_meshes(&mut self) -> Vec<&mut Mesh> {
		let mut res = Vec::new();
		for object in self.objects.iter_mut() {
			let any_v = object.as_any();
			if let Some(hit) = any_v.downcast_mut::<Mesh>(){
				res.push(hit);
			}
		}
		return res;
	}

	pub fn get_all_light_sources_immut(&self) -> Vec<&LightSource> {
		let mut res = Vec::new();
		for object in self.objects.iter() {
			if let Some(hit) = object.as_any_immut().downcast_ref::<LightSource>(){
				res.push(hit);
			}
		}
		return res;
	}

	pub fn get_all_meshes_immut(&self) -> Vec<&Mesh> {
		let mut res = Vec::new();
		for object in self.objects.iter() {
			let any_v = object.as_any_immut();
			if let Some(hit) = any_v.downcast_ref::<Mesh>(){
				res.push(hit);
			}
		}
		return res;
	}

	pub fn trace(&self, ray: &Ray) -> Option<(Vec3, Material, Vec3)> {
		let mut closest_intersect = None;
		for object in self.objects.iter() {
			let intersect_opt = object.ray_hit(&ray);
			if let Some(intersect) = intersect_opt {
				if closest_intersect.is_none() { 
					closest_intersect = Some(intersect);
					continue;
				}
				if self.current_camera.pos.dist(&intersect.0) < self.current_camera.pos.dist(&closest_intersect.unwrap().0) {
					closest_intersect = Some(intersect);
				}
			}
		}
		return closest_intersect
	}

	pub fn cast_ray(&mut self, index: u64, width: i32, height: i32) -> Color {
		let x = index as i32 % width;
		let y = index as i32 / width;

		let aspect_ratio = width as f32 / height as f32;
		let inv_width = 1.0 / width as f32;
		let inv_height = 1.0 / height as f32;
		let angle = (std::f32::consts::PI * 0.5 * (self.current_camera.fov as f32) / 180.0).tan(); 
		let xx = (2.0 * ((x as f32 + 0.5) * inv_width) - 1.0) * angle * aspect_ratio; 
		let yy = (1.0 - 2.0 * ((y as f32 + 0.5) * inv_height as f32)) * angle;
		let direction = (Vec3 {x: xx as f64, y: yy as f64, z: -1.0}).normalize().rotate(&self.current_camera.rot);
		let ray = Ray { origin: self.current_camera.pos, direction };

		let mut mix_color = Color {
			r: 0 as u8,
			g: 0 as u8,
			b: 0 as u8,
		};

		let hit = self.trace(&ray);
		if let Some(hit) = hit {
			// Cast Shadow Ray
			let light_sources = self.get_all_light_sources_immut();
			for ls in light_sources.iter() {
				let shadow_ray = Ray::from_to(&hit.0, &ls.pos);

				if let Some(shadow_hit) = self.trace(&shadow_ray) {
					let luminosity = 0.22 / (hit.0.dist(&ls.pos) * hit.0.dist(&ls.pos)); // Inverse Square Law
					mix_color = Color {
						r: capped_f64( ls.color.r as f64 * luminosity + hit.1.color.r as f64 * luminosity as f64, 0.0, 255.0) as u8,
						g: capped_f64( ls.color.g as f64 * luminosity + hit.1.color.g as f64 * luminosity as f64, 0.0, 255.0) as u8,
						b: capped_f64( ls.color.b as f64 * luminosity + hit.1.color.b as f64 * luminosity  as f64, 0.0, 255.0) as u8,
					}
				} else {
					let luminosity = 1.0 / (hit.0.dist(&ls.pos) * hit.0.dist(&ls.pos)); // Inverse Square Law
					mix_color = Color {
						r: capped_f64( ls.color.r as f64 * luminosity + hit.1.color.r as f64 * luminosity as f64, 0.0, 255.0) as u8,
						g: capped_f64( ls.color.g as f64 * luminosity + hit.1.color.g as f64 * luminosity as f64, 0.0, 255.0) as u8,
						b: capped_f64( ls.color.b as f64 * luminosity + hit.1.color.b as f64 * luminosity  as f64, 0.0, 255.0) as u8,
					}
				}
			}

			// Cast Reflect Rays
			let reflect_ray = Ray { origin: hit.0, direction: hit.2 };
			if let Some(reflect_hit) = self.trace(&reflect_ray) {
				let light_sources = self.get_all_light_sources_immut();
				for ls in light_sources.iter() {
					let shadow_ray = Ray::from_to(&hit.0, &ls.pos);
	
					if let Some(shadow_hit) = self.trace(&shadow_ray) {
						let luminosity = 0.22 / (hit.0.dist(&ls.pos) * hit.0.dist(&ls.pos)); // Inverse Square Law
						mix_color = Color {
							r: capped_f64( ls.color.r as f64 * luminosity + reflect_hit.1.color.r as f64 * luminosity as f64, 0.0, 255.0) as u8,
							g: capped_f64( ls.color.g as f64 * luminosity + reflect_hit.1.color.g as f64 * luminosity as f64, 0.0, 255.0) as u8,
							b: capped_f64( ls.color.b as f64 * luminosity + reflect_hit.1.color.b as f64 * luminosity  as f64, 0.0, 255.0) as u8,
						}
					} else {
						let luminosity = 1.0 / (hit.0.dist(&ls.pos) * hit.0.dist(&ls.pos)); // Inverse Square Law
						mix_color = Color {
							r: capped_f64( ls.color.r as f64 * luminosity + mix_color.r as f64, 0.0, 255.0) as u8,
							g: capped_f64( ls.color.g as f64 * luminosity + mix_color.g as f64, 0.0, 255.0) as u8,
							b: capped_f64( ls.color.b as f64 * luminosity + mix_color.b as f64, 0.0, 255.0) as u8,
						}
					}
				}
			} else {
				
			}
		} 

		mix_color
	}
}

#[test]
fn tri_hit() {
	let white_difuse = Material {
		color: Color {r: 255, g: 255, b: 255},
		transparency: 0.0,
		reflectivity: 0.0
	};
	let tri = Tri { a: Vec3 {x: -1.0, y: 0.0, z: 0.0}, b: Vec3 {x: 0.0, y: 1.0, z: 0.0}, c: Vec3 {x: 1.0, y: 0.0, z: 0.0}, mat: white_difuse};
	let ray = Ray { origin: Vec3 {x: 0.0, y: 0.33, z: 1.0}, direction: Vec3 { x: 0.0, y: 0.0, z: -1.0 }};
	let dist = tri.ray_hit(&ray);
	assert_eq!(dist.is_some(), true);

	let origin = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
	let up = Vec3 { x: 0.0, y: 0.0, z: 1.0 };
	let right = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
	assert_eq!(origin.dist(&up), 1.0);
	assert_eq!(origin.dist(&right), 1.0);
}