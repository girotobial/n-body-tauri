import React, { useEffect, useRef } from 'react';
import { Body, Boundary } from '../types';

type CanvasProps = {
    bodies: Body[],
    bounds: Boundary[]
}

const Canvas: React.FC<CanvasProps> = ({ bodies, bounds }) => {
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
            context.arc(body.position.x, body.position.y, 1, 0, 2 * Math.PI);
            context.fillStyle = 'red';
            context.fill();
            context.stroke();
        }

        const drawBoundary = (bound: Boundary): void => {
            const width = bound.max.x - bound.min.x;
            const height = bound.max.y - bound.min.y;

            context.beginPath();
            context.strokeRect(bound.min.x, bound.min.y, width, -height);
        }

        const update = () => {
            context.clearRect(0, 0, canvas.width, canvas.height);

            bodies.forEach(body => {
                draw(body);
            });

            bounds.forEach(bound => {
                drawBoundary(bound);
            })
        }

        update()

        // Draw bodies
    }, [bodies, bounds]);


    return <canvas ref={canvasRef} />;
};

export default Canvas;
