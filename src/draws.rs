#![allow(unused_imports)]
#![allow(missing_docs)]
#![allow(dead_code)]

use crate::engine::{Backtest, Candle};
use crate::errors::{Error, Result};
use chrono::{DateTime, Utc};
use std::fmt::Write;
use std::path::Path;

#[derive(Debug)]
pub enum DrawOutput<P: AsRef<Path>> {
    Svg(P),
    Png(P),
    Html(P),
}

impl Default for DrawOutput<&'static str> {
    fn default() -> Self {
        Self::Svg("output.svg")
    }
}

#[derive(Debug, Default)]
pub struct DrawOptions {
    title: Option<String>,
    width: Option<usize>,
    height: Option<usize>,
    show_volume: bool,
}

impl DrawOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl ToString) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn size(mut self, width: usize, height: usize) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn show_volume(mut self, show: bool) -> Self {
        self.show_volume = show;
        self
    }
}

#[derive(Debug)]
pub struct Draw<'d> {
    backtest: Option<&'d Backtest>,
    options: DrawOptions,
}

impl<'d> Draw<'d> {
    pub fn with_backtest(backtest: &'d Backtest) -> Self {
        Self {
            backtest: Some(backtest),
            options: DrawOptions::default(),
        }
    }

    pub fn with_options(mut self, options: DrawOptions) -> Self {
        self.options = options;
        self
    }

    pub fn 

