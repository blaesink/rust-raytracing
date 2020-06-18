mod vec3;
mod ray;
use ray::Ray;
use vec3::{Color, Point3,Vec3};

fn hit_sphere(center: Point3, radius: f32, r: &Ray) -> f32 {
    use vec3::dot;
    let oc: Point3 = r.origin() - &center;
    let a: f32 = r.direction().length_squared();
    let half_b: f32 = dot(&oc, &r.direction());
    let c: f32 = oc.length_squared() - radius * radius;

    let discriminant: f32 = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return ((-1.0 * half_b) - discriminant.sqrt()) / a;
    }
}

fn ray_color(r: &Ray) -> Color {
    let t: f32 = hit_sphere(Point3::new(0.0,0.0,-1.0),0.5,&r);

    if t > 0.0 {
        let n: Vec3 = vec3::unit_vector(&(r.at(t) - Vec3::new(0.0,0.0,-1.0)));
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction: Vec3 = vec3::unit_vector(&r.direction());
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);

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

    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = ASPECT_RATIO * viewport_height;
    let focal_length: f32 = 1.0;

    let origin: Point3 = Point3::new(0.0,0.0,0.0);
    let horizontal: Vec3 = Vec3::new(viewport_width,0.0,0.0);
    let vertical: Vec3 = Vec3::new(0.0,viewport_height,0.0);
    let lower_left_corner: Point3 = origin - (horizontal/2.0) - (vertical/2.0) - Vec3::new(0.0,0.0,focal_length);

    println!("P3\n{} {}\n255",WIDTH,HEIGHT);

    let mut j: u16 = HEIGHT - 1;
    while j > 0 {
        for i in 0..WIDTH {
            let u: f32 = i as f32 / (WIDTH - 1) as f32;
            let v: f32 = j as f32 / (HEIGHT - 1) as f32;

            let r: Ray = Ray::new(&origin, &(lower_left_corner + (u * horizontal) + (v * vertical) - origin));
            let pixel_color: Color = ray_color(&r);

            write_color(pixel_color);
        }
        j -= 1;
    }
}
