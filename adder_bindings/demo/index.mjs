import { RenderInfo } from "./demo_gen/index.mjs";

Object.values(RenderInfo.termini).toSorted((a, b) => a.funcName < b.funcName ? -1 : 1).forEach((t) => {
	let a = document.createElement("li");
	a.innerHTML = `<a href="demo_gen/rendering/template.html?func=${t.funcName}">${t.funcName}</a>`;
	document.getElementById("links").appendChild(a);
});
