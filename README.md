# Urban Learn

A terminal experiment comparing how random walking, serpentine scanning, and nearest-house exploration affect house-price prediction. Macroquad is no longer part of the application or dependencies. Old experimental files in `src/bin` are excluded from the build; the unused resource manager has been removed.

```sh
cargo run --release
cargo run --release -- --maps 5 --size 60 --steps 500 --limit 100 --no-maps
cargo run --release -- --sizes 30,60,100 --output results/my_experiment
cargo test
```

Running without arguments prompts for the number of maps and then a separate square size for each map. Enter `3`, then `30`, `60`, and `100` to run those three sizes. Press Enter to accept a displayed default. Invalid numbers are requested again. Supply `--interactive` alongside options if you also want prompts when setting energy or output options.

Command-line runs with arguments use those options without prompting. `--maps 3 --size 60` repeats a 60-by-60 size three times; `--sizes 30,60,100` supplies a different size per map and infers the map count. If both `--maps` and `--sizes` are supplied, their counts must match. Each size must be from 1 to 200, meaning up to 40,000 tiles per map. Legacy `--width` and `--height` options must now be supplied together and equal.

Defaults: one 30-by-30 map, 500 movement energy per agent, and up to 100 unique samples. `--no-maps` hides the terminal map grids; all files are still exported. Console grids are also automatically hidden above 50-by-50 to keep the output readable; full grids remain in TXT and PNG. The old `--headless` flag is accepted for compatibility; every run is terminal-only. A run overwrites files at its output path; use a different `--output` directory to retain separate experiments.

Map PNGs grow with the grid, keeping tiles at least 8 pixels wide and reserving separate space for the legend. A 30-by-30 map exports at 1110 by 1000 pixels; a 200-by-200 map exports at 1810 by 1700 pixels. All map tiles are rendered. Large maps take longer to generate, process environmental features, and evaluate. The maximum is a practical application limit rather than a PNG format limit. A fixed 500-step budget covers a much smaller fraction of a large map, so adjust the energy budget deliberately for your experiment.

Generated `results/`, `plots/`, and JSON files in `saved_maps/` are ignored by Git. Keep custom output folders under `results/` to inherit that rule. Existing tracked saved maps remain tracked; `test_vectors/`, source files, assets, and `Cargo.lock` should remain in version control.

Agents start at `(0,0)` at the top left. Each cardinal tile move costs one energy. Observing the starting tile is free. Collection and model training do not consume movement energy. All tiles are traversable. Random walking chooses uniformly from valid neighbours. Systematic exploration scans alternating rows and stops at the last tile. Smart exploration knows only house locations before visiting, chooses the nearest unvisited house by Manhattan distance, and moves horizontally first. Ties follow house order. It collects houses encountered en route. These are shortest paths to successive targets, not a globally optimal tour.

All agents explore the same generated neighbourhood sequentially, with identical energy, sample limits, and Random Forest settings (100 trees, seed 42). They train only on their own observations. After exploration, the test set consists of every house that **none** of the agents collected. All models predict the exact same test houses. No test price enters model training or route selection. The first ten test houses appear side by side in the terminal with actual prices, predictions, and absolute errors; the files contain every test prediction.

This common unseen set is selected by the routes, so it describes prediction in the area left unexplored by all strategies. It is not an independent random holdout representative of the whole neighbourhood. If the agents collectively observe every house, prediction metrics are N/A: increase map size or reduce energy/sample limits. At least two training houses are required to train a model. Price statistics use the existing statistics function, which requires three houses.

| Metric | Meaning | Better |
| --- | --- | --- |
| MAE | Average absolute price error | Lower |
| RMSE | Square root of average squared price error; emphasizes large misses | Lower |
| R2 | `1 - sum(error^2) / sum((actual - test_mean)^2)` | Higher |
| Steps per house | Movement energy divided by unique samples | Lower |

R2 can be negative and is undefined for a single test house or constant test prices. The separately reported baseline always predicts the agent's **training sample mean**; its RMSE gives a comparison that uses only observed training data. The test mean in the mathematical definition of R2 is not used to make predictions.

For each map, the terminal displays population and sample mean, median, standard deviation, range, minimum, maximum, count, sample diversity by property type, movement efficiency, and prediction metrics. Final summaries give the mean and sample SD of map-level metrics, with equal weight per map and the number of valid maps contributing. Undefined values are excluded, not treated as zero. Sampling methods can change both sample count and composition; use equal sample counts in a separate experiment to isolate composition alone.

House generation uses `SimulationManager::multithread_process_house` with two scoped worker threads, preserving house order when combining their outputs. Zero or one house uses the serial processor directly. Agent exploration and model evaluation still run sequentially.

The existing `StatisticsManager`, `HouseStatistics`, `HistogramBin`, and `BinDetails` now drive population and sample statistics and histogram exports. Each map uses five shared price intervals for the neighbourhood and all three agents, rounded up to a $100,000 maximum boundary. Bin counts are printed in the terminal and included in `report.json`; the final interval includes its upper boundary. Empty and constant-price samples also export valid charts. Histogram counts show distribution and sample size together; their vertical axes adjust to each dataset.

Outputs under `results/` (or `--output`):

- `summary.csv`: one row per map and strategy, including map dimensions, suitable for a spreadsheet.
- `summary.json`: means and sample SD across maps, including sample and energy summaries.
- `map_001/report.json` (and subsequent map folders): every house's position, features, actual price, feature names, each agent's sampled house IDs, route, energy, statistics, metrics, baseline, and full test predictions/errors. Sample IDs reference the houses array in this file.
- `map_001/predictions.csv`: actual and predicted prices and signed/absolute errors for each test house and strategy.
- `map_001/comparison.png`: MAE, RMSE, R2, sample counts, and energy comparison charts.
- `map_001/histogram_neighbourhood.png`, `histogram_random.png`, `histogram_systematic.png`, `histogram_smart.png`: population and sample price distributions using shared price boundaries.
- `map_001/neighbourhood.png`, `random.png`, `systematic.png`, `smart.png`: map and routes.
- `map_001/neighbourhood.txt`, `random.txt`, `systematic.txt`, `smart.txt`: printable map grids with legends, visited tiles, collected houses, and common unseen test houses.
- `map_001/layout.json`: building grid.

House type feature encoding is townhouse=0, semi-detached=1, detached=2. Route positions and house positions use named `x` and `y` fields; grid arrays are indexed `[y][x]`. Signed prediction error is predicted minus actual.

Maps and house attributes/prices are random across runs. The random walker and forest use fixed seeds, so all variation across maps also reflects differences in generated neighbourhoods. Full generated feature data is saved for inspection; rerunning the same command generates new maps. The synthetic price formula includes its existing additive environmental adjustment, with a final minimum price of $10,000. Strong environmental penalties can therefore produce multiple houses at this minimum.
