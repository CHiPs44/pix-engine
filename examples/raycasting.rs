use pix_engine::prelude::*;
use std::{borrow::Cow, cmp::Ordering::Less};

const TITLE: &str = "Raycasting";
const WIDTH: u32 = 1000;
const HEIGHT: u32 = 800;
const SCALE: u32 = 1;

const BLOCK_SIZE: u32 = 40;
const NORTH: usize = 0;
const SOUTH: usize = 1;
const EAST: usize = 2;
const WEST: usize = 3;

struct RayScene {
    cells: Vec<Cell>,
    edges: Vec<Line<f64>>,
    points: Vec<Point<f64>>,
    polygons: Vec<(f64, Point<f64>)>,
    xcells: u32,
    ycells: u32,
    drawing: bool,
    light: Option<Image>,
}

impl RayScene {
    fn new() -> Self {
        let xcells = WIDTH / (BLOCK_SIZE * SCALE);
        let ycells = HEIGHT / (BLOCK_SIZE * SCALE);
        let mut cells = Vec::with_capacity((xcells * ycells) as usize);
        for y in 0..ycells {
            for x in 0..xcells {
                cells.push(Cell::new([x as f64, y as f64]));
            }
        }
        Self {
            cells,
            edges: Vec::with_capacity(20),
            points: Vec::with_capacity(40),
            polygons: Vec::new(),
            xcells,
            ycells,
            drawing: false,
            light: None,
        }
    }

    fn get_cell_index(&self, x: i32, y: i32) -> usize {
        ((y / BLOCK_SIZE as i32) * self.xcells as i32 + (x / BLOCK_SIZE as i32)) as usize
    }

    fn exists(&self, i: usize) -> bool {
        self.cells.get(i).map(|c| c.exists).unwrap_or(false)
    }

    fn has_edge(&self, i: usize, dir: usize) -> bool {
        self.cells.get(i).map(|c| c.edges[dir].0).unwrap_or(false)
    }
    fn get_edge_index(&mut self, i: usize, dir: usize) -> PixResult<usize> {
        self.cells
            .get(i)
            .map(|c| c.edges[dir].1)
            .ok_or_else(|| PixError::Other(Cow::from("invalid cell index")))
    }
    fn get_edge_mut(&mut self, i: usize) -> PixResult<&mut Line<f64>> {
        self.edges
            .get_mut(i)
            .ok_or_else(|| PixError::Other(Cow::from("invalid edge index")))
    }

