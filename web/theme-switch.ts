// This software is in the public domain.

import { Theme } from "@spectrum-web-components/theme";

const pageTheme = document.getElementById("page-theme") as Theme;
pageTheme.scale = "large";
pageTheme.theme = "express";

const preferDark = matchMedia("(prefers-color-scheme: dark)");
preferDark.addEventListener("change", () => {
	pageTheme.color = preferDark.matches ? "dark" : "light";
});
preferDark.dispatchEvent(new Event("change"));
