html! {
  <LineChartComponent lines={vec![
      LineChartData {
          label: "Team A".into(),
          color: "#3b82f6".into(),
          points: vec![
              LineChartPoint { x: 0.0, y: 50.0 },
              LineChartPoint { x: 100.0, y: 150.0 },
              LineChartPoint { x: 200.0, y: 100.0 },
              LineChartPoint { x: 300.0, y: 250.0 },
          ],
      },
      LineChartData {
          label: "Team B".into(),
          color: "#10b981".into(),
          points: vec![
              LineChartPoint { x: 0.0, y: 30.0 },
              LineChartPoint { x: 100.0, y: 80.0 },
              LineChartPoint { x: 200.0, y: 200.0 },
              LineChartPoint { x: 300.0, y: 300.0 },
          ],
      },
  ]} />
}
