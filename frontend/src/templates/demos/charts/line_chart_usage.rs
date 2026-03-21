html! {
  <div class="w-full max-w-2xl md:max-w-3xl mx-auto">
    <LineChartComponent
      legend_position={LegendPosition::Bottom}
      x_axis_formatter={LineChartValueFormatter::Integer}
      y_axis_formatter={LineChartValueFormatter::currency("$", 2)}
      tooltip_formatter={LineChartTooltipFormatter::value_pair(
        LineChartValueFormatter::Integer,
        LineChartValueFormatter::currency("$", 4),
      )}
      lines={vec![
        LineChartData {
          label: "Inference".into(),
          color: "#3b82f6".into(),
          points: vec![
            LineChartPoint { x: 1.0, y: 0.1825 },
            LineChartPoint { x: 2.0, y: 0.3162 },
            LineChartPoint { x: 3.0, y: 0.2741 },
            LineChartPoint { x: 4.0, y: 0.6484 },
            LineChartPoint { x: 5.0, y: 0.9113 },
          ],
        },
        LineChartData {
          label: "Embeddings".into(),
          color: "#10b981".into(),
          points: vec![
            LineChartPoint { x: 1.0, y: 0.0412 },
            LineChartPoint { x: 2.0, y: 0.0638 },
            LineChartPoint { x: 3.0, y: 0.0594 },
            LineChartPoint { x: 4.0, y: 0.1025 },
            LineChartPoint { x: 5.0, y: 0.1481 },
          ],
        },
        LineChartData {
          label: "Messages".into(),
          color: "#81306f".into(),
          points: vec![
            LineChartPoint { x: 1.0, y: 1.00 },
            LineChartPoint { x: 2.0, y: 2.0638 },
            LineChartPoint { x: 3.0, y: 3.0594 },
            LineChartPoint { x: 4.0, y: 4.1025 },
            LineChartPoint { x: 5.0, y: 5.0 },
          ],
        },
      ]}
    />
  </div>
}
