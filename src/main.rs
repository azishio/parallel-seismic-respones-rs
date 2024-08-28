use seismic_response::{ResponseAccAnalyzer, ResponseAccAnalyzerParams};
use std::fs::File;
use std::io;
use std::io::Write;

const MAX_T: u32 = 5000;
const T_STEP: usize = 100;
const OUTPUT_FILE_PATH: &str = "output.csv";
const INPUT_FILE_PATH: &str = "./pEW.csv";

fn main() {
    let file = File::open(INPUT_FILE_PATH).expect("Failed to open input file");
    let input = io::read_to_string(file)
        .expect("Failed to read input file")
        .lines()
        .flat_map(|line| line.parse::<f64>())
        .collect::<Vec<_>>();

    // #[cfg(not(feature = "rayon"))]
    let result = (0..MAX_T).step_by(T_STEP).map(|t| {
        let params = ResponseAccAnalyzerParams {
            natural_period_ms: t,
            dt_ms: 10,
            damping_h: 0.05,
            beta: 0.25,
            init_x: 0.0,
            init_v: 0.0,
            init_a: 0.0,
            init_xg: 0.0,
        };

        let analyzer = ResponseAccAnalyzer::from_params(params);


        analyzer.analyze(&input).abs_acc.into_iter().reduce(|a, b| a.max(b.abs())).expect("Failed to reduce")
    });

    // #[cfg(feature = "rayon")]
    // let result = (0..MAX_T).into_par_iter().step_by(T_STEP).map(|t| {
    //     let analyzer = ResponseAccAnalyzer::builder()
    //         .natural_period_ms(t)
    //         .build();

    //     analyzer.analyze(&input).abs_acc.into_par_iter().reduce(|| 0., |a, b| a.max(b.abs()))
    // });

    let mut output_file = File::create(OUTPUT_FILE_PATH).expect("Failed to create output file");
    let str = result.map(|v| v.to_string()).collect::<Vec<_>>().join("\n");
    output_file.write_all(str.as_bytes()).expect("Failed to write output file");
}
