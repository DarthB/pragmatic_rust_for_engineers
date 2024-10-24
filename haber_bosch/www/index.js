class WebChart {}
class WebInput {}
class WebModelRange {}
class HaberBoschBedSetup {}

const canvas = document.getElementById("canvas");
const csde = document.getElementById("canvas_sized_disp_el");
const coord = document.getElementById("coord");
const status = document.getElementById("status");

const plot_type = document.getElementById("plot_type");
const auto_ranges = document.getElementById("auto_ranges");
const diff_tool = document.getElementById("diff_tool")

const catalyst_lhs = document.getElementById("catalyst_lhs");
const pressure_lhs = document.getElementById("pressure_lhs");
const pressure_lhs_ro = document.getElementById("pressure_lhs_ro");
const num_beds_lhs = document.getElementById("num_beds_lhs");
const num_beds_lhs_ro = document.getElementById("num_beds_lhs_ro");

const catalyst_rhs = document.getElementById("catalyst_rhs");
const pressure_rhs = document.getElementById("pressure_rhs");
const pressure_rhs_ro = document.getElementById("pressure_rhs_ro");
const num_beds_rhs = document.getElementById("num_beds_rhs");
const num_beds_rhs_ro = document.getElementById("num_beds_rhs_ro");

const x_max = document.getElementById("x_max");
const c_max = document.getElementById("c_max");
const min_temp = document.getElementById("t_min");
const max_temp = document.getElementById("t_max");

let chart = null;

/** Main entry point */
export function main() {
    setupUI();
    setupCanvas();
}

/** This function is used in `bootstrap.js` to setup imports. */
export function setup(WasmChart, WasmWebInput, WasmWebInputConfig, WasmBedSetup) {
    WebChart = WasmChart;
	WebInput = WasmWebInput;
	WebModelRange = WasmWebInputConfig;
	HaberBoschBedSetup = WasmBedSetup;
}

/** Add event listeners. */
function setupUI() {
    status.innerText = "Status: WebAssembly loaded!";
    
	window.addEventListener("resize", setupCanvas);
    window.addEventListener("mousemove", onMouseMove);

	plot_type.addEventListener("change", updateHBPlot);
	auto_ranges.addEventListener("change", function() {
		toggle_visibility(auto_ranges);
		updateHBPlot();
	})
	diff_tool.addEventListener("change", function() {
		toggle_visibility(diff_tool);
		updateHBPlot();
	})

	let postfix = ["lhs", "rhs"];	

	for(let k=0; k<postfix.length; ++k) {
		let pf = "_"+postfix[k];
		let cid = document.getElementById("catalyst"+pf);
		cid.addEventListener("change", function() {
			setup_ranges(cid.value, k>0);
		});

		let pid = "pressure"+pf;
		let pel = document.getElementById(pid);
		let pelro = document.getElementById(pid+"_ro");
		pel.addEventListener("change", function() {update(pel, pelro, true);});
		pel.addEventListener("input", function() {update(pel, pelro, false);});

		let nid = "num_beds"+pf;
		let elb = document.getElementById(nid);
		let elbro = document.getElementById(nid+"_ro");
		elb.addEventListener("change", function() {
			show_beds(elb.value, k>0);
			update(elb, elbro, false);
		})
	}

	catalyst_lhs.addEventListener("change", function() {
		setup_ranges(catalyst_lhs.value, false);
		updateHBPlot();
	});

	catalyst_rhs.addEventListener("change", function() {
		setup_ranges(catalyst_rhs.value, true);
		updateHBPlot();
	})



	setup_bed_events(1, false);
	setup_bed_events(2, false);
	setup_bed_events(3, false);

	setup_bed_events(1, true);
	setup_bed_events(2, true);
	setup_bed_events(3, true);

	setup_ranges("KMIR");
	setup_ranges("FN", true);
}

