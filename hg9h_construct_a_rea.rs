// Data model for the real-time mobile app controller

pub struct MobileAppController {
    id: i32,
    app_name: String,
    device_id: String,
    os: String,
    screen_resolution: (i32, i32),
    touch_events: Vec<TouchEvent>,
    accelerometer_data: Vec<AccelerometerData>,
    gyroscope_data: Vec<GyroscopeData>,
}

pub struct TouchEvent {
    timestamp: i64,
    x: i32,
    y: i32,
    event_type: String, // tap, swipe, pinch, etc.
}

pub struct AccelerometerData {
    timestamp: i64,
    x: f64,
    y: f64,
    z: f64,
}

pub struct GyroscopeData {
    timestamp: i64,
    x: f64,
    y: f64,
    z: f64,
}

impl MobileAppController {
    pub fn new(id: i32, app_name: String, device_id: String, os: String, screen_resolution: (i32, i32)) -> Self {
        MobileAppController {
            id,
            app_name,
            device_id,
            os,
            screen_resolution,
            touch_events: Vec::new(),
            accelerometer_data: Vec::new(),
            gyroscope_data: Vec::new(),
        }
    }

    pub fn add_touch_event(&mut self, event: TouchEvent) {
        self.touch_events.push(event);
    }

    pub fn add_accelerometer_data(&mut self, data: AccelerometerData) {
        self.accelerometer_data.push(data);
    }

    pub fn add_gyroscope_data(&mut self, data: GyroscopeData) {
        self.gyroscope_data.push(data);
    }
}