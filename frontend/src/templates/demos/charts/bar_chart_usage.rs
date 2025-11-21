html! {
  <div class="w-full max-w-2xl md:max-w-3xl mx-auto">
    <BarChartComponent
      data={vec![
        BarChartData { label: "Jan".into(), value: 100.0, color: "#3b82f6".into() },
        BarChartData { label: "Feb".into(), value: 200.0, color: "#10b981".into() },
        BarChartData { label: "Mar".into(), value: 150.0, color: "#f59e0b".into() },
        BarChartData { label: "Apr".into(), value: 300.0, color: "#ef4444".into() },
      ]}
      legend_position={LegendPosition::Auto}
    />
  </div>
}
