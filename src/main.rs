const LINES: f32 = 30.0;
const RADIUS: f32 = 3.0;

#[derive(Debug)]
enum CoordinatePair {
    Pair { x: f64, _y: f32 },
}

#[derive(Debug)]
struct CirclePoints {
    left_side: CoordinatePair,
    right_side: CoordinatePair,
}

fn main() {
    let mut points: Vec<CirclePoints> = Vec::new();
    let step = (2.0 * RADIUS) / LINES;
    for i in 0..(LINES as i32) {
        let y = -RADIUS + (i as f32) * step;
        points.push(calculate_points(y as f32, RADIUS));
    }
    let _ = draw_circle(points);
}

fn calculate_points(y: f32, r: f32) -> CirclePoints {
    let x_coordinate_right = f64::sqrt((f32::powf(r, 2.0) as f32 - f32::powf(y, 2.0)) as f64);

    let x_coordinate_left = -&x_coordinate_right;
    CirclePoints {
        left_side: CoordinatePair::Pair {
            x: x_coordinate_left,
            _y: y,
        },
        right_side: CoordinatePair::Pair {
            x: x_coordinate_right,
            _y: y,
        },
    }
}

fn draw_circle(points: Vec<CirclePoints>) {
    let template = "-".repeat((RADIUS as i32 * 20) as usize);
    let mut matrix = vec![template; LINES as usize];
    let mut counter = 0;
    for line in matrix.iter_mut() {
        let CoordinatePair::Pair { x: left_x, .. } = points[counter].left_side;
        let left_x_scaled = left_x * 10.0;
        let left_x_rounded = left_x_scaled.round() + RADIUS as f64 * 10.0;

        let CoordinatePair::Pair { x: right_x, .. } = points[counter].right_side;
        let right_x_scaled = right_x * 10.0;
        let right_x_rounded = right_x_scaled.round() + RADIUS as f64 * 10.0;

        let start = left_x_rounded as usize;
        let end = right_x_rounded as usize;
        if end >= start {
            let fill = "0".repeat(end - start);
            line.replace_range(start..end, &fill);
        }

        println!("{:?}", line);

        counter += 1;
    }
}