    #[allow(clippy::many_single_char_names)]
    fn convert_edges_to_poly_map(&mut self) -> PixResult<()> {
        let s = Rect::new(0, 0, self.xcells as i32, self.ycells as i32);
        let pitch = self.xcells as i32;
        let block_size = BLOCK_SIZE as i32;
        // Reset edges state, keeping only the window boundaries
        self.edges.truncate(4);
        for c in self.cells.iter_mut() {
            c.reset();
        }
        let width = s.width as i32;
        let height = s.height as i32;
        for x in 0..width {
            for y in 0..height {
                let x_off = x + s.x;
                let y_off = y + s.y;
                let i = (y_off * pitch + x_off) as usize; // This
                let n = ((y_off - 1) * pitch + x_off) as usize; // Northern neighbor
                let s = ((y_off + 1) * pitch + x_off) as usize; // Sourthern neighbor
                let w = (y_off * pitch + (x_off - 1)) as usize; // Western neighbor
                let e = (y_off * pitch + (x_off + 1)) as usize; // Eastern neighbor

                // Cell exists, check for edges
                if self.exists(i) {
                    let x_off = x_off as f64;
                    let y_off = y_off as f64;
                    let block_size = block_size as f64;
                    // No western neighbor, so needs an edge
                    if x > 0 && !self.exists(w) {
                        // Can extend down from northern neighbors WEST edge
                        if self.has_edge(n, WEST) {
                            let edge_id = self.get_edge_index(n, WEST)?;
                            self.get_edge_mut(edge_id)?.end.y += block_size as f64;
                            self.cells[i].edges[WEST] = (true, edge_id);
                        } else {
                            // Create WEST edge extending downward
                            let start = vector!(x_off * block_size, y_off * block_size);
                            let end = vector!(start.x, start.y + block_size as f64);
                            let edge = Line::new(start, end);
                            let edge_id = self.edges.len();
                            self.edges.push(edge);
                            self.cells[i].edges[WEST] = (true, edge_id);
                        }
                    }
                    // No eastern neighbor, so needs an edge
                    if x < width && !self.exists(e) {
                        // Can extend down from northern neighbors EAST edge
                        if self.has_edge(n, EAST) {
                            let edge_id = self.get_edge_index(n, EAST)?;
                            self.get_edge_mut(edge_id)?.end.y += block_size as f64;
                            self.cells[i].edges[EAST] = (true, edge_id);
                        } else {
                            // Create EAST edge extending downward
                            let start =
                                vector!(x_off * block_size + block_size, y_off * block_size);
                            let end = vector!(start.x, start.y + block_size as f64);
                            let edge = Line::new(start, end);
                            let edge_id = self.edges.len();
                            self.edges.push(edge);
                            self.cells[i].edges[EAST] = (true, edge_id);
                        }
                    }
                    // No northern neighbor, so needs an edge
                    if y > 0 && !self.exists(n) {
                        // Can extend from western neighbors NORTH edge
                        if self.has_edge(w, NORTH) {
                            let edge_id = self.get_edge_index(w, NORTH)?;
                            self.get_edge_mut(edge_id)?.end.x += block_size as f64;
                            self.cells[i].edges[NORTH] = (true, edge_id);
                        } else {
                            // Create NORTH edge extending right
                            let start = vector!(x_off * block_size, y_off * block_size);
                            let end = vector!(start.x + block_size as f64, start.y);
                            let edge = Line::new(start, end);
                            let edge_id = self.edges.len();
                            self.edges.push(edge);
                            self.cells[i].edges[NORTH] = (true, edge_id);
                        }
                    }
                    // No southern neighbor, so needs an edge
                    if y < height && !self.exists(s) {
                        // Can extend from western neighbors SOUTH edge
                        if self.has_edge(w, SOUTH) {
                            let edge_id = self.get_edge_index(w, SOUTH)?;
                            self.get_edge_mut(edge_id)?.end.x += block_size as f64;
                            self.cells[i].edges[SOUTH] = (true, edge_id);
                        } else {
                            // Create SOUTH edge extending right
                            let start =
                                vector!(x_off * block_size, y_off * block_size + block_size);
                            let end = vector!(start.x + block_size as f64, start.y);
                            let edge = Line::new(start, end);
                            let edge_id = self.edges.len();
                            self.edges.push(edge);
                            self.cells[i].edges[SOUTH] = (true, edge_id);
                        }
                    }
                }
            }
        }

        self.points.clear();
        for edge in &self.edges {
            self.points.push(edge.start);
            self.points.push(edge.end);
        }
        self.points
            .sort_unstable_by(|a, b| a.partial_cmp(&b).unwrap_or(Less));
        self.points.dedup();
        Ok(())
    }

    fn calc_visibility_polygons(&mut self, o: Vector<f64>) {
        self.polygons.clear();
        for &p in self.points.iter() {
            // Cast three rays - one at and one off to each side
            for offset in -1..=1 {
                let angle = offset as f64 / 10_000.0;
                let mut r = p.as_vector() - o;
                r.rotate(angle);
                if let Some(intersect) = self.cast_ray(o, r) {
                    self.polygons.push((r.heading(), intersect));
                    continue;
                }
            }
        }

        // Could fail with NaN or Infinity
        self.polygons
            .sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Less));
        self.polygons
            .dedup_by(|a, b| (a.1.x - b.1.x).abs() <= 0.1 && (a.1.y - b.1.y).abs() <= 0.1);
    }

    fn cast_ray(&self, o: Vector<f64>, r: Vector<f64>) -> Option<Point<f64>> {
        let mut intersect = None;
        let mut closest_param = f64::INFINITY;
        let ray: Line<f64> = Line::new(o, r + o);
        for &e in self.edges.iter() {
            if let Some((point, param)) = ray.intersects(e) {
                if intersect.is_none() || param < closest_param {
                    intersect = Some(point);
                    closest_param = param;
                }
            }
        }
        intersect
    }

    fn draw_visibility_polygons(&mut self, s: &mut PixState) -> PixResult<bool> {
        let mouse = s.mouse_pos();
        if mouse.x <= 0 || mouse.x > s.width() as i32 || mouse.y <= 0 || mouse.y > s.height() as i32
        {
            return Ok(true);
        }

        self.calc_visibility_polygons(mouse.as_vector());

        s.fill(WHITE);
        s.stroke(WHITE);
        let mouse = mouse.into();
        if !self.polygons.is_empty() {
            for i in 0..self.polygons.len() - 1 {
                let p1 = self.polygons[i].1;
                let p2 = self.polygons[i + 1].1;
                s.triangle([mouse, p1, p2])?;
            }
            // Draw last triangle, connecting back to first point.
            // SAFETY: self.polygons has at least one element due to is_empty() check above
            let p1 = self.polygons.last().unwrap().1;
            let p2 = self.polygons[0].1;
            s.triangle([mouse, p1, p2])?;
        }

        s.fill(RED);
        s.no_stroke();
        s.circle([mouse.x, mouse.y, 2.0])?;
        Ok(true)
    }
}