    pub fn to_html(&self) -> Result<String> {
        let backtest = self.backtest.ok_or(Error::Msg("No backtest provided".to_string()))?;
        let candles = backtest.candles().collect::<Vec<_>>();
        if candles.is_empty() {
            return Err(Error::CandleDataEmpty);
        }

        let title = self.options.title.as_deref().unwrap_or("BTS Report");
        let width = self.options.width.unwrap_or(800);
        let height = self.options.height.unwrap_or(600);
        let show_volume = self.options.show_volume;
        let candles_json = serde_json::to_string(
            &candles
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    serde_json::json!({
                        "index": i,
                        "open": c.open(),
                        "high": c.high(),
                        "low": c.low(),
                        "close": c.close(),
                        "volume": c.volume(),
                        "time": c.open_time().to_rfc3339()
                    })
                })
                .collect::<Vec<_>>(),
        )?;

        let mut html = String::new();

        writeln!(
            html,
            r###"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>{title}</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        h1 {{ color: #333; }}
        .chart {{ width: {width}px; height: {height}px; border: 1px solid #ccc; margin: 20px 0; position: relative; }}
        .candle {{ position: absolute; }}
        .wick {{ stroke: #000; stroke-width: 1; }}
        .body {{ fill: #000; }}
        .bullish {{ fill: #26a69a; }}
        .bearish {{ fill: #ef5350; }}
        .volume {{ position: absolute; bottom: 0; left: 0; background: rgba(0, 0, 255, 0.2); }}
        .axis {{ stroke: #999; stroke-width: 1; }}
        .label {{ font-size: 12px; color: #666; }}
        .tooltip {{ position: absolute; background: rgba(0, 0, 0, 0.8); color: white; padding: 5px; border-radius: 3px; pointer-events: none; }}
    </style>
    <script src="https://d3js.org/d3.v7.min.js"></script>
</head>
<body>
    <h1>{title}</h1>
    <div class="chart" id="chart"></div>
    <script>
        document.addEventListener('DOMContentLoaded', function() {{
            const candles = {candles_json};
            const showVolume = {show_volume};

            // Configuration du graphique
            const margin = {{top: 20, right: 20, bottom: showVolume ? 50 : 30, left: 50}},
                  width = {width} - margin.left - margin.right,
                  height = {height} - margin.top - margin.bottom;

            // Créer le SVG
            const svg = d3.select("#chart")
                .append("svg")
                .attr("width", {width})
                .attr("height", {height})
                .append("g")
                .attr("transform", `translate(${{margin.left}}, ${{margin.top}})`);

            // Échelles
            const x = d3.scaleBand()
                .domain(candles.map((_, i) => i))
                .range([0, width])
                .padding(0.1);

            const y = d3.scaleLinear()
                .domain(d3.extent(candles, d => d.close))
                .range([height, 0]);

            const yVolume = d3.scaleLinear()
                .domain([0, d3.max(candles, d => d.volume)])
                .range([height, 0]);

            // Dessiner les axes
            svg.append("g")
                .attr("class", "axis")
                .attr("transform", `translate(0, ${{height}})`)
                .call(d3.axisBottom(x));

            svg.append("g")
                .attr("class", "axis")
                .call(d3.axisLeft(y));

            // Dessiner les candles
            svg.selectAll(".candle")
                .data(candles)
                .enter()
                .append("g")
                .attr("class", "candle")
                .attr("transform", (_, i) => `translate(${{x(i) + x.bandwidth() / 2}}, 0)`)
                .each(function(d) {{
                    const g = d3.select(this);
                    const isBullish = d.close >= d.open;

                    // Corps du candle
                    g.append("rect")
                        .attr("class", isBullish ? "body bullish" : "body bearish")
                        .attr("x", -x.bandwidth() / 2)
                        .attr("y", d => y(Math.max(d.open, d.close)))
                        .attr("width", x.bandwidth())
                        .attr("height", d => Math.abs(y(d.open) - y(d.close)));

                    // Mèches
                    g.append("line")
                        .attr("class", "wick")
                        .attr("x1", 0)
                        .attr("y1", d => y(d.high))
                        .attr("x2", 0)
                        .attr("y2", d => y(d.low));

                    // Tooltip
                    g.append("title")
                        .text(d => `O: ${{d.open.toFixed(2)}}, H: ${{d.high.toFixed(2)}},
                              L: ${{d.low.toFixed(2)}}, C: ${{d.close.toFixed(2)}}`);
                }});

            // Dessiner les volumes si activé
            if (showVolume) {{
                svg.selectAll(".volume")
                    .data(candles)
                    .enter()
                    .append("rect")
                    .attr("class", "volume")
                    .attr("x", (_, i) => x(i))
                    .attr("y", d => yVolume(d.volume))
                    .attr("width", x.bandwidth())
                    .attr("height", d => height - yVolume(d.volume));
            }}
        }});
    </script>
</body>
</html>
"###
        )?;

        html = html.replace("{title}", title);
        html = html.replace("{width}", &width.to_string());
        html = html.replace("{height}", &height.to_string());
        html = html.replace("{show_volume}", &show_volume.to_string().to_lowercase());

        Ok(html)
    }

    pub fn to_text(&self) -> Result<String> {
        let backtest = self.backtest.ok_or(Error::Msg("No backtest provided".to_string()))?;
        let candles = backtest.candles().collect::<Vec<_>>();
        if candles.is_empty() {
            return Err(Error::CandleDataEmpty);
        }

        let mut output = String::new();
        let title = self.options.title.as_deref().unwrap_or("BTS Backtest Report");

        writeln!(output, "{}", title).unwrap();
        writeln!(output, "{}", "=".repeat(title.len())).unwrap();
        writeln!(output, "Number of candles: {}", candles.len()).unwrap();
        writeln!(
            output,
            "Time range: {} to {}",
            candles.first().ok_or(Error::CandleNotFound)?.open_time().to_rfc3339(),
            candles.last().ok_or(Error::CandleNotFound)?.close_time().to_rfc3339()
        )
        .unwrap();
        writeln!(output).unwrap();

        writeln!(
            output,
            "{:<10} {:<10} {:<10} {:<10} {:<10} {:<20} {:<20}",
            "Index", "Open", "High", "Low", "Close", "Volume", "Time"
        )
        .unwrap();

        for (i, candle) in candles.iter().enumerate() {
            writeln!(
                output,
                "{:<10} {:<10.2} {:<10.2} {:<10.2} {:<10.2} {:<20.2} {:<20}",
                i,
                candle.open(),
                candle.high(),
                candle.low(),
                candle.close(),
                candle.volume(),
                candle.open_time().to_rfc3339()
            )
            .unwrap();
        }

        Ok(output)
    }

    pub fn save_to_file(&self, path: impl AsRef<std::path::Path>) -> Result<()> {
        use std::fs::File;
        use std::io::Write;

        let html = self.to_html()?;
        let mut file = File::create(path)?;
        file.write_all(html.as_bytes())?;
        Ok(())
    }
}
