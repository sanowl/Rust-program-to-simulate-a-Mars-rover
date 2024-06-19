use std::io::{self, Write};
use std::thread;
use std::time::Duration;

struct Motor {
    power: f64,
}

impl Motor {
    fn new() -> Motor {
        Motor { power: 0.0 }
    }

    fn set_power(&mut self, power: f64) {
        self.power = power;
        println!("Motor power set to {:.2}%", self.power);
    }

    fn stop(&mut self) {
        self.power = 0.0;
        println!("Motor stopped");
    }
}

struct Rover {
    motor_left: Motor,
    motor_right: Motor,
    position: (f64, f64), // (x, y) position
    orientation: f64,     // Current orientation angle in degrees
}

impl Rover {
    fn new() -> Rover {
        Rover {
            motor_left: Motor::new(),
            motor_right: Motor::new(),
            position: (0.0, 0.0),
            orientation: 0.0,
        }
    }

    fn move_forward(&mut self, distance: f64) {
        let dx = distance * self.orientation.to_radians().cos();
        let dy = distance * self.orientation.to_radians().sin();
        self.position.0 += dx;
        self.position.1 += dy;
        println!("Moving forward by {:.2} meters", distance);
        self.drive_motors(50.0); // Simulate both motors running at 50% power
        thread::sleep(Duration::from_millis(200)); // Simulate movement time
        self.stop_motors();
    }

    fn move_backward(&mut self, distance: f64) {
        let dx = -distance * self.orientation.to_radians().cos();
        let dy = -distance * self.orientation.to_radians().sin();
        self.position.0 += dx;
        self.position.1 += dy;
        println!("Moving backward by {:.2} meters", distance);
        self.drive_motors(-50.0); // Simulate both motors running in reverse at 50% power
        thread::sleep(Duration::from_millis(200)); // Simulate movement time
        self.stop_motors();
    }

    fn turn_left(&mut self, angle: f64) {
        self.orientation += angle;
        println!("Turning left by {:.2} degrees", angle);
        self.drive_motors(-30.0, 30.0); // Simulate left turn
        thread::sleep(Duration::from_millis(100)); // Simulate turning time
        self.stop_motors();
    }

    fn turn_right(&mut self, angle: f64) {
        self.orientation -= angle;
        println!("Turning right by {:.2} degrees", angle);
        self.drive_motors(30.0, -30.0); // Simulate right turn
        thread::sleep(Duration::from_millis(100)); // Simulate turning time
        self.stop_motors();
    }

    fn drive_motors(&mut self, power_left: f64, power_right: f64) {
        self.motor_left.set_power(power_left);
        self.motor_right.set_power(power_right);
    }

    fn stop_motors(&mut self) {
        self.motor_left.stop();
        self.motor_right.stop();
    }

    fn get_position(&self) -> (f64, f64) {
        self.position
    }

    fn get_orientation(&self) -> f64 {
        self.orientation
    }

    // Advanced navigation using basic pathfinding (placeholder)
    fn navigate_to(&mut self, target_x: f64, target_y: f64) {
        println!("Navigating to ({:.2}, {:.2})", target_x, target_y);
        let current_x = self.position.0;
        let current_y = self.position.1;

        // Calculate distance and angle to target
        let dx = target_x - current_x;
        let dy = target_y - current_y;
        let distance = (dx.powi(2) + dy.powi(2)).sqrt();
        let angle_to_target = dy.atan2(dx).to_degrees();

        // Turn towards the target
        let angle_difference = angle_to_target - self.orientation;
        if angle_difference.abs() > 1.0 {
            self.turn_right(angle_difference);
        }

        // Move towards the target
        self.move_forward(distance);
        println!("Arrived at: {:?}", self.get_position());
    }

    // Simulated obstacle detection (placeholder)
    fn detect_obstacle(&self) -> bool {
        // Simulate obstacle detection based on rover's position
        let obstacle_present = (self.position.0.abs() < 2.0 && self.position.1.abs() < 2.0);
        if obstacle_present {
            println!("Obstacle detected!");
        }
        obstacle_present
    }

    // Advanced environmental scanning (placeholder)
    fn scan_environment(&self) {
        println!("Scanning environment...");
        // Simulate scanning for obstacles or features
        thread::sleep(Duration::from_secs(1));
        println!("Environment scan complete.");
    }

    // Simulated terrain traversal (placeholder)
    fn traverse_terrain(&mut self, terrain_type: &str) {
        println!("Traversing {} terrain...", terrain_type);
        // Simulate different terrains affecting movement
        match terrain_type {
            "rocky" => {
                self.move_forward(2.0);
                self.turn_left(45.0);
                self.move_forward(1.5);
            }
            "sand" => {
                self.move_forward(3.0);
                self.turn_right(30.0);
                self.move_backward(1.0);
            }
            _ => {
                println!("Unknown terrain type.");
            }
        }
        println!("Terrain traversal complete.");
    }
}

fn main() {
    let mut rover = Rover::new();

    loop {
        println!("Enter command ('forward 10', 'left 90', 'navigate 5 5', 'scan', 'terrain rocky', 'exit'):");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "forward" => {
                if parts.len() > 1 {
                    let distance: f64 = parts[1].parse().unwrap_or(0.0);
                    rover.move_forward(distance);
                    println!("Current Position: {:?}", rover.get_position());
                }
            }
            "backward" => {
                if parts.len() > 1 {
                    let distance: f64 = parts[1].parse().unwrap_or(0.0);
                    rover.move_backward(distance);
                    println!("Current Position: {:?}", rover.get_position());
                }
            }
            "left" => {
                if parts.len() > 1 {
                    let angle: f64 = parts[1].parse().unwrap_or(0.0);
                    rover.turn_left(angle);
                    println!("Current Orientation: {:.2} degrees", rover.get_orientation());
                }
            }
            "right" => {
                if parts.len() > 1 {
                    let angle: f64 = parts[1].parse().unwrap_or(0.0);
                    rover.turn_right(angle);
                    println!("Current Orientation: {:.2} degrees", rover.get_orientation());
                }
            }
            "navigate" => {
                if parts.len() > 2 {
                    let target_x: f64 = parts[1].parse().unwrap_or(0.0);
                    let target_y: f64 = parts[2].parse().unwrap_or(0.0);
                    rover.navigate_to(target_x, target_y);
                }
            }
            "scan" => {
                rover.scan_environment();
            }
            "terrain" => {
                if parts.len() > 1 {
                    let terrain_type = parts[1];
                    rover.traverse_terrain(terrain_type);
                }
            }
            "exit" => {
                println!("Exiting program.");
                break;
            }
            _ => {
                println!("Invalid command.");
            }
        }

        // Check for obstacles after each action
        if rover.detect_obstacle() {
            println!("Stopping due to obstacle!");
            break;
        }
    }
}
