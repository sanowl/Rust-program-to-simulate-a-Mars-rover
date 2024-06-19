use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Motor {
    power: f64,
    efficiency: f64, // Efficiency factor for converting power to force
    friction: f64,   // Friction factor affecting motor deceleration
}

impl Motor {
    fn new() -> Motor {
        Motor {
            power: 0.0,
            efficiency: 0.8, // Example efficiency factor (80% efficient)
            friction: 0.1,   // Example friction factor
        }
    }

    fn set_power(&mut self, power: f64) {
        self.power = power;
        println!("Motor power set to {:.2}%", self.power);
    }

    fn stop(&mut self) {
        self.power = 0.0;
        println!("Motor stopped");
    }

    fn apply_force(&self) -> f64 {
        self.power * self.efficiency * 0.01 // Arbitrary conversion to force for simulation
    }

    fn apply_friction(&self, velocity: f64) -> f64 {
        -self.friction * velocity
    }
}

#[derive(Debug)]
struct Rover {
    motor_left: Motor,
    motor_right: Motor,
    position: (f64, f64), // (x, y) position
    velocity: (f64, f64), // (vx, vy) velocity
    orientation: f64,     // Current orientation angle in degrees
    communication_module: CommunicationModule,
    battery: f64,         // Battery level
    sensors: Sensors,     // Sensors for environment scanning
    grid: Vec<Vec<char>>, // Map grid for pathfinding
    energy_consumed: f64, // Total energy consumed by the rover
}

#[derive(Debug)]
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

#[derive(Debug)]
struct Sensors {
    has_camera: bool,
    has_lidar: bool,
    battery_level: f64,
}

impl Sensors {
    fn new() -> Sensors {
        Sensors {
            has_camera: true,
            has_lidar: true,
            battery_level: 100.0,
        }
    }

    fn scan_environment(&self) {
        println!("Scanning environment...");
        if self.has_camera {
            println!("Using camera for environment scan.");
        }
        if self.has_lidar {
            println!("Using LiDAR for environment scan.");
        }
        thread::sleep(Duration::from_secs(2));
        println!("Environment scan complete.");
    }

    fn detect_obstacle(&self) -> bool {
        let camera_result = self.detect_with_camera();
        let lidar_result = self.detect_with_lidar();
        camera_result || lidar_result
    }

    fn detect_with_camera(&self) -> bool {
        println!("Analyzing camera data...");
        thread::sleep(Duration::from_secs(1));
        println!("Camera analysis complete.");
        self.has_camera && (rand::random::<f64>() < 0.2)
    }

    fn detect_with_lidar(&self) -> bool {
        println!("Scanning with LiDAR...");
        thread::sleep(Duration::from_secs(1));
        println!("LiDAR scan complete.");
        self.has_lidar && (rand::random::<f64>() < 0.2)
    }

    fn battery_usage(&mut self, usage: f64) {
        self.battery_level -= usage;
        if self.battery_level < 0.0 {
            self.battery_level = 0.0;
        }
        println!("Battery level: {:.2}%", self.battery_level);
    }