impl AppState for RayScene {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        s.background(BLACK);
        s.scale(SCALE as f32, SCALE as f32)?;
        s.cursor(false);

        let w = (self.xcells * BLOCK_SIZE) as i32 - 1;
        let h = (self.ycells * BLOCK_SIZE) as i32 - 1;

        // Random scattered cells to start with
        for _ in 0..50 {
            let i = self.get_cell_index(random!(w - 1), random!(h - 1));
            self.cells[i].exists = !self.cells[i].exists;
        }

        // Screen Edges
        let w = w as f64;
        let h = h as f64;
        self.edges.push(Line::new([0.0, 0.0], [w, 0.0])); // Top
        self.edges.push(Line::new([w, 0.0], [w, h])); // Right
        self.edges.push(Line::new([0.0, h], [w, h])); // Bottom
        self.edges.push(Line::new([0.0, 0.0], [0.0, h])); // Left

        self.convert_edges_to_poly_map()?;

        self.light = Some(s.create_image_from_file("static/light.png")?);
        s.blend_mode(BlendMode::Mod);

        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.clear();

        let mouse = s.mouse_pos();

        let (cx, cw) = if mouse.x - 254 < 0 {
            (0, mouse.x + 255)
        } else {
            (mouse.x - 254, 511)
        };
        let (cy, ch) = if mouse.y - 254 < 0 {
            (0, mouse.y + 255)
        } else {
            (mouse.y - 254, 511)
        };
        s.clip([cx, cy, cw, ch]);

        self.draw_visibility_polygons(s)?;

        s.fill(BLUE);
        s.stroke(BLUE);
        for cell in self.cells.iter().filter(|c| c.exists) {
            let p = cell.pos.as_point();
            s.square([p.x, p.y, BLOCK_SIZE as i32 + 1])?;
        }

        if let Some(ref light) = self.light {
            s.image(mouse.x - 255, mouse.y - 255, &light)?;
        }

        Ok(())
    }

    fn on_mouse_pressed(&mut self, s: &mut PixState, btn: Mouse) -> PixResult<()> {
        if btn == Mouse::Left {
            let m = s.mouse_pos();
            if m.x > 0 && m.x <= s.width() as i32 && m.y > 0 && m.y <= s.height() as i32 {
                let i = self.get_cell_index(m.x, m.y);
                self.cells[i].exists = !self.cells[i].exists;
                self.drawing = self.cells[i].exists;
                self.convert_edges_to_poly_map()?;
            }
        }
        Ok(())
    }

    fn on_mouse_dragged(&mut self, s: &mut PixState) -> PixResult<()> {
        if s.mouse_buttons().contains(&Mouse::Left) {
            let m = s.mouse_pos();
            let pm = s.pmouse_pos();
            if m.x > 0 && m.x <= s.width() as i32 && m.y > 0 && m.y <= s.height() as i32 {
                if m != pm {
                    let i = self.get_cell_index(m.x, m.y);
                    self.cells[i].exists = self.drawing;
                }
                self.convert_edges_to_poly_map()?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Cell {
    pos: Vector<f64>,
    edges: [(bool, usize); 4],
    exists: bool,
}

// 0,0 -> 0,0,16,16
// 1,0 -> 16,0,32,16
impl Cell {
    pub fn new<P: Into<Vector<f64>>>(pos: P) -> Self {
        Self {
            pos: pos.into() * BLOCK_SIZE as f64,
            edges: [(false, 0); 4],
            exists: false,
        }
    }

    fn reset(&mut self) {
        self.edges = [(false, 0); 4];
    }
}

fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title(TITLE)
        .with_frame_rate()
        .position_centered()
        .vsync_enabled()
        .icon("static/light.png")
        .resizable()
        .build();
    let mut app = RayScene::new();
    engine.run(&mut app)
}
