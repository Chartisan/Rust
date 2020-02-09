pub mod data;

extern crate serde_json;

use data::{ChartData, DatasetData, ServerData};
use std::collections::HashMap;

/// Represents a chartisan chart instance.
pub struct Chartisan<'a> {
    /// Stores the server data of the chart.
    server_data: ServerData<'a>,
}

impl<'a> Chartisan<'a> {
    /// Creates a new instance of a chartisan chart.
    pub fn build() -> Chartisan<'a> {
        Chartisan {
            server_data: ServerData {
                chart: ChartData {
                    labels: &[],
                    extra: None,
                },
                datasets: Vec::new(),
            },
        }
    }

    /// Sets the chart labels.
    pub fn labels(&mut self, labels: &'a [&'a str]) -> &mut Self {
        self.server_data.chart.labels = labels;
        self
    }

    /// Adds extra information to the chart.
    pub fn extra(&mut self, value: HashMap<&'a str, &'a str>) -> &mut Self {
        self.server_data.chart.extra = Some(value);
        self
    }

    /// advanced_dataset appends a new dataset to the chart or modifies an existing one.
    /// If the ID has already been used, the dataset will be replaced with this one.
    pub fn advanced_dataset(
        &mut self,
        name: &'a str,
        values: &'a [f64],
        extra: Option<HashMap<&'a str, &'a str>>,
    ) -> &mut Self {
        match self.get_dataset(name) {
            Some(dataset) => {
                dataset.name = name;
                dataset.values = values;
                dataset.extra = extra;
            }
            None => self.server_data.datasets.push(DatasetData {
                name,
                values,
                extra,
            }),
        };
        self
    }

    /// Dataset adds a new simple dataset to the chart. If more advanced control is
    /// needed, consider using `AdvancedDataset` instead.
    pub fn dataset(&mut self, name: &'a str, values: &'a [f64]) -> &mut Self {
        self.advanced_dataset(name, values, None)
    }

    /// Transforms the chart into a JSON string.
    pub fn to_JSON(&self) -> String {
        serde_json::to_string(&self.server_data)
            .unwrap_or("{\"error\": \"Error converting chart to JSON\"}".to_string())
    }

    /// Transforms it to an object.
    pub fn to_object(&mut self) -> &mut ServerData<'a> {
        &mut self.server_data
    }

    /// Gets the dataset with the given name.
    fn get_dataset(&mut self, name: &'a str) -> Option<&mut DatasetData<'a>> {
        for dataset in &mut self.server_data.datasets {
            if dataset.name == name {
                return Some(dataset);
            }
        }
        None
    }
}