    fn get_battery_level(&self) -> f64 {
        self.battery_level
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    cost: f64,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Rover {
    fn new(grid_size: usize) -> Rover {
        let grid = vec![vec!['.'; grid_size]; grid_size];
        Rover {
            motor_left: Motor::new(),
            motor_right: Motor::new(),
            position: (0.0, 0.0),
            velocity: (0.0, 0.0),
            orientation: 0.0,
            communication_module: CommunicationModule::new(),
            battery: 100.0,
            sensors: Sensors::new(),
            grid,
            energy_consumed: 0.0,
        }
    }

    fn move_forward(&mut self, distance: f64) {
        let dx = distance * self.orientation.to_radians().cos();
        let dy = distance * self.orientation.to_radians().sin();
        self.position.0 += dx;
        self.position.1 += dy;
        self.velocity = (self.velocity.0 + dx, self.velocity.1 + dy);
        println!("Moving forward by {:.2} meters", distance);
        self.drive_motors(50.0);
        self.energy_consumed += distance * 0.5; // Example energy consumption rate
        self.battery -= distance * 0.5; // Example battery usage rate
        self.sensors.battery_usage(distance * 0.5); // Update sensor battery level
        thread::sleep(Duration::from_millis(200));
        self.stop_motors();
    }

    fn move_backward(&mut self, distance: f64) {
        let dx = -distance * self.orientation.to_radians().cos();
        let dy = -distance * self.orientation.to_radians().sin();
        self.position.0 += dx;
        self.position.1 += dy;
        self.velocity = (self.velocity.0 + dx, self.velocity.1 + dy);
        println!("Moving backward by {:.2} meters", distance);
        self.drive_motors(-50.0);
        self.energy_consumed += distance * 0.5;
        self.battery -= distance * 0.5;
        self.sensors.battery_usage(distance * 0.5); // Update sensor battery level
        thread::sleep(Duration::from_millis(200));
        self.stop_motors();
    }

    fn turn_left(&mut self, angle: f64) {
        self.orientation += angle;
        println!("Turning left by {:.2} degrees", angle);
        self.drive_motors(-30.0, 30.0);
        self.energy_consumed += angle * 0.1; // Example energy consumption rate for turning
        self.battery -= angle * 0.1; // Example battery usage rate for turning
        self.sensors.battery_usage(angle * 0.1); // Update sensor battery level
        thread::sleep(Duration::from_millis(100));
        self.stop_motors();
    }

    fn turn_right(&mut self, angle: f64) {
        self.orientation -= angle;
        println!("Turning right by {:.2} degrees", angle);
        self.drive_motors(30.0, -30.0);
        self.energy_consumed += angle * 0.1;
        self.battery -= angle * 0.1;
        self.sensors.battery_usage(angle * 0.1); // Update sensor battery level
        thread::sleep(Duration::from_millis(100));
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

    fn update_velocity(&mut self) {
        let force_left = self.motor_left.apply_force();
        let force_right = self.motor_right.apply_force();
        let acceleration = (
            (force_left + force_right) * self.orientation.to_radians().cos(),
            (force_left + force_right) * self.orientation.to_radians().sin(),
        );
        self.velocity = (
            self.velocity.0 + acceleration.0 + self.motor_left.apply_friction(self.velocity.0),
            self.velocity.1 + acceleration.1 + self.motor_right.apply_friction(self.velocity.1),
        );
    }

    fn update_position(&mut self, time_step: f64) {
        self.position.0 += self.velocity.0 * time_step;
        self.position.1 += self.velocity.1 * time_step;
    }

    fn get_position(&self) -> (f64, f64) {
        self.position
    }

    fn get_velocity(&self) -> (f64, f64) {
        self.velocity
    }

    fn get_orientation(&self) -> f64 {
        self.orientation
    }

    fn get_battery_level(&self) -> f64 {
        self.battery
    }

    fn get_energy_consumed(&self) -> f64 {
        self.energy_consumed
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

    fn a_star(&self, start: (usize, usize), goal: (usize, usize)) -> Option<Vec<(usize, usize)>> {
        let mut open_set = BinaryHeap::new();
        let mut came_from = vec![vec![None; self.grid.len()]; self.grid.len()];
        let mut g_score = vec![vec![f64::INFINITY; self.grid.len()]; self.grid.len()];
        let mut f_score = vec![vec![f64::INFINITY; self.grid.len()]; self.grid.len()];

        g_score[start.0][start.1] = 0.0;
        f_score[start.0][start.1] = self.heuristic(start, goal);

        open_set.push(State {
            cost: f_score[start.0][start.1],
            position: start,
        });

        while let Some(State { cost: _, position }) = open_set.pop() {
            if position == goal {
                return Some(self.reconstruct_path(came_from, position));
            }

            for neighbor in self.get_neighbors(position) {
                let tentative_g_score = g_score[position.0][position.1] + self.distance(position, neighbor);

                if tentative_g_score < g_score[neighbor.0][neighbor.1] {
                    came_from[neighbor.0][neighbor.1] = Some(position);
                    g_score[neighbor.0][neighbor.1] = tentative_g_score;
                    f_score[neighbor.0][neighbor.1] = g_score[neighbor.0][neighbor.1] + self.heuristic(neighbor, goal);

                    open_set.push(State {
                        cost: f_score[neighbor.0][neighbor.1],
                        position: neighbor,
                    });
                }
            }
        }

        None
    }

    fn heuristic(&self, start: (usize, usize), goal: (usize, usize)) -> f64 {
        let (x1, y1) = start;
        let (x2, y2) = goal;
        ((x2 as isize - x1 as isize).abs() + (y2 as isize - y1 as isize).abs()) as f64
    }

    fn distance(&self, start: (usize, usize), neighbor: (usize, usize)) -> f64 {
        let (x1, y1) = start;
        let (x2, y2) = neighbor;
        (((x2 as isize - x1 as isize).pow(2) + (y2 as isize - y1 as isize).pow(2)) as f64).sqrt()
    }

    fn get_neighbors(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let (x, y) = position;

        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x < self.grid.len() - 1 {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y < self.grid.len() - 1 {
            neighbors.push((x, y + 1));
        }

        neighbors
    }

    fn reconstruct_path(&self, came_from: Vec<Vec<Option<(usize, usize)>>>, mut current: (usize, usize)) -> Vec<(usize, usize)> {
        let mut total_path = vec![current];
        while let Some(next) = came_from[current.0][current.1] {
            current = next;
            total_path.push(current);
        }
        total_path.reverse();
        total_path
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
    let mut rover = Rover::new(10);
    let time_step = 0.1; // Simulation time step in seconds

    loop {
        rover.update_velocity();
        rover.update_position(time_step);

        println!("Enter command ('forward 10', 'left 90', 'navigate 5 5', 'scan', 'terrain rocky', 'connect', 'send_data', 'autopilot', 'pathfind 0 0 9 9', 'exit'):");
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
                    println!("Battery Level: {:.2}%", rover.get_battery_level());
                    println!("Energy Consumed: {:.2} J", rover.get_energy_consumed());
                }
            }
            "backward" => {
                if parts.len() > 1 {
                    let distance: f64 = parts[1].parse().unwrap_or(0.0);
                    rover.move_backward(distance);
                    println!("Current Position: {:?}", rover.get_position());
                    println!("Battery Level: {:.2}%", rover.get_battery_level());
                    println!("Energy Consumed: {:.2} J", rover.get_energy_consumed());
                }
            }
            "left" => {
                if parts.len() > 1 {
                    let angle: f64 = parts[1].parse().unwrap_or(0.0);
                    rover.turn_left(angle);
                    println!("Current Orientation: {:.2} degrees", rover.get_orientation());
                    println!("Battery Level: {:.2}%", rover.get_battery_level());
                    println!("Energy Consumed: {:.2} J", rover.get_energy_consumed());
                }
            }
            "right" => {
                if parts.len() > 1 {
                    let angle: f64 = parts[1].parse().unwrap_or(0.0);
                    rover.turn_right(angle);
                    println!("Current Orientation: {:.2} degrees", rover.get_orientation());
                    println!("Battery Level: {:.2}%", rover.get_battery_level());
                    println!("Energy Consumed: {:.2} J", rover.get_energy_consumed());
                }
            }
            "navigate" => {
                if parts.len() > 2 {
                    let target_x: f64 = parts[1].parse().unwrap_or(0.0);
                    let target_y: f64 = parts[2].parse().unwrap_or(0.0);
                    rover.navigate_to(target_x, target_y);
                    println!("Battery Level: {:.2}%", rover.get_battery_level());
                    println!("Energy Consumed: {:.2} J", rover.get_energy_consumed());
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
            "pathfind" => {
                if parts.len() > 4 {
                    let start_x: usize = parts[1].parse().unwrap_or(0);
                    let start_y: usize = parts[2].parse().unwrap_or(0);
                    let goal_x: usize = parts[3].parse().unwrap_or(0);
                    let goal_y: usize = parts[4].parse().unwrap_or(0);
                    if let Some(path) = rover.a_star((start_x, start_y), (goal_x, goal_y)) {
                        println!("Path found: {:?}", path);
                    } else {
                        println!("No path found.");
                    }
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

        if rover.sensors.detect_obstacle() {
            println!("Stopping due to obstacle!");
            break;
        }

        println!("Current Velocity: {:?}", rover.get_velocity());
        thread::sleep(Duration::from_secs_f64(time_step));
    }
}
