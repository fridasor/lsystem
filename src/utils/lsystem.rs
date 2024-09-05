use core::f64::consts::PI;

#[derive(Debug, Default, Clone)]
pub struct LSystem {
    n_iterations: usize,
    pub vertices: Vec<Vec<(f64, f64)>>,
    axiom: String,
    angle: f64,
    length: f64,
    pub rulestring: String,
    pub symbols: Vec<char>,
    pub rules: Vec<String>,
    current_string: String,
}


impl LSystem {
    pub fn new(n: usize, rule: &str, axiom: &str, angle: f64, length: f64) -> Self {

        let mut symbols = Vec::new();
        let mut rules: Vec<_> = rule
            .split(',')
            .map(|x| x.trim().to_string().replace("=>", ""))
            .collect();
        
        for rule in &mut rules {
            symbols.push(rule.remove(0));
        }

        Self {
            n_iterations: n,
            vertices: Vec::new(),
            angle: angle * PI / 180.0,
            axiom: axiom.to_string(),
            symbols: symbols,
            rulestring: rule.to_string(),
            rules: rules,
            length: length,
            current_string: axiom.to_string(),
        }
    }

    pub fn fill_string(&mut self) {

        let placeholders = ["f", "g", "h", "i"];
        for _ in 0..self.n_iterations {
            for (placeholder, symbol) in placeholders.iter().zip(self.symbols.iter()) {
                self.current_string = self.current_string.replace(*symbol, *placeholder);
            }
            for (rule, placeholder) in self.rules.iter().zip(placeholders.iter()) {
                self.current_string = self.current_string.replace(*placeholder, &rule);
            }
        }
        

    }



    pub fn fill_vertices(&mut self) {
        let mut velocity: (f64, f64) = (0.0, 1.0);
        let mut point: (f64, f64) = (0.0, 0.0);
        let mut current_path: Vec<(f64, f64)> = vec![(0.0, 0.0)];

        let mut brackets_points: Vec<(f64, f64)> = Vec::new();
        let mut brackets_velocities: Vec<(f64, f64)> = Vec::new();


        for c in self.current_string.chars() {
            match c {
                '[' => {
                    brackets_points.push(point.clone());
                    brackets_velocities.push(velocity.clone());
                }
                ']' => {
                    self.vertices.push(current_path);
                    point = brackets_points.pop().expect("Could not pop point.");
                    current_path = vec![point.clone()];

                    velocity = brackets_velocities.pop().expect("Could not pop velocity.");
                }
                'F' | 'G' | 'X' => {
                    point = (point.0 + velocity.0 * self.length, point.1 + velocity.1 * self.length);
                    current_path.push(point.clone());
                }
                '+' => {
                    velocity = (
                        velocity.0 * (- self.angle).cos() - velocity.1 * (- self.angle).sin(),
                        velocity.0 * (- self.angle).sin() + velocity.1 * (-self.angle).cos()
                    );
                }
                '-' => {
                    velocity = (
                        velocity.0 * self.angle.cos() - velocity.1 * self.angle.sin(), 
                        velocity.0 * self.angle.sin() + velocity.1 * self.angle.cos()
                    );
                }
                _ => (),
            }

        }

        self.vertices.push(current_path);

    }
}


