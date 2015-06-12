extern crate vpsearch;

#[derive(Copy, Clone)]
struct Point {
    x: f32, y: f32,
}

impl vpsearch::MetricSpace for Point {
    fn distance(&self, other: &Self) -> vpsearch::Distance {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        return (dx*dx + dy*dy).sqrt(); // sqrt is required
    }
}

fn main() {

    let points = vec![Point{x:2.0,y:3.0}, Point{x:0.0,y:1.0}, Point{x:4.0,y:5.0}];

    let vp = vpsearch::Handle::new(&points);

    let index = vp.find_nearest(&Point{x:1.0,y:2.0});

    println!("The nearest point is at ({}, {})", points[index].x, points[index].y);
}
