mod vec3;
mod ray;
use ray::Ray;
use vec3::{Color, Point3,Vec3};

fn hit_sphere(center: &Point3, radius: f32, r: &Ray) -> bool {
    let oc = r.origin() - center;
    let a = vec3::dot(&r.direction(), &r.direction());
    let b = 2.0 * vec3::dot(&oc, &r.direction());
    let c = vec3::dot(&oc, &oc) - (&radius * &radius);
    let discrim = (b*b) - ((4 as f32) * a * c);

    discrim > 0.0
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0,0.0,-1.0),0.5,&r) {
        return Color::new(1.0,0.0,0.0);
    }
    let unit_direction: Vec3 = vec3::unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Color::new(1.0,1.0,1.0) + t * Color::new(0.5,0.7,1.0)
}

fn write_color(pixel_color: Color) {
   println!("{} {} {}", (255.999 *  pixel_color.x()) as u16,
    (255.999 * pixel_color.y()) as u16,
    (255.999 * pixel_color.z()) as u16);
}

fn main() {
    const ASPECT_RATIO: f32 = 16 as f32 / 9 as f32;
    const WIDTH: u16 = 384;
    const HEIGHT: u16 = (WIDTH as f32 / ASPECT_RATIO) as u16;


    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;
    
    let origin = Point3::new(0.0,0.0,0.0);
    let horizontal = Vec3::new(viewport_width,0.0,0.0);
    let vertical = Vec3::new(0.0,viewport_height,0.0);

    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0,0.0,focal_length);

    println!("P3\n{} {}\n255",WIDTH,HEIGHT);

    let mut j = HEIGHT - 1;
    while j > 0 {
        for i in 0..WIDTH {
            let u = i as f32 / (WIDTH - 1) as f32;
            let v = j as f32 / (HEIGHT - 1) as f32;

            let r: Ray = Ray::new(&lower_left_corner,
                    &(u * horizontal + v * vertical - origin));

            let pixel_color = ray_color(&r);

            write_color(pixel_color);
        }
        j -= 1;
    }
}
