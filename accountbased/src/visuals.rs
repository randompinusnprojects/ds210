use plotters::prelude::*;

evcxr_figure!((640, 480), |root| {
    let root = root.titled("Scatter with Histogram Example", ("Arial", 20).into_font())?;
    
    let areas = root.split_by_breakpoints([560], [80]);

    let mut x_hist_ctx = ChartBuilder::on(&areas[0])
        .y_label_area_size(40)
        .build_cartesian_2d(0u32..100u32, 0f64..0.5f64)?;
    let mut y_hist_ctx = ChartBuilder::on(&areas[3])
        .x_label_area_size(40)
        .build_cartesian_2d(0f64..0.5f64, 0..100u32)?;
    let mut scatter_ctx = ChartBuilder::on(&areas[2])
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..1f64, 0f64..1f64)?;
    scatter_ctx.configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;
    scatter_ctx.draw_series(random_points.iter().map(|(x,y)| Circle::new((*x,*y), 3, GREEN.filled())))?;
    let x_hist = Histogram::vertical(&x_hist_ctx)
        .style(RED.filled())
        .margin(0)
        .data(random_points.iter().map(|(x,_)| ((x*100.0) as u32, 0.01)));
    let y_hist = Histogram::horizontal(&y_hist_ctx)
        .style(GREEN.filled())
        .margin(0)
        .data(random_points.iter().map(|(_,y)| ((y*100.0) as u32, 0.01)));
    x_hist_ctx.draw_series(x_hist)?;
    y_hist_ctx.draw_series(y_hist)?;
    
    Ok(())
}).style("width:60%")

:dep plotters = { version = "^0.3.0", default_features = false, features = ["evcxr", "all_series", "all_elements"] }
use plotters::prelude::*;

evcxr_figure!((640, 480), |root| {
    let root = root.titled("3D Plotting", ("Arial", 20).into_font())?;
    
    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_3d(-10.0..10.0, -10.0..10.0, -10.0..10.0)?;
    
    chart.configure_axes().draw()?;
    
    // Draw a red circle parallel to XOZ panel
    chart.draw_series(LineSeries::new(
        (-314..314).map(|a| a as f64 / 100.0).map(|a| (8.0 * a.cos(), 0.0, 8.0 *a.sin())),
        &RED,
    ))?;
    // Draw a green circle parallel to YOZ panel
    chart.draw_series(LineSeries::new(
        (-314..314).map(|a| a as f64 / 100.0).map(|a| (0.0, 8.0 * a.cos(), 8.0 *a.sin())),
        &GREEN,
    ))?;
    
    Ok(())
})