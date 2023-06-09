mod rounding;
use chrono::{DateTime, Utc, NaiveDateTime};

type TimePoint = DateTime<Utc>;

pub fn create_time_point_from_string(datetime: &str) -> TimePoint {
    let t: NaiveDateTime = NaiveDateTime::parse_from_str(datetime, "%Y-%m-%d %H:%M:%S").unwrap();
    return DateTime::from_utc(t, Utc);
}

pub struct TimeWindow {
    t1: TimePoint,
    t2: TimePoint,
    tx: Option<TimePoint>,
    pub total_millis: u64,
    pub left_millis: u64,
    pub passed_millis: u64,
    pub percentage: f64
}

impl TimeWindow {
    pub fn new(t1: TimePoint, t2: TimePoint) -> Self {
        return Self {
            t1,
            t2,
            tx: None,
            total_millis: 0,
            left_millis: 0,
            passed_millis: 0,
            percentage: 0.0,
        }
    }

    pub fn set_point(&mut self, t: TimePoint) {
        self.tx = Some(t);
        self.calculate();
    }

    /**
     * get left time in days
     * */
    pub fn get_left_days(&self) -> f64 {
        return (self.left_millis as f64)/(1000.0*60.0*60.0*24.0);
    }

    fn calculate(&mut self) {
        let t1 = self.t1.timestamp_millis();
        let t2 = self.t2.timestamp_millis();
        let tx = self.tx.unwrap().timestamp_millis();
        self.total_millis = (t2 - t1) as u64;
        self.passed_millis = (tx - t1) as u64;
        self.left_millis = (t2 - tx) as u64;
        self.percentage = rounding::round(self.passed_millis as f64 / self.total_millis as f64, 3);
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    fn create_time_from(datetime: &str) -> TimePoint {
        let t: NaiveDateTime = NaiveDateTime::parse_from_str(datetime, "%Y-%m-%d %H:%M:%S").unwrap();
        return DateTime::from_utc(t, Utc);
    }

    #[test]
    fn given_right_edge_of_window_then_returns_1() {
        let mut day = TimeWindow::new(create_time_from("2023-01-01 00:00:00"), create_time_from("2023-01-02 00:00:00"));
        day.set_point(create_time_from("2023-01-02 00:00:00"));

        assert_eq!(1.0, day.percentage);
    }

    #[test]
    fn given_left_edge_of_window_then_returns_0() {
        let mut day = TimeWindow::new(create_time_from("2023-01-01 00:00:00"), create_time_from("2023-01-02 00:00:00"));
        day.set_point(create_time_from("2023-01-01 00:00:00"));

        assert_eq!(0.0, day.percentage);
    }

    #[test]
    fn given_middle_of_window_then_returns_half() {
        let mut day = TimeWindow::new(create_time_from("2023-01-01 00:00:00"), create_time_from("2023-01-02 00:00:00"));
        day.set_point(create_time_from("2023-01-01 12:00:00"));

        assert_eq!(0.5, day.percentage);
    }

    #[test]
    fn given_arbitrary_point_in_window_then_returns_correct_value() {
        let mut day = TimeWindow::new(create_time_from("2023-01-01 00:00:00"), create_time_from("2023-01-02 00:00:00"));
        day.set_point(create_time_from("2023-01-01 13:31:45"));

        assert_eq!(0.564, day.percentage);
    }

    #[test]
    fn given_point_in_window_when_left_days_then_returns_correct_value() {
        let mut window = TimeWindow::new(create_time_from("2023-01-01 00:00:00"), create_time_from("2023-01-05 00:00:00"));
        window.set_point(create_time_from("2023-01-02 13:31:45"));

        assert_eq!(2.436284722222222, window.get_left_days());
    }
}