function show_beds(num, alt) {
	let pf;
	if (alt) {
		pf = "_rhs";
	} else {
		pf = "_lhs";
	}

	for(let i=0; i<3; ++i) {
		let id = "bed" + (i+1) + pf;
		let el = document.getElementById(id);
		if(i>=num) {
			el.classList.add("hide");
		} else {
			el.classList.remove("hide");
		}
	} 
}

function setup_axis_range(config) {
	let ids = ["x_max", "c_max", "t_max", "t_min"];
	let factors = [0.1, 0.01, 1, 1];
	for(var i=0; i<ids.length; ++i) {
		let id = ids[i];
		let factor = factors[i];
		let el = document.getElementById(id);
		let el_ro = document.getElementById(id+"_ro");
		el.addEventListener("input", function() {update(el, el_ro, false, factor);});
		el.addEventListener("change", function() {update(el, el_ro, true, factor);});
	}
}

function update_all_ro() {

	let post = ["_lhs"];
	let controls = ["pressure", "num_beds"]
	for(let i=0; i<3; ++i) {
		controls.push("bed0" + (i+1) + "_start_temp")
		controls.push("bed0" + (i+1) + "_slope_temp")
	}

	for(let sides=0; sides<post.length; ++sides) {
		for(let ctrl=0; ctrl<controls.length; ++ctrl) {
			let id = controls[ctrl] + post;
			let el = document.getElementById(id);
			let ro_el = document.getElementById(id + "_ro");
			update(el, ro_el);
		}
	}
}

function setup_ranges(cat, alt) {
	let config = WebModelRange.from_catalyst(cat);

	let post = null;
	if(alt) {
		post = "_rhs";
	} else {
		post = "_lhs";
	}
	
	let pid = "pressure" + post;
	let pel = document.getElementById(pid); 
	pel.min = config.pressure_range.min_val;
	pel.max = config.pressure_range.max_val;
	pel.value = config.pressure_range.def_val;

	let nid = "num_beds" + post;
	let nel = document.getElementById(nid);
	nel.min = config.num_beds_range.min_val;
	nel.max = config.num_beds_range.max_val;
	nel.value = config.num_beds_range.def_val;

	for (let i=0; i<config.bed_start_temp_ranges.length; i++)
	{
		let el = config.bed_start_temp_ranges[i];

		let tsid = "bed0" + (i+1) + "_start_temp" + post;
		let tsel = document.getElementById(tsid);
		
		tsel.min = el.min_val;
		tsel.max = el.max_val;
		tsel.value = el.def_val;
		tsel.step = el.step;
	}

	setup_axis_range();
	update_all_ro();
}

function toggle_visibility(sender) {
	let id = sender.id;
	let el = null;

	switch (id) {
		case 'diff_tool':
			el = document.getElementById("alternative_scenario_control");
			break;
		case 'auto_ranges':
			el = document.getElementById("range_control");
			break;
		default:
			// this should never happen
			alert('toggle for ' + id + ' should never happen');
			return;
	}
	
	el.classList.toggle("hide");
}

function update(sender, ro, redraw_flag, factor) {
	if(factor == null) factor = 1;

	if(ro != null) {
		ro.innerText = sender.value * factor;
	}
	if (!redraw_flag) {
		updateHBPlot()
	}
}

/** Setup canvas to properly handle high DPI and redraw current plot. */
function setupCanvas() {
	const dpr = window.devicePixelRatio || 1.0;
	let parentWidth = window.innerWidth;
    let parentHeight = window.innerHeight;
    let aspectRatio = parentWidth / parentHeight;
    //const size = parentWidth * 1.0;
	let size = window.innerWidth - 500;
	//size = 2000;
    //canvas.style.width = size + "px";
    //canvas.style.height = size + "px";
    canvas.width = size * dpr;
    canvas.height = (size / aspectRatio) * dpr;

	csde.style.width = (size*dpr) + "px";

    updateHBPlot();
}

/** Update displayed coordinates. */
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

