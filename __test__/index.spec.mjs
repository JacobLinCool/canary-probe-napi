import test from "ava";

import { available, check } from "../index.js";

test("check binding", async (t) => {
	if (!(await available())) {
		console.warn("docker is not available");
	} else {
		console.log("docker is available");
	}
	t.pass();
});

test.skip("check zip", async (t) => {
	const { executables } = await check("./__test__/hw.zip");
	t.deepEqual(executables, ["hw0101", "hw0102", "hw0103", "hw0104"]);
});
