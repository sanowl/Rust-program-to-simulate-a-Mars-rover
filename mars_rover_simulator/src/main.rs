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
    communication_module: CommunicationModule,
    battery: f64,         // Battery level
    sensors: Sensors,     // Sensors for environment scanning
}

struct CommunicationModule {
    is_connected: bool,
}

impl CommunicationModule {
    fn new() -> CommunicationModule {
        CommunicationModule { is_connected: false }
    }

    fn connect(&mut self) {
        self.is_connected = true;
        println!("Communication module connected");
    }

    fn disconnect(&mut self) {
        self.is_connected = false;
        println!("Communication module disconnected");
    }

    fn send_data(&self, data: &str) {
        if self.is_connected {
            println!("Sending data: {}", data);
        } else {
            println!("Communication module not connected. Cannot send data.");
        }
    }
}

struct Sensors {
    has_camera: bool,
    has_lidar: bool,
}

impl Sensors {
    fn new() -> Sensors {
        Sensors {
            has_camera: true,
            has_lidar: false,
        }
    }

    fn scan_environment(&self) {
        println!("Scanning environment...");
        // Simulate sensor scanning based on sensor types
        if self.has_camera {
            println!("Using camera for environment scan.");
        }
        if self.has_lidar {
            println!("Using LiDAR for environment scan.");
        }
        thread::sleep(Duration::from_secs(2));
        println!("Environment scan complete.");
    }
}

impl Rover {
    fn new() -> Rover {
        Rover {
            motor_left: Motor::new(),
            motor_right: Motor::new(),
            position: (0.0, 0.0),
            orientation: 0.0,
            communication_module: CommunicationModule::new(),
            battery: 100.0,
            sensors: Sensors::new(),
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

    fn navigate_to(&mut self, target_x: f64, target_y: f64) {
        println!("Navigating to ({:.2}, {:.2})", target_x, target_y);
        let current_x = self.position.0;
        let current_y = self.position.1;

        let dx = target_x - current_x;
        let dy = target_y - current_y;
        let distance = (dx.powi(2) + dy.powi(2)).sqrt();
        let angle_to_target = dy.atan2(dx).to_degrees();

        let angle_difference = angle_to_target - self.orientation;
        if angle_difference.abs() > 1.0 {
            self.turn_right(angle_difference);
        }

        self.move_forward(distance);
        println!("Arrived at: {:?}", self.get_position());
    }

    fn detect_obstacle(&self) -> bool {
        let obstacle_present = (self.position.0.abs() < 2.0 && self.position.1.abs() < 2.0);
        if obstacle_present {
            println!("Obstacle detected!");
        }
        obstacle_present
    }

    fn scan_environment(&self) {
        self.sensors.scan_environment();
    }

    fn traverse_terrain(&mut self, terrain_type: &str) {
        println!("Traversing {} terrain...", terrain_type);
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

    fn communicate(&mut self, command: &str) {
        match command {
            "connect" => {
                self.communication_module.connect();
            }
            "disconnect" => {
                self.communication_module.disconnect();
            }
            "send_data" => {
                self.communication_module.send_data("Rover status report");
            }
            _ => {
                println!("Invalid communication command.");
            }
        }
    }

    fn auto_pilot(&mut self) {
        println!("Activating auto-pilot mode...");
        self.navigate_to(10.0, 10.0);
        self.scan_environment();
        self.traverse_terrain("rocky");
        self.navigate_to(-5.0, -5.0);
        println!("Auto-pilot mode complete.");
    }
}

fn main() {
    let mut rover = Rover::new();

    loop {
        println!("Enter command ('forward 10', 'left 90', 'navigate 5 5', 'scan', 'terrain rocky', 'connect', 'send_data', 'autopilot', 'exit'):");
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
            "connect" => {
                rover.communicate("connect");
            }
            "send_data" => {
                rover.communicate("send_data");
            }
            "autopilot" => {
                rover.auto_pilot();
            }
            "exit" => {
                println!("Exiting program.");
                break;
            }
            _ => {
                println!("Invalid command.");
            }
        }

        if rover.detect_obstacle() {
            println!("Stopping due to obstacle!");
            break;
        }
    }
}