function bed_info(idx, alt) {
	let prefix = "bed0" + idx +"_"
	let postfix = null;
	if(alt) {
		postfix = "_rhs";
	} else {
		postfix = "_lhs";
	}

	return [prefix, postfix];
}

function setup_bed_events(idx, alt) {
	let [pre, post] = bed_info(idx,alt);
	let id = pre + "start_temp" + post;

	let el_range = document.getElementById(id);
	let el_ro = document.getElementById(id + "_ro");
	el_range.addEventListener("input", function() {update(el_range, el_ro, false);});
	el_range.addEventListener("change", function() {update(el_range, el_ro, true);});

	id = pre + "slope_temp" + post;
	let el_range2 = document.getElementById(id);
	let el_ro2 = document.getElementById(id + "_ro");
	if (el_range2 != null && el_ro2 != null) {
		el_range2.addEventListener("input", function() {update(el_range2, el_ro2, false);});
		el_range2.addEventListener("change", function() {update(el_range2, el_ro2, true);});
	}
}

function read_bed(idx, alt) {
	let [pre, post] = bed_info(idx, alt);

	let cat_name = null;
	if (alt) {
		cat_name = "FN";
	} else {
		cat_name = "KMIR";
	}

	let bed = HaberBoschBedSetup.from_constants(idx-1, cat_name);
    bed.t_start = Number(document.getElementById(pre + "start_temp" + post).value) + 273;
    //bed.t_slope = Number(document.getElementById(pre + "slope_temp" + post).value);
	if(alt) {
		bed.t_slope = 20;
	} else {
		bed.t_slope = 20;
	}
	return bed;
}

function read_input() {
	let input = WebInput.new();

	let selected = catalyst_lhs.selectedOptions[0];
	let wip_main = input.main;
	wip_main.set_catalyst(selected.value);
	
	wip_main.pressure = Number(pressure_lhs.value);
	wip_main.num_beds = Number(num_beds_lhs.value);

	let tmp_beds = wip_main.beds;
	for(let i=0; i<wip_main.num_beds; ++i) {
		tmp_beds.push(read_bed(i+1, false));
	}
	wip_main.beds = tmp_beds;
	input.main = wip_main;

	if (diff_tool.checked) {
		let wip_alt = input.alt;
		let alt_selected = catalyst_rhs.selectedOptions[0];
		wip_alt.set_catalyst(alt_selected.value);

		wip_alt.pressure = Number(pressure_rhs.value);
		wip_alt.num_beds = Number(num_beds_rhs.value);

		let alt_tmp_beds = wip_alt.beds;
		for(let k=0; k<wip_alt.num_beds; ++k) {
			alt_tmp_beds.push(read_bed(k+1, true));
		}
		wip_alt.beds = alt_tmp_beds;
		input.alt = wip_alt;
	} else {
		input.alt = null;
	}

	if(!auto_ranges.checked) {
		let wip = input.axis_settings;
		wip.length_max = Number(x_max.value * 0.1);
		wip.concentration_max = Number(c_max.value * 0.01);
		wip.min_temp = Number(min_temp.value);
		wip.max_temp = Number(max_temp.value);
		input.axis_settings = wip;
	} else {
		input.axis_settings = null;
	}
	return input;
}

function updateHBPlot() {
    status.innerText = `Simulating then Rendering Haber-Bosch Scenario`;
	chart = null;
	let input = read_input();
	const start = performance.now();

	let selected = plot_type.selectedOptions[0];
	switch(selected.value) {
		case "cbt":
			chart = WebChart.draw_concentration_balances("canvas", input);		
			break;
		case "toy":
			chart = WebChart.draw_temperature_over_yield("canvas", input);		
			break;
		default:
			status.innerText = `Status: Simulation and Rendering done in ${Math.ceil(end - start)}ms`;
			return;
	}

	const end = performance.now();
    status.innerText = `Status: Simulation and Rendering done in ${Math.ceil(end - start)}ms`;
}