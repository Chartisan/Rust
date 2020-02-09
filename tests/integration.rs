extern crate chartisan;

use chartisan::Chartisan;

#[test]
fn chart_empty() {
    let chart = Chartisan::build().to_json();
    assert_eq!(
        chart,
        "{\"chart\":{\"labels\":[],\"extra\":null},\"datasets\":[]}"
    );
}

#[test]
fn chart_labels() {
    let chart = Chartisan::build().labels(&["a", "b", "c"]).to_json();
    assert_eq!(
        chart,
        "{\"chart\":{\"labels\":[\"a\",\"b\",\"c\"],\"extra\":null},\"datasets\":[]}"
    );
}

#[test]
fn chart_complete() {
    let chart = Chartisan::build()
        .labels(&["a", "b", "c"])
        .dataset("Sample 1", &[1.0, 2.0, 3.0])
        .dataset("Sample 2", &[4.0, 5.0, 6.0])
        .to_json();
    assert_eq!(
        chart,
        "{\"chart\":{\"labels\":[\"a\",\"b\",\"c\"],\"extra\":null},\"datasets\":[{\"name\":\"Sample 1\",\"values\":[1.0,2.0,3.0],\"extra\":null},{\"name\":\"Sample 2\",\"values\":[4.0,5.0,6.0],\"extra\":null}]}"
    );
}
