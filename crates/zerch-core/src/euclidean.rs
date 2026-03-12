pub struct EuclideanDistance {
    pub distance: f32,
}

pub fn euclidean_distance(a: &[f32], b: &[f32]) -> EuclideanDistance {
    if a.len() != b.len() {
        return EuclideanDistance {
            distance: f32::INFINITY,
        };
    }

    #[rustfmt::skip]
    let dist = a
        .iter()
        .zip(b)
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f32>()
        .sqrt();

    EuclideanDistance { distance: dist }
}
