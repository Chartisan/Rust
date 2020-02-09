extern crate serde;
use serde::Serialize;
use std::collections::HashMap;

/// Reprensets the ChartData structure.
#[derive(Serialize)]
pub struct ChartData<'a> {
    /// Stores the chart labels.
    pub labels: &'a [&'a str],
    /// Stores the extra information of the chart.
    pub extra: Option<HashMap<&'a str, &'a str>>,
}

/// Represents the DatasetData of the chart.
#[derive(Serialize)]
pub struct DatasetData<'a> {
    /// Stores the dataset name (the label).
    pub name: &'a str,
    /// Stores the dataset values.
    pub values: &'a [f64],
    /// Stores the dataset extra information.
    pub extra: Option<HashMap<&'a str, &'a str>>,
}

/// Represents the server data of the chart.
#[derive(Serialize)]
pub struct ServerData<'a> {
    /// Store the chart information.
    pub chart: ChartData<'a>,
    /// Stores the chart's datasets.
    pub datasets: Vec<DatasetData<'a>>,
}
