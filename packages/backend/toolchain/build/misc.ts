import dedent from "dedent";

export function autoGenHeader(commentChar = "//") {
	return dedent`
		${commentChar} This file is auto-generated by the Rivet (https://rivet.gg) build system.
		${commentChar} 
		${commentChar} Do not edit this file directly.

	`;
}
