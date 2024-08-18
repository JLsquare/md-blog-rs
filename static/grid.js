const GRID_SIZE = 64;
const CELL_SIZE = 32;

const canvas = document.getElementById('gridCanvas');
const ctx = canvas.getContext('2d');
const main = document.getElementsByTagName('main')[0];

let grid = Array(GRID_SIZE).fill().map(() => Array(GRID_SIZE).fill(false));

function resizeCanvas() {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    drawGrid();
}

function drawGrid() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    
    for (let y = 0; y < GRID_SIZE; y++) {
        for (let x = 0; x < GRID_SIZE; x++) {
            if (grid[y][x]) {
                ctx.fillStyle = 'rgba(69, 134, 168, 0.1)';
                ctx.fillRect(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE);
            }
        }
    }
}

function getMousePos(canvas, evt) {
    const rect = canvas.getBoundingClientRect();
    return {
        x: Math.floor((evt.clientX - rect.left) / CELL_SIZE),
        y: Math.floor((evt.clientY - rect.top) / CELL_SIZE)
    };
}

canvas.addEventListener('click', onClick);

main.addEventListener('click', onClick);

function onClick(event) {
    const mousePos = getMousePos(canvas, event);
    if (mousePos.x >= 0 && mousePos.x < GRID_SIZE && mousePos.y >= 0 && mousePos.y < GRID_SIZE) {
        switchCell(mousePos.x, mousePos.y);
    }
}

function switchCell(x, y) {
    fetch('/switch', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify([x, y]),
    });
}

function fetchGrid() {
    fetch('/grid')
        .then(response => response.json())
        .then(data => {
            grid = data.cells;
            drawGrid();
        });
}

const socket = new WebSocket(`ws://${window.location.host}/ws`);
socket.onmessage = function(event) {
    const data = JSON.parse(event.data);
    grid = data.cells;
    drawGrid();
};

window.addEventListener('resize', resizeCanvas);
resizeCanvas();
fetchGrid();
