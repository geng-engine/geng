use batbox_la::*;
use std::collections::HashMap;

#[derive(ugli::Vertex, Debug, Copy, Clone)]
pub struct Vertex {
    pub a_v: vec3<f32>,
    pub a_vt: vec2<f32>,
    pub a_vn: vec3<f32>,
}

pub fn parse(source: &str) -> HashMap<String, Vec<Vertex>> {
    let mut result = HashMap::new();

    let mut current_name = String::from("__unnamed__");

    let mut v = Vec::new();
    let mut vn = Vec::new();
    let mut vt = Vec::new();
    let mut current_obj = Vec::new();
    for line in source.lines().chain(std::iter::once("o _")) {
        if line.starts_with("v ") {
            let mut parts = line.split_whitespace();
            parts.next();
            let x: f32 = parts.next().unwrap().parse().unwrap();
            let y: f32 = parts.next().unwrap().parse().unwrap();
            let z: f32 = parts.next().unwrap().parse().unwrap();
            v.push(vec3(x, y, z));
        } else if line.starts_with("vn ") {
            let mut parts = line.split_whitespace();
            parts.next();
            let x: f32 = parts.next().unwrap().parse().unwrap();
            let y: f32 = parts.next().unwrap().parse().unwrap();
            let z: f32 = parts.next().unwrap().parse().unwrap();
            vn.push(vec3(x, y, z));
        } else if line.starts_with("vt ") {
            let mut parts = line.split_whitespace();
            parts.next();
            let x: f32 = parts.next().unwrap().parse().unwrap();
            let y: f32 = parts.next().unwrap().parse().unwrap();
            vt.push(vec2(x, y));
        } else if line.starts_with("f ") {
            let mut parts = line.split_whitespace();
            parts.next();
            let to_vertex = |s: &str| {
                let mut parts = s.split('/');
                let i_v: usize = parts.next().unwrap().parse().unwrap();
                let i_vt: usize = parts.next().unwrap().parse().unwrap();
                let i_vn: usize = parts.next().unwrap().parse().unwrap();
                Vertex {
                    a_v: v[i_v - 1],
                    a_vn: vn[i_vn - 1],
                    a_vt: vt[i_vt - 1],
                }
            };
            let mut cur = Vec::new();
            for s in parts {
                cur.push(to_vertex(s));
            }
            for i in 2..cur.len() {
                current_obj.push(cur[0]);
                current_obj.push(cur[i - 1]);
                current_obj.push(cur[i]);
            }
        } else if line.starts_with("o ") || line.starts_with("g ") {
            if !current_obj.is_empty() {
                result.insert(current_name.clone(), current_obj);
                current_obj = Vec::new();
            }
            current_name = String::from(&line[2..]);
        }
    }
    result
}

pub fn recalculate_normals(data: &mut [Vertex]) {
    for face in data.chunks_mut(3) {
        let n = vec3::cross(face[1].a_v - face[0].a_v, face[2].a_v - face[0].a_v).normalize();
        for v in face {
            v.a_vn = n;
        }
    }
}

pub fn unitize<'a, I: Iterator<Item = &'a mut [Vertex]>>(iter: I) {
    let vss: Vec<&'a mut [Vertex]> = iter.collect();
    const INF: f32 = 1e9;
    let mut min_x: f32 = INF;
    let mut max_x: f32 = -INF;
    let mut min_y: f32 = INF;
    let mut max_y: f32 = -INF;
    for vs in &vss {
        for v in vs.iter() {
            let x = v.a_v.x;
            let y = v.a_v.y;
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }
    }
    let center = vec3(min_x + max_x, min_y + max_y, 0.0) / 2.0;
    let div = (max_y - min_y).max(max_x - min_x) / 2.0;
    for vs in vss {
        for v in vs.iter_mut() {
            v.a_v = (v.a_v - center) / div;
        }
    }
}

pub fn scale(data: &mut [Vertex], k: f32) {
    for v in data {
        v.a_v *= k;
    }
}

pub fn united(data: HashMap<String, Vec<Vertex>>) -> Vec<Vertex> {
    let mut result = Vec::new();
    for part in data.values() {
        result.extend(part);
    }
    result
}
