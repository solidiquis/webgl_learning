import { Routes, Route } from "react-router-dom";
import Triangle from "./examples/triangle";
import Cube from "./examples/cube";

function App() {
  return (
    <Routes>
      <Route path="/">
        <Route index element={<Cube />} />
        <Route path="triangle" element={<Triangle />} />
        <Route path="cube" element={<Cube />} />
        <Route path="*" element={<Cube />} />
      </Route>
    </Routes>
  )
}

export default App
