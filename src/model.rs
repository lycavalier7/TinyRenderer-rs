use crate::geometry::Vec3f;
use std::fs::File;
use std::io::{BufRead, BufReader};
pub struct Model {
    verts: Vec<Vec3f>,
    faces: Vec<[usize; 3]>,
}

impl Model {
    pub fn new(verts: Vec<Vec3f>, faces: Vec<[usize; 3]>) -> Self {
        Self { verts, faces }
    }
    pub fn load_obj(path: &str) -> Result<Model, String> {
        let file = File::open(path).map_err(|e| format!("Fail to open {}: {}", path, e))?;

        let reader = BufReader::new(file);
        let mut model = Model::new(Vec::new(), Vec::new());

        for line_result in reader.lines() {
            match line_result {
                Ok(line) => {
                    if let Err(e) = model.parse_obj_line(&line) {
                        eprintln!("Failed to parse line `{}`: {}", line, e);
                    }
                }
                Err(e) => eprintln!("Failed to read line: {}", e),
            }
        }

        Ok(model)
    }

    fn parse_obj_line(&mut self, line: &str) -> std::result::Result<(), String> {
        let line = line.trim();

        if line.is_empty() {
            return Ok(());
        }

        if line.starts_with('#') {
            return Ok(());
        }

        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts[0] {
            "v" => self.parse_vertex(&parts[1..]),
            "f" => self.parse_face(&parts[1..]),
            _ => Ok(()),
        }
    }
    fn parse_vertex(&mut self, vertices: &[&str]) -> std::result::Result<(), String> {
        if vertices.len() != 3 {
            return Err("Vertex must have 3 values".to_string());
        }

        let x = vertices[0].parse::<f32>().map_err(|e| e.to_string())?;
        let y = vertices[1].parse::<f32>().map_err(|e| e.to_string())?;
        let z = vertices[2].parse::<f32>().map_err(|e| e.to_string())?;

        self.verts.push(Vec3f{x: x, y: y, z: z});

        Ok(())
    }

    fn parse_face(&mut self, faces: &[&str]) -> std::result::Result<(), String> {
        if faces.len() != 3 {
            return Err("Just support triangle".to_string());
        }

        let mut idx = [0usize; 3];

        for (i, f) in faces.iter().enumerate() {
            let v = f.split('/')
                .next()
                .ok_or("Invalid face")?
                .parse::<usize>()
                .map_err(|e| e.to_string())?;
            
            if v < 1 || v > self.verts.len() {
                return Err("Invalid index".to_string());
            }
            idx[i] = v - 1;
        }

        self.faces.push(idx);

        Ok(())
    }
    
    pub fn nfaces(&self) -> usize {
        self.faces.len()
    }
    
    pub fn face(&self, i: usize) -> Result<[usize; 3], String> {
        self.faces.get(i).copied().ok_or_else(|| "Invalid face index".to_string())
    }
    
    pub fn vert(&self, i: usize) -> Result<Vec3f, String> {
        self.verts.get(i).copied().ok_or_else(|| "Invalid vertex index".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn new_stores_verts_and_faces() {
        let verts = vec![
            Vec3f { x: 1.0, y: 2.0, z: 3.0 },
            Vec3f { x: -1.0, y: 0.5, z: 4.0 },
        ];
        let faces = vec![[0, 1, 0]];

        let model = Model::new(verts, faces);

        assert_eq!(model.verts.len(), 2);
        assert_eq!(model.faces.len(), 1);
        assert!(approx_eq(model.verts[0].x, 1.0));
        assert!(approx_eq(model.verts[0].y, 2.0));
        assert!(approx_eq(model.verts[0].z, 3.0));
        assert_eq!(model.faces[0], [0, 1, 0]);
    }

    #[test]
    fn parse_obj_line_ignores_empty_comment_and_unknown() {
        let mut model = Model::new(Vec::new(), Vec::new());

        model.parse_obj_line("").unwrap();
        model.parse_obj_line("    ").unwrap();
        model.parse_obj_line("# comment").unwrap();
        model.parse_obj_line("vt 0.1 0.2").unwrap();

        assert!(model.verts.is_empty());
        assert!(model.faces.is_empty());
    }

    #[test]
    fn parse_vertex_success_and_invalid_arity() {
        let mut model = Model::new(Vec::new(), Vec::new());

        model.parse_vertex(&["1.5", "-2.0", "3.25"]).unwrap();
        assert_eq!(model.verts.len(), 1);
        assert!(approx_eq(model.verts[0].x, 1.5));
        assert!(approx_eq(model.verts[0].y, -2.0));
        assert!(approx_eq(model.verts[0].z, 3.25));

        let err = model.parse_vertex(&["1.0", "2.0"]).unwrap_err();
        assert_eq!(err, "Vertex must have 3 values");
    }

    #[test]
    fn parse_face_success_with_obj_style_tokens() {
        let mut model = Model::new(
            vec![
                Vec3f { x: 0.0, y: 0.0, z: 0.0 },
                Vec3f { x: 1.0, y: 0.0, z: 0.0 },
                Vec3f { x: 0.0, y: 1.0, z: 0.0 },
            ],
            Vec::new(),
        );

        model.parse_face(&["1/10/20", "2/11/21", "3/12/22"]).unwrap();

        assert_eq!(model.faces, vec![[0, 1, 2]]);
    }

    #[test]
    fn parse_face_rejects_non_triangle() {
        let mut model = Model::new(Vec::new(), Vec::new());

        let err = model.parse_face(&["1", "2", "3", "4"]).unwrap_err();
        assert_eq!(err, "Just support triangle");
        assert!(model.faces.is_empty());
    }

    #[test]
    fn parse_obj_line_reports_face_parse_error() {
        let mut model = Model::new(Vec::new(), Vec::new());

        let err = model.parse_obj_line("f a b c").unwrap_err();
        assert!(!err.is_empty());
        assert!(model.faces.is_empty());
    }

    #[test]
    fn load_obj_parses_vertices_faces_and_skips_bad_lines() {
        let mut path: PathBuf = std::env::temp_dir();
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        path.push(format!("tiny_renderer_model_test_{}.obj", unique));

        let content = "\
# comment
v 0.0 0.0 0.0
v 1.0 0.0 0.0
v 0.0 1.0 0.0
v 1.0 2.0
f 1/1/1 2/2/2 3/3/3
f 1 2
";
        fs::write(&path, content).unwrap();

        let path_str = path.to_str().unwrap();
        let model = Model::load_obj(path_str).unwrap();

        assert_eq!(model.verts.len(), 3);
        assert_eq!(model.faces.len(), 1);
        assert_eq!(model.faces[0], [0, 1, 2]);

        fs::remove_file(path).unwrap();
    }
}
