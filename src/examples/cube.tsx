import { useEffect } from 'react'
import { Cube } from "wasm-src";

const FPS_THROTTLE = 1000.0 / 60;

export default () => {
  useEffect(() => {
    const c = new Cube("gl-playground");

    const initTime = Date.now();

    const animationID = setInterval(() => {
      window.requestAnimationFrame(() => {
        const elapsedTime = (Date.now() - initTime) / 10;
        c.render(800, 800, elapsedTime, 0);
      })
    }, FPS_THROTTLE);

    return () => clearInterval(animationID);
  }, []);

  return (
    <div className="bg-black flex items-center justify-center w-screen h-screen">
      <canvas id="gl-playground" height={800} width={800} />
    </div>
  )
}
