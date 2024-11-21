import React, { useEffect, useRef } from 'react';
import { Body, Boundary, Tree } from '../types';

type CanvasProps = {
    bodies: Body[],
    tree: Tree
}

const Canvas: React.FC<CanvasProps> = ({ bodies, tree }) => {
    const canvasRef = useRef<HTMLCanvasElement | null>(null);
    var lastCalledTime;
    var fps;
    performance.now

    useEffect(() => {
        const canvas = canvasRef.current;
        if (!canvas) return;

        const context = canvas.getContext('2d');
        if (!context) return;

        // Set canvas dimensions
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;

        // Clear the canvas
        const draw = (body: Body): void => {
            context.beginPath();
            context.fillStyle = 'red';
            context.strokeStyle = 'red';
            const x = body.position.x;
            const y = body.position.y;
            const radius = body.radius;
            context.arc(x, y, radius / 3, 0, 2  * Math.PI);
            context.fill();
            context.stroke();
        }

        const drawBoundary = (bound: Boundary): void => {
            const width = bound.max.x - bound.min.x;
            const height = bound.max.y - bound.min.y;
            
            context.strokeStyle = "grey";
            context.beginPath();
            context.strokeRect(bound.min.x, bound.min.y, width, height);
        }

        const drawCom = (tree: Tree): void => {
            context.fillStyle = 'green';
            context.strokeStyle = 'green';
            context.beginPath();
            const x = tree.center_of_mass.x;
            const y = tree.center_of_mass.y;
            context.arc(x, y, 1, 0, 2 * Math.PI);
            context.fillText(`Center of Mass: ${x.toFixed()}, ${y.toFixed()}`, x + 5, y);
            context.fill();
            context.stroke();
        }

        const drawFps = (ps: number | void): void => {
            context.fillStyle = "Black";
            context.font = "normal 16pt Arial";
            context.fillText(ps + " fps",10,26);
        }

        const update = () => {
            if (!lastCalledTime) {
                lastCalledTime = performance.now();
                fps = 0;
                return;
            }
            const delta = (performance.now() - lastCalledTime) / 1000;
            lastCalledTime = performance.now();
            fps = 1/delta;

            context.clearRect(0, 0, canvas.width, canvas.height);

            bodies.forEach(body => {
                draw(body);
            });

            tree.boundaries.forEach(bound => {
                drawBoundary(bound);
            });

            drawCom(tree);
            drawFps(fps);
        }

        update()

        // Draw bodies
    }, [bodies, tree]);


    return <canvas ref={canvasRef} />;
};

export default Canvas;
