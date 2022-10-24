import { Routes, Route } from "react-router-dom";
import Triangle from "./examples/triangle";

function App() {
  return (
    <Routes>
      <Route path="/">
        <Route index element={<Triangle />} />
        <Route path="triangle" element={<Triangle />} />
        <Route path="*" element={<Triangle />} />
      </Route>
    </Routes>
  )
}

export default App
