import { useEffect, useRef } from 'react'
import { Triangle } from "wasm-src";

export default () => {
  const triangle = useRef<Triangle | null>(null);
  
  useEffect(() => {
    const tri = new Triangle("gl-playground");
    triangle.current = tri;
    tri.render();
  }, []);

  return (
    <div className="bg-black flex items-center justify-center w-screen h-screen">
      <canvas id="gl-playground" height={800} width={800} />
    </div>
  )
}
