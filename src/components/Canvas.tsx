import React, { useEffect, useRef } from 'react';
import { Body, Boundary, Tree, Vec } from '../types';

type CanvasProps = {
    bodies: Body[],
    tree: Tree
}

const Canvas: React.FC<CanvasProps> = ({ bodies, tree }) => {
    const canvasRef = useRef<HTMLCanvasElement | null>(null);

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
            context.fillText(`${x.toFixed()}, ${y.toFixed()}`, x, y)
            context.fill();
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

        const translate_and_scale = (pos: Vec, bounds: Boundary): Vec => {
            const min = bounds.min;
            const max = bounds.max;
            const scale = {
                x: canvas.width / (max.x - min.x),
                y: canvas.height / (max.y - min.y)
            };

            return {
                x: (pos.x - bounds.min.x) * scale.x,
                y: (pos.y - bounds.min.y) * scale.y
            }
        }

        const update = () => {
            context.clearRect(0, 0, canvas.width, canvas.height);
            let boundary = tree.outer_bounds;


            context.beginPath();
            context.fillText(`${boundary.min.x.toFixed()} -> ${boundary.max.x.toFixed()}`, 50, 50);
            context.fillText(`${boundary.min.y.toFixed()} -> ${boundary.max.y.toFixed()}`, 50, 100);

            bodies.forEach(body => {
                draw(body);
            });

            tree.boundaries.forEach(bound => {
                drawBoundary(bound);
            });
        }

        update()

        // Draw bodies
    }, [bodies, tree]);


    return <canvas ref={canvasRef} />;
};

export default Canvas;
