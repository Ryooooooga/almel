use crate::context::Context;
use crate::segments::Segment;

fn render_duration(duration: f64) -> String {
    static MICRO_SEC: f64 = 1e-6;
    static MILLI_SEC: f64 = 1e-3;
    static SEC: f64 = 1.0;
    static MIN: f64 = 60.0 * SEC;
    static HOUR: f64 = 60.0 * MIN;

    if duration < MILLI_SEC {
        format!("{:.0}μs", duration / MICRO_SEC)
    } else if duration < 10.0 * MILLI_SEC {
        format!("{:.2}ms", duration / MILLI_SEC)
    } else if duration < 100.0 * MILLI_SEC {
        format!("{:.1}ms", duration / MILLI_SEC)
    } else if duration < SEC {
        format!("{:.0}ms", duration / MILLI_SEC)
    } else if duration < 10.0 * SEC {
        format!("{:.2}s", duration / SEC)
    } else if duration + 0.05 * SEC < MIN {
        format!("{:.1}s", duration / SEC)
    } else if duration + 0.5 * SEC < HOUR {
        let d = duration + 0.5 * SEC;
        let mins = (d / MIN).floor();
        let secs = ((d % MIN) / SEC).floor();
        format!("{:.0}m {:.0}s", mins, secs)
    } else {
        let d = duration + 0.5 * SEC;
        let hours = (d / HOUR).floor();
        let mins = ((d % HOUR) / MIN).floor();
        let secs = ((d % MIN) / SEC).floor();
        format!("{:.0}h {:.0}m {:.0}s", hours, mins, secs)
    }
}

#[test]
fn test_render_duration() {
    assert_eq!(render_duration(0.000_001_234), "1μs");
    assert_eq!(render_duration(0.000_012_345), "12μs");
    assert_eq!(render_duration(0.000_123_456), "123μs");

    assert_eq!(render_duration(0.001_000_000), "1.00ms");
    assert_eq!(render_duration(0.001_200_000), "1.20ms");
    assert_eq!(render_duration(0.001_234_567), "1.23ms");

    assert_eq!(render_duration(0.012_000_000), "12.0ms");
    assert_eq!(render_duration(0.012_345_678), "12.3ms");

    assert_eq!(render_duration(0.100_000_000), "100ms");
    assert_eq!(render_duration(0.120_000_000), "120ms");
    assert_eq!(render_duration(0.123_456_789), "123ms");

    assert_eq!(render_duration(1.000_000_000), "1.00s");
    assert_eq!(render_duration(1.200_000_000), "1.20s");
    assert_eq!(render_duration(1.234_567_890), "1.23s");

    assert_eq!(render_duration(10.000_000_000), "10.0s");
    assert_eq!(render_duration(12.000_000_000), "12.0s");
    assert_eq!(render_duration(12.345_678_900), "12.3s");

    assert_eq!(render_duration(59.949), "59.9s");
    assert_eq!(render_duration(59.950), "1m 0s");

    assert_eq!(render_duration(100.000), "1m 40s");
    assert_eq!(render_duration(119.500), "2m 0s");
    assert_eq!(render_duration(120.000), "2m 0s");
    assert_eq!(render_duration(123.456), "2m 3s");
    assert_eq!(render_duration(133.456), "2m 13s");
    assert_eq!(render_duration(1234.567), "20m 35s");

    assert_eq!(render_duration(3599.499), "59m 59s");
    assert_eq!(render_duration(3599.500), "1h 0m 0s");

    assert_eq!(render_duration(3600.0), "1h 0m 0s");
    assert_eq!(render_duration(3912.0), "1h 5m 12s");
}

pub fn build_segment<'ctx>(context: &'ctx Context) -> Option<Segment<'ctx>> {
    let config = &context.config.duration;
    let duration = context.opt.duration;

    if duration > 0.0 {
        Some(Segment {
            style: &config.style,
            content: format!("{}{}", config.icon, render_duration(duration)),
        })
    } else {
        None
    }
}
