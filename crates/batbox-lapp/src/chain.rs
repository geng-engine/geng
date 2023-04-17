use super::*;

/// A polygonal chain connecting a vector of points in space
#[derive(Debug, Clone)]
pub struct Chain<T> {
    /// List of points
    pub vertices: Vec<vec2<T>>,
}

impl<T: Float> Chain<T> {
    /// Construct a new chain
    pub fn new(vertices: Vec<vec2<T>>) -> Self {
        Self { vertices }
    }

    /// Returns the total length of the chain
    pub fn length(&self) -> T
    where
        T: std::iter::Sum<T>,
    {
        self.vertices
            .iter()
            .zip(self.vertices.iter().skip(1))
            .map(|(&a, &b)| (a - b).len())
            .sum()
    }

    /// Returns a part of the chain. The full chain's range is `0.0..=1.0`.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// # use batbox_lapp::*;
    /// let chain = Chain::new(vec![vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(1.0, 1.0), vec2(0.0, 1.0)]);
    /// assert_eq!(chain.clone().take_range_ratio(0.0..=1.0).vertices, chain.vertices);
    /// ```
    pub fn take_range_ratio(self, range: RangeInclusive<T>) -> Self
    where
        T: std::iter::Sum<T>,
    {
        let len = self.length();
        let (start, end) = range.into_inner();
        self.take_range_length(start * len..=end * len)
    }

    /// Returns a part of the chain. The full chain's range is `0.0..=chain.length()`.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// # use batbox_lapp::*;
    /// let chain = Chain::new(vec![vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(1.0, 1.0), vec2(0.0, 1.0)]);
    /// assert_eq!(chain.clone().take_range_ratio(0.0..=chain.length()).vertices, chain.vertices);
    /// ```
    pub fn take_range_length(self, range: RangeInclusive<T>) -> Self {
        let &(mut start_len) = range.start();
        let &(mut end_len) = range.end();

        let segments = self.vertices.iter().zip(self.vertices.iter().skip(1));

        let mut start = self.vertices[0];
        let mut start_i = 1;
        for (i, (&a, &b)) in segments.enumerate() {
            let len = (a - b).len();
            start_len -= len;

            if start_len < T::ZERO {
                start = if len.approx_eq(&T::ZERO) {
                    b
                } else {
                    b + (a - b) * (-start_len / len)
                };
                start_i = i + 1;
                break;
            }

            end_len -= len;
        }

        let mut vertices = vec![start];

        for i in start_i..self.vertices.len() {
            let a = self.vertices[i - 1];
            let b = self.vertices[i];
            let len = (a - b).len();
            end_len -= len;

            if end_len <= T::ZERO {
                let end = if len.approx_eq(&T::ZERO) {
                    b
                } else {
                    b + (a - b) * (-end_len / len)
                };
                vertices.push(end);
                break;
            }

            vertices.push(b);
        }

        Self { vertices }
    }

    /// Converts a chain into a vector of segments.
    pub fn segments(&self) -> Vec<Segment<T>> {
        let length = self.vertices.len();
        if length < 2 {
            return vec![];
        }

        let mut segments = Vec::with_capacity(length - 1);
        let mut prev = self.vertices[0];
        for &vertex in self.vertices.iter().skip(1) {
            segments.push(Segment(prev, vertex));
            prev = vertex;
        }
        segments
    }
}
