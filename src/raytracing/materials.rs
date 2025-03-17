use super::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    vec3::{dot, Vec3},
};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, scattered: &mut Ray) -> bool {
        false
    }

    fn attenuation(&self) -> Color;
    fn clone_box(&self) -> Box<dyn Material>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord, scattered: &mut Ray) -> bool {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal
        }

        scattered.origin = hit_record.point;
        scattered.direction = scatter_direction;
        true
    }

    fn attenuation(&self) -> Color {
        self.albedo
    }

    fn clone_box(&self) -> Box<dyn Material> {
        let material = Lambertian {
            albedo: self.albedo,
        };
        Box::new(material)
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_box()
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz > 1.0 { 1.0 } else { fuzz },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, scattered: &mut Ray) -> bool {
        let mut reflected = ray_in.direction.reflect(hit_record.normal);
        reflected = reflected.unit_vector() + (Vec3::random_unit_vector() * self.fuzz);

        scattered.origin = hit_record.point;
        scattered.direction = reflected;

        dot(scattered.direction, hit_record.normal) > 0.0
    }

    fn attenuation(&self) -> Color {
        self.albedo
    }

    fn clone_box(&self) -> Box<dyn Material> {
        let material = Metal {
            albedo: self.albedo,
            fuzz: self.fuzz,
        };
        Box::new(material)
    }
}
