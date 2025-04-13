html! {
  <ScatterPlotComponent points={vec![
      ScatterPlotPoint { x: 120.0, y: 300.0, color: "#3b82f6".into(), label: "Mercury".into() },
      ScatterPlotPoint { x: 180.0, y: 220.0, color: "#10b981".into(), label: "Venus".into() },
      ScatterPlotPoint { x: 250.0, y: 200.0, color: "#f59e0b".into(), label: "Earth".into() },
      ScatterPlotPoint { x: 300.0, y: 160.0, color: "#ef4444".into(), label: "Mars".into() },
      ScatterPlotPoint { x: 400.0, y: 100.0, color: "#8b5cf6".into(), label: "Jupiter".into() },
      ScatterPlotPoint { x: 430.0, y: 80.0, color: "#ec4899".into(), label: "Saturn".into() },
      ScatterPlotPoint { x: 460.0, y: 120.0, color: "#14b8a6".into(), label: "Uranus".into() },
      ScatterPlotPoint { x: 490.0, y: 150.0, color: "#6366f1".into(), label: "Neptune".into() },
      ScatterPlotPoint { x: 500.0, y: 180.0, color: "#eab308".into(), label: "Pluto".into() },

      // Extra filler items for overflow test
      ScatterPlotPoint { x: 60.0, y: 90.0, color: "#f87171".into(), label: "Kepler-22b".into() },
      ScatterPlotPoint { x: 85.0, y: 120.0, color: "#34d399".into(), label: "Gliese 581g".into() },
      ScatterPlotPoint { x: 110.0, y: 110.0, color: "#60a5fa".into(), label: "HD 209458b".into() },
      ScatterPlotPoint { x: 135.0, y: 150.0, color: "#a78bfa".into(), label: "TRAPPIST-1e".into() },
      ScatterPlotPoint { x: 160.0, y: 140.0, color: "#fbbf24".into(), label: "Proxima Centauri b".into() },
      ScatterPlotPoint { x: 185.0, y: 130.0, color: "#fb7185".into(), label: "TOI 700 d".into() },
      ScatterPlotPoint { x: 210.0, y: 145.0, color: "#818cf8".into(), label: "LHS 1140 b".into() },
      ScatterPlotPoint { x: 235.0, y: 135.0, color: "#f472b6".into(), label: "K2-18b".into() },
      ScatterPlotPoint { x: 260.0, y: 125.0, color: "#fcd34d".into(), label: "55 Cancri e".into() },
      ScatterPlotPoint { x: 285.0, y: 115.0, color: "#86efac".into(), label: "WASP-12b".into() },
      ScatterPlotPoint { x: 310.0, y: 105.0, color: "#a3e635".into(), label: "CoRoT-7b".into() },
  ]} />
}
