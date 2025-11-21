html! {
  <div class="w-full max-w-2xl md:max-w-3xl mx-auto">
    <BubbleChartComponent
      points={vec![
        BubbleChartPoint {
          x: 100.0,
          y: 100.0,
          radius: 20.0,
          color: "#3b82f6".into(),
          label: "Blue Team".into(),
        },
        BubbleChartPoint {
          x: 200.0,
          y: 150.0,
          radius: 30.0,
          color: "#10b981".into(),
          label: "Green Squad".into(),
        },
        BubbleChartPoint {
          x: 300.0,
          y: 200.0,
          radius: 25.0,
          color: "#f59e0b".into(),
          label: "Yellow Group".into(),
        },
        BubbleChartPoint {
          x: 400.0,
          y: 100.0,
          radius: 15.0,
          color: "#ef4444".into(),
          label: "Red Alert".into(),
        },
      ]}
    />
  </div>
}
