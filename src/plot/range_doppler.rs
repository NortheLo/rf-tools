use ndarray::ArrayView2;
use ruviz::prelude::*;


/// Plots the range doppler map
pub fn range_doppler(
    rdm: &ArrayView2<f32>,
    filename: &str,
) -> PlotResult<()> {

    Plot::new()
        .dpi(1024)
        .heatmap(rdm)
        .title("Range-Doppler Map")
        .xlabel("Doppler Bin [Hz]")
        .ylabel("Range Bin [ ]")
        .colorbar_label("Power [dB]")
        .cmap(ColorMap::viridis())
        .theme(Theme::publication())
        .typst(true)
        .save(filename)?;
    Ok(())
}