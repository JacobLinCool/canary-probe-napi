import test from "ava";

import { check } from "../index.js";

test("check zip", async (t) => {
	const executables = await check("./__test__/hw.zip");
	t.deepEqual(executables, ["./hw0101", "./hw0102", "./hw0103", "./hw0104"]);
});
