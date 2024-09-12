use std::usize;

use ndarray::Array1; // 0.16.0
use plotly::common::Title;
use plotly::layout::{Axis, Layout};
use plotly::{Plot, Scatter};
use rand::thread_rng; // 0.8.5
use rand_distr::{Distribution, Normal}; // 0.4.3

const VOL: f32 = 0.17;
const T: f32 = 1. / 2.;
const N: f32 = 10000.;
const S_0: f32 = 100.;
const R: f32 = 0.05;
const _K: f32 = 100.;

fn calculate_spot(prev: f32, sigma: f32, r: f32, step: f32, random: f32) -> f32 {
    prev + (sigma * prev * random) + (r * prev * step)
}

fn sim_spot(s0: f32, r: f32, steps: f32, maturity: f32, vol: f32) -> Vec<f32> {
    let delta_t = T / steps;
    let _time = Array1::from_iter((0..=steps as i32).map(|_i| (maturity + delta_t).round()));
    let mut prices = vec![s0];
    let normal = Normal::new(0.0, delta_t.sqrt()).unwrap();
    let mut rng = thread_rng();
    let normal_dist: Array1<f32> =
        Array1::from_iter((0..N as i32).map(|_| normal.sample(&mut rng)));

    for a in 0..(steps as i32) {
        prices.push(calculate_spot(
            prices[prices.len() - 1],
            vol,
            r,
            T / N,
            normal_dist[a as usize],
        ));
    }
    prices
}

fn main() {
    let sims = sim_spot(S_0, R, N, T, VOL);
    let high: Vec<f32> = sims
        .iter()
        .enumerate()
        .map(|(i, _)| i as f32 + 0.5)
        .collect();
    let trace = Scatter::new(high, sims);
    let layout = Layout::new()
        .x_axis(Axis::new().title(Title::from("X Axis")))
        .y_axis(Axis::new().title(Title::from("Y Axis")))
        .title(Title::from("My Plot"));

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);
    plot.show();
}
