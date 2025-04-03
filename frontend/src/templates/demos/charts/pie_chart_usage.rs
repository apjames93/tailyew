html! {
  <PieChartComponent
      chart_id="sales"
      data={vec![
          PieChartData { label: "Product A".into(), value: 30.0, color: "#3b82f6".into() },
          PieChartData { label: "Product B".into(), value: 45.0, color: "#10b981".into() },
          PieChartData { label: "Product C".into(), value: 25.0, color: "#f59e0b".into() },
      ]}
  />
}
