class WebChart{};

const canvas = document.getElementById("canvas");
const coord = document.getElementById("coord");
const status = document.getElementById("status");

let chart = null;

/** Main entry point */
export function main() {
    status.innerText = "Status: WebAssembly loaded!";
    setupCanvas();
    setupUI();
 }

 /** This function is used in `bootstrap.js` to setup imports. */
export function setup(WasmChart) {
    WebChart = WasmChart;
}

function setupUI() {
    window.addEventListener("resize", setupCanvas);
    window.addEventListener("mousemove", onMouseMove);
}

function setupCanvas() {
    const dpr = window.devicePixelRatio || 1.0;
    const aspectRatio = canvas.width / canvas.height;
    const size = canvas.parentNode.offsetWidth * 0.6;
    canvas.style.width = size + "px";
    canvas.style.height = size / aspectRatio + "px";
    canvas.width = size;
    canvas.height = size / aspectRatio;

    updatePlot();
}

function onMouseMove(event) {
    if (chart) {
		var text = "Mouse pointer is out of range";

		if(event.target == canvas) {
			let actualRect = canvas.getBoundingClientRect();
			let logicX = event.offsetX * canvas.width / actualRect.width;
			let logicY = event.offsetY * canvas.height / actualRect.height;
			const point = chart.coord(logicX, logicY);
			text = (point) 
				? `(${point.x.toFixed(3)}, ${point.y.toFixed(3)})`
				: text;
		}
        coord.innerText = text;
    }
}

function updatePlot() {
    status.innerText = "Simulating and Rendering Haber-Bosch";

    chart = null;
    const start = performance.now();
    var pres = 100;
    var cat = "FN"
    chart = WebChart.draw_concentration_balances("canvas", cat, pres);
	const end = performance.now();

    status.innerText = `Status: Simulation and Rendering done in ${Math.ceil(end - start)}ms`;
}