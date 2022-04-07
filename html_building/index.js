//import("world_generators");
import * as wasm from "ld50_lib";
import("./html5game/ld50.js");

// HACKY AF
//window["reset"] = wasm.reset;
//window["add_world"] = wasm.add_world;
//window["sample"] = wasm.sample;
//window["slam"] = wasm.slam;
//window["tick"] = wasm.tick;
//
//window["get_amp"] = wasm.get_amp;
//window["add_weight"] = wasm.add_weight;
//
// used in water
//window["sample_osc"] = wasm.sample_osc;
//window["osc_count"] = wasm.osc_count;

// EVEN HACKIER
// Dont have access to non-obfuscated output :(
	/*
window["_35"] = wasm.reset;
window["_A4"] = wasm.add_world;
window["_e4"] = wasm.sample; // () => console.log("_e4");
window["_o5"] = wasm.tick; // () => console.log("_o5");
window["_l6"] = wasm.slam;

window["_V7"] = wasm.reset;
window["_r7"] = wasm.add_world;
window["_57"] = wasm.sample; // () => console.log("_e4");
window["_e8"] = wasm.tick; // () => console.log("_o5");
window["_b9"] = wasm.slam;

window["_D6"] = wasm.reset;
window["_96"] = wasm.add_world;
window["_O5"] = wasm.sample;
window["_X6"] = wasm.tick;
window["_U7"] = wasm.slam;
*/

/*
window["_B6"] = wasm.add_world;
window["_f6"] = wasm.sample;
window["_o7"] = wasm.tick;
window["_m8"] = wasm.slam;
*/


/*
window["_p6"] = wasm.reset;
window["_c7"] = wasm.add_node;
window["_r7"] = wasm.add_rope_length;
window["_57"] = wasm.add_box;
window["_C6"] = wasm.tick;
window["_D6"] = wasm.get_tension;
window["_u8"] = wasm.get_node_x;
window["_B7"] = wasm.get_node_y;
window["_D7"] = () => console.log("_D7");
*/

/*
window["_u7"] = () => console.log("_u7");
window["_h8"] = () => console.log("h8");
window["_w8"] = () => console.log("w8");
window["_a8"] = () => console.log("a8");
window["_H7"] = () => console.log("H7");
window["_I7"] = () => console.log("I7");
window["_z9"] = () => console.log("_z9");
window["_I8"] = () => {console.log("_I8"); return 0};
window["_I9"] = () => {console.log("_I9"); return 0};
window["_T9"] = () => {console.log("_T9"); return 0};
window["_S8"] = () => {console.log("_S8"); return 0};
window["_G8"] = () => {console.log("_G8"); return 0};
*/

window["_u7"] = wasm.reset;
window["_h8"] = wasm.add_node;
window["_w8"] = wasm.add_rope_length;
window["_a8"] = wasm.add_box;
window["_H7"] = wasm.tick;
window["_I7"] = wasm.get_tension;
window["_z9"] = wasm.get_rope_broken;
window["_G8"] = wasm.get_node_x;
window["_I8"] = wasm.get_node_y;
window["_T9"] = wasm.set_free;
//window["_I9"] = () => {console.log("_I9"); return 0};
window["_S8"] = wasm.set_node_pos_player;

/*
window["_P9"] = () => {console.log("_P9"); return 0};
window["_R9"] = () => {console.log("_R9"); return 0};
window["_S9"] = () => {console.log("_S9"); return 0};
*/

window["_P9"] = wasm.set_rope_broken;
window["_R9"] = wasm.set_node_pos;
window["_S9"] = wasm.set_fixed;
//window["_js2"] = () => {console.log("_js2"); return 0};

// set_fixed
// set_node_pos
// set_rope_broken

/*
window["_u7"] = wasm.reset;
window["_h8"] = wasm.add_node;
window["_w8"] = wasm.add_rope_length;
window["_H7"] = wasm.add_box;
window["_I7"] = wasm.tick;
window["_z9"] = wasm.get_tension;
window["_a8"] = wasm.get_rope_broken;
window["_G8"] = wasm.get_node_x;
window["_I8"] = wasm.get_node_x;
////window["_a8"] = () => { console.log("a8"); return 0};
////window["_G8"] = () => { console.log("G8"); return 0};
////window["_a8"] = wasm.get_node_x;
////window["_G8"] = wasm.get_node_y;
//window["_I8"] = () => {console.log("_I8"); return 0};
window["_I9"] = () => {console.log("_I9"); return 0};
window["_T9"] = () => {console.log("_T9"); return 0};
window["_S8"] = () => {console.log("_S8"); return 0};
*/


// tick
// get_tension
// add_node
// add_rope_length

//game.GameMaker_Init();
//window["GameMaker_Init"]();
//GameMaker_Init();
//window["GameMaker_Init"]();



const canvas = document.getElementById("canvas");

const canvasStyle = 
"image-rendering: -moz-crisp-edges;" +
"image-rendering: pixelated;" +
"image-rendering: -webkit-crisp-edges;" +
"image-rendering: crisp-edges;";

// Landscape
canvas.style = canvasStyle + 
	//"bottom: 0px;" +
	//"left: 0px;" +
	"margin: auto;" +
	"width: 100%;";