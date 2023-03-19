import "./style.css";
import "@spectrum-web-components/theme/sp-theme.js";
import "@spectrum-web-components/theme/express/theme-dark.js";
import "@spectrum-web-components/theme/express/theme-light.js";
import "@spectrum-web-components/theme/express/scale-large.js";
import "@spectrum-web-components/icons-workflow/icons/sp-icon-image.js";
import "@spectrum-web-components/illustrated-message/sp-illustrated-message.js";
import "@spectrum-web-components/link/sp-link.js";
import "@spectrum-web-components/dropzone/sp-dropzone.js";
import "@spectrum-web-components/toast/sp-toast.js";
import "@spectrum-web-components/switch/sp-switch.js";
import "@spectrum-web-components/dialog/sp-dialog.js";
import { Theme } from "@spectrum-web-components/theme";

const pageTheme = document.getElementById("page-theme") as Theme;
pageTheme.scale = "large";
pageTheme.theme = "express";

const preferDark = matchMedia("(prefers-color-scheme: dark)");
preferDark.addEventListener("change", function () {
  pageTheme.color = this.matches ? "dark" : "light";
});
preferDark.dispatchEvent(new Event("change"));
