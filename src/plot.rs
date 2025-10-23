use anyhow::Result;
use plotters::{element::PointCollection, prelude::*};

pub(crate) fn draw_lines<DB, CT, I, S>(
    chart: &mut ChartContext<DB, CT>,
    series: I,
    color: S,
) -> Result<()>
where
    DB: DrawingBackend,
    CT: CoordTranslate,
    I: IntoIterator<Item = f64>,
    S: Into<ShapeStyle>,
    for<'b> &'b DynElement<'static, DB, (i32, f64)>: PointCollection<'b, <CT>::From>,
    <DB>::ErrorType: 'static,
{
    let points = series.into_iter().enumerate().map(|(i, v)| (i as i32, v));
    let series = LineSeries::new(points, color);
    chart.draw_series(series)?;
    Ok(())
}
