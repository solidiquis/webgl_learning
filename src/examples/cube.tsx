import { useEffect, useRef } from 'react'
import { Cube } from "wasm-src";

export default () => {
  const cube = useRef<Cube | null>(null);
  
  useEffect(() => {
    console.log("cube")
    const c = new Cube("gl-playground");
    cube.current = c;
    c.render();
  }, []);

  return (
    <div className="bg-black flex items-center justify-center w-screen h-screen">
      <canvas id="gl-playground" height={800} width={800} />
    </div>
  )
}
